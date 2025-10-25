#!/usr/bin/env python3
"""
Extract all generated harnesses from fuzzomatic results and set them up in the fuzz project.

Usage:
    python3 extract_harnesses.py <crate_directory>
    
Example:
    python3 extract_harnesses.py crate_batch_1
"""

import argparse
import json
import os
import sys
import re


def read_fuzzomatic_results(crate_dir):
    """Read the .fuzzomatic_results.json file"""
    results_path = os.path.join(crate_dir, ".fuzzomatic_results.json")
    
    if not os.path.exists(results_path):
        print(f"Error: No .fuzzomatic_results.json found in {crate_dir}")
        print(f"Run fuzzomatic first: cd fuzzomatic && poetry run fz ../{crate_dir}")
        sys.exit(1)
    
    with open(results_path, 'r') as f:
        return json.load(f)


def sanitize_target_name(name):
    """Convert function name to valid fuzz target name"""
    # Remove common prefixes/suffixes
    name = re.sub(r'^run_', '', name)
    name = re.sub(r'_test$', '', name)
    
    # Replace invalid characters with underscore
    name = re.sub(r'[^a-zA-Z0-9_]', '_', name)
    
    # Ensure it doesn't start with a number
    if name and name[0].isdigit():
        name = 'fuzz_' + name
    
    return name or 'unnamed'


def extract_function_name_from_code(code):
    """Try to extract the function name being fuzzed from the harness code"""
    # Look for patterns like: run_1(, run_11(, function_name(
    matches = re.findall(r'([a-zA-Z_][a-zA-Z0-9_]*)\s*\([^)]*data', code)
    if matches:
        return matches[0]
    
    # Look for use statements like: use crate_batch_1::run_1;
    matches = re.findall(r'use\s+[^:]+::([a-zA-Z_][a-zA-Z0-9_]*);', code)
    if matches:
        return matches[0]
    
    return None


def write_harness_files(crate_dir, harnesses):
    """Write all harness files to fuzz/fuzz_targets/"""
    fuzz_targets_dir = os.path.join(crate_dir, "fuzz", "fuzz_targets")
    
    if not os.path.exists(fuzz_targets_dir):
        print(f"Error: Fuzz targets directory not found: {fuzz_targets_dir}")
        print(f"Run 'cargo fuzz init' in {crate_dir} first")
        sys.exit(1)
    
    target_names = []
    
    for i, harness in enumerate(harnesses):
        code = harness['fuzz_target_code']
        approach = harness['successful_approach']
        is_useful = harness['is_useful']
        bug_found = harness['bug_found']
        
        # Try to extract function name from code
        func_name = extract_function_name_from_code(code)
        
        if func_name:
            target_name = sanitize_target_name(func_name)
        else:
            target_name = f"target_{i+1}"
        
        # Ensure unique names
        base_name = target_name
        counter = 1
        while target_name in target_names:
            target_name = f"{base_name}_{counter}"
            counter += 1
        
        target_names.append(target_name)
        
        # Write the harness file
        target_path = os.path.join(fuzz_targets_dir, f"{target_name}.rs")
        with open(target_path, 'w') as f:
            f.write(code)
        
        status = []
        if is_useful:
            status.append("useful")
        if bug_found:
            status.append("BUG FOUND!")
        
        status_str = f" [{', '.join(status)}]" if status else ""
        
        print(f"  [{i+1:2d}] {target_name:20s} (approach: {approach:15s}){status_str}")
        print(f"       -> {target_path}")
    
    return target_names


def update_cargo_toml(crate_dir, target_names):
    """Update fuzz/Cargo.toml to include all harness targets"""
    cargo_toml_path = os.path.join(crate_dir, "fuzz", "Cargo.toml")
    
    if not os.path.exists(cargo_toml_path):
        print(f"Error: Cargo.toml not found: {cargo_toml_path}")
        sys.exit(1)
    
    with open(cargo_toml_path, 'r') as f:
        content = f.read()
    
    # Remove existing [[bin]] sections
    content = re.sub(r'\n\[\[bin\]\].*?(?=\n\[|\Z)', '', content, flags=re.DOTALL)
    
    # Add new [[bin]] sections for each target
    bin_sections = []
    for target_name in target_names:
        bin_section = f"""
[[bin]]
name = "{target_name}"
path = "fuzz_targets/{target_name}.rs"
test = false
doc = false
"""
        bin_sections.append(bin_section)
    
    # Append all bin sections at the end
    content = content.rstrip() + '\n' + '\n'.join(bin_sections) + '\n'
    
    with open(cargo_toml_path, 'w') as f:
        f.write(content)
    
    print(f"\nUpdated {cargo_toml_path} with {len(target_names)} targets")


def main():
    parser = argparse.ArgumentParser(
        description="Extract fuzzomatic harnesses and set them up in fuzz project"
    )
    parser.add_argument(
        'crate_dir',
        help="Path to the crate directory (e.g., crate_batch_1)"
    )
    parser.add_argument(
        '--backup',
        action='store_true',
        help="Backup existing fuzz/Cargo.toml before modifying"
    )
    
    args = parser.parse_args()
    
    # Make path absolute
    crate_dir = os.path.abspath(args.crate_dir)
    
    if not os.path.exists(crate_dir):
        print(f"Error: Directory not found: {crate_dir}")
        sys.exit(1)
    
    print(f"Extracting harnesses from: {crate_dir}")
    print("=" * 70)
    
    # Read fuzzomatic results
    results = read_fuzzomatic_results(crate_dir)
    harnesses = results.get('generated_fuzz_targets', [])
    
    if not harnesses:
        print("No harnesses found in .fuzzomatic_results.json")
        sys.exit(0)
    
    print(f"\nFound {len(harnesses)} harnesses:")
    print("-" * 70)
    
    # Backup Cargo.toml if requested
    if args.backup:
        cargo_toml_path = os.path.join(crate_dir, "fuzz", "Cargo.toml")
        if os.path.exists(cargo_toml_path):
            backup_path = cargo_toml_path + ".backup"
            import shutil
            shutil.copy2(cargo_toml_path, backup_path)
            print(f"Backed up Cargo.toml to: {backup_path}\n")
    
    # Write harness files
    target_names = write_harness_files(crate_dir, harnesses)
    
    # Update Cargo.toml
    update_cargo_toml(crate_dir, target_names)
    
    print("\n" + "=" * 70)
    print("✓ All harnesses extracted successfully!")
    print("\nTo build a specific harness:")
    print(f"  cd {crate_dir} && cargo fuzz build {target_names[0]}")
    print("\nTo run a specific harness:")
    print(f"  cd {crate_dir} && cargo fuzz run {target_names[0]}")
    print("\nTo list all targets:")
    print(f"  cd {crate_dir} && cargo fuzz list")


if __name__ == '__main__':
    main()
