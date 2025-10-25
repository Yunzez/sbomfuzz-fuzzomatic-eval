#!/usr/bin/env python3
"""
Automate refactoring of crate_batch_2 through crate_batch_6
to extract hardcoded inputs into function parameters.
"""

import os
import re
import shutil

# Note: This is a template/framework script. Actual refactoring would
# require parsing Rust AST or manual edits per-function since the 
# hardcoded data varies significantly between functions.

def backup_file(filepath):
    """Create a backup of the file"""
    if os.path.exists(filepath):
        backup_path = filepath + ".backup"
        shutil.copy2(filepath, backup_path)
        print(f"Backed up {filepath} to {backup_path}")
        return True
    return False

def process_crate_batch(batch_num):
    """Process a single crate batch"""
    batch_dir = f"/home/yzzhao3/sbomfuzz-fuzzomatic-eval/crate_batch_{batch_num}"
    src_dir = os.path.join(batch_dir, "src")
    
    # Check if main.rs exists
    main_rs = os.path.join(src_dir, "main.rs")
    lib_rs = os.path.join(src_dir, "lib.rs")
    
    source_file = lib_rs if os.path.exists(lib_rs) else main_rs
    
    if not os.path.exists(source_file):
        print(f"No source file found for batch {batch_num}")
        return False
    
    # Backup
    backup_file(source_file)
    
    # Rename main.rs to lib.rs if needed
    if source_file == main_rs and not os.path.exists(lib_rs):
        os.rename(main_rs, lib_rs)
        print(f"Renamed main.rs to lib.rs in batch {batch_num}")
        source_file = lib_rs
    
    print(f"Processing {source_file}...")
    print(f"Note: Actual refactoring requires manual edits per function")
    print(f"      due to varying hardcoded data patterns")
    
    return True

def main():
    for batch_num in range(2, 7):  # batches 2-6
        print(f"\n=== Processing crate_batch_{batch_num} ===")
        process_crate_batch(batch_num)

if __name__ == "__main__":
    main()
