#!/usr/bin/env python3
"""
Run all fuzz targets in a tmux session with each target in its own window.

Each window runs `cargo fuzz` with libFuzzer configured to continue after
crashes, timeouts, and OOMs for the requested duration (defaults to 24 hours).
Optional multi-process fuzzing is enabled via libFuzzer's `-fork` flag.

Usage:
    python3 run_all_fuzz.py <crate_directory> [--session-name NAME] [--time SECONDS]
    
Example:
    python3 run_all_fuzz.py crate_batch_1
    python3 run_all_fuzz.py crate_batch_1 --session-name fuzz_session --time 60
"""

import argparse
import os
import subprocess
import sys
import time


def get_fuzz_targets(crate_dir):
    """Get list of all fuzz targets using cargo fuzz list"""
    try:
        result = subprocess.run(
            ['cargo', 'fuzz', 'list'],
            cwd=crate_dir,
            capture_output=True,
            text=True,
            check=True
        )
        targets = [line.strip() for line in result.stdout.strip().split('\n') if line.strip()]
        return targets
    except subprocess.CalledProcessError as e:
        print(f"Error: Failed to list fuzz targets")
        print(f"Make sure you're in a directory with cargo fuzz initialized")
        print(f"Error: {e.stderr}")
        sys.exit(1)


def check_tmux_installed():
    """Check if tmux is installed"""
    try:
        subprocess.run(['tmux', '-V'], capture_output=True, check=True)
        return True
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("Error: tmux is not installed")
        print("Install it with: sudo apt-get install tmux  (or your package manager)")
        sys.exit(1)


def session_exists(session_name):
    """Check if a tmux session already exists"""
    try:
        result = subprocess.run(
            ['tmux', 'has-session', '-t', session_name],
            capture_output=True,
            stderr=subprocess.DEVNULL
        )
        return result.returncode == 0
    except:
        return False


def create_tmux_session(session_name, crate_dir, targets, max_time, fork_jobs):
    """Create a tmux session with each fuzz target in its own window"""
    
    if not targets:
        print("No fuzz targets found!")
        sys.exit(1)
    
    print(f"Creating tmux session '{session_name}' with {len(targets)} fuzz targets...")
    print(f"Crate directory: {crate_dir}")
    print(f"Max time per target: {max_time} seconds ({max_time/3600:.1f} hours)")
    base_flag_msg = "LibFuzzer flags: -ignore_crashes=1 -ignore_timeouts=1 -ignore_ooms=1"
    if fork_jobs and fork_jobs > 0:
        base_flag_msg += f" -fork={fork_jobs}"
    print(base_flag_msg)
    print("-" * 70)
    
    # Kill existing session if it exists
    if session_exists(session_name):
        print(f"Session '{session_name}' already exists. Killing it...")
        subprocess.run(['tmux', 'kill-session', '-t', session_name], 
                      capture_output=True)
        time.sleep(0.5)
    
    # Create new session with first target in first window
    first_target = targets[0]
    libfuzzer_flags = [
        f"-max_total_time={max_time}",
        "-ignore_crashes=1",
        "-ignore_timeouts=1",
        "-ignore_ooms=1",
    ]
    if fork_jobs and fork_jobs > 0:
        libfuzzer_flags.append(f"-fork={fork_jobs}")
    joined_flags = " ".join(libfuzzer_flags)

    cargo_cmd = os.environ.get("FUZZ_CARGO_COMMAND", "cargo +nightly fuzz run")
    cmd = f"cd {crate_dir} && {cargo_cmd} {first_target} -- {joined_flags}"
    
    print(f"[1/{len(targets)}] Window 0: {first_target}")
    
    try:
        subprocess.run([
            'tmux', 'new-session', '-d', '-s', session_name,
            '-n', first_target,  # Window name is the target name
            'bash', '-lc', cmd
        ], check=True)
    except subprocess.CalledProcessError as exc:
        print(f"Failed to create tmux session for target '{first_target}': {exc}")
        print("Command:", cmd)
        sys.exit(1)
    
    # Add remaining targets in new windows
    for i, target in enumerate(targets[1:], start=2):
        print(f"[{i}/{len(targets)}] Window {i-1}: {target}")
        cmd = f"cd {crate_dir} && {cargo_cmd} {target} -- {joined_flags}"

        # Create a new window for each target
        try:
            subprocess.run([
                'tmux', 'new-window', '-t', session_name,
                '-n', target,  # Window name
                'bash', '-lc', cmd
            ], check=True)
        except subprocess.CalledProcessError as exc:
            print(f"Failed to create tmux window for target '{target}': {exc}")
            print("Command:", cmd)
            print("Cleaning up session...")
            subprocess.run(['tmux', 'kill-session', '-t', session_name], capture_output=True)
            sys.exit(1)
    
    # Select the first window
    subprocess.run([
        'tmux', 'select-window', '-t', f'{session_name}:0'
    ])
    
    print("-" * 70)
    print(f"✓ Tmux session '{session_name}' created successfully!")
    print()
    print("To attach to the session:")
    print(f"  tmux attach -t {session_name}")
    print()
    print("Inside tmux:")
    print("  - Switch windows: Ctrl+b then 0-9 (for windows 0-9)")
    print("  - Next window: Ctrl+b then n")
    print("  - Previous window: Ctrl+b then p")
    print("  - List windows: Ctrl+b then w (then use arrow keys)")
    print("  - Detach: Ctrl+b then d")
    print("  - Kill session: Ctrl+b then type ':kill-session' and Enter")
    print()
    print("To kill the session from outside:")
    print(f"  tmux kill-session -t {session_name}")
    print()
    
    return session_name


def main():
    parser = argparse.ArgumentParser(
        description="Run all fuzz targets in a tmux session",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Run all targets in crate_batch_1
  python3 run_all_fuzz.py crate_batch_1
  
  # Custom session name and 60 second timeout
  python3 run_all_fuzz.py crate_batch_1 --session-name my_fuzz --time 60
  
    # Run with four forked libFuzzer workers per target
    python3 run_all_fuzz.py crate_batch_1 --forks 4
  
  # Attach to running session
  tmux attach -t fuzz_session
        """
    )
    parser.add_argument(
        'crate_dir',
        help="Path to the crate directory (e.g., crate_batch_1)"
    )
    parser.add_argument(
        '--session-name', '-s',
        default='fuzz_session',
        help="Name for the tmux session (default: fuzz_session)"
    )
    parser.add_argument(
        '--time', '-t',
        type=int,
        default=86400,
        help="Max time in seconds for each fuzzer (default: 86400 = 24 hours)"
    )
    parser.add_argument(
        '--forks',
        type=int,
        default=0,
        help="Number of libFuzzer forked workers per target (0 disables -fork)"
    )
    parser.add_argument(
        '--attach', '-a',
        action='store_true',
        help="Attach to the session after creating it"
    )
    
    args = parser.parse_args()
    
    # Check tmux is installed
    check_tmux_installed()
    
    # Make path absolute
    crate_dir = os.path.abspath(args.crate_dir)
    
    if not os.path.exists(crate_dir):
        print(f"Error: Directory not found: {crate_dir}")
        sys.exit(1)
    
    # Check if fuzz directory exists
    fuzz_dir = os.path.join(crate_dir, 'fuzz')
    if not os.path.exists(fuzz_dir):
        print(f"Error: No fuzz directory found in {crate_dir}")
        print(f"Run 'cargo fuzz init' first or use extract_harnesses.py")
        sys.exit(1)
    
    # Get all fuzz targets
    print("Discovering fuzz targets...")
    targets = get_fuzz_targets(crate_dir)
    
    if not targets:
        print("No fuzz targets found!")
        sys.exit(1)
    
    print(f"Found {len(targets)} targets: {', '.join(targets)}")
    print()
    
    # Create tmux session
    session_name = create_tmux_session(
        args.session_name,
        crate_dir,
        targets,
        args.time,
        args.forks
    )
    
    # Attach if requested
    if args.attach:
        print(f"Attaching to session '{session_name}'...")
        print("(Press Ctrl+b then d to detach)")
        time.sleep(1)
        subprocess.run(['tmux', 'attach', '-t', session_name])


if __name__ == '__main__':
    main()
