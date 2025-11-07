# Fuzzomatic Setup Instructions

This document provides step-by-step instructions to set up the Fuzzomatic project from scratch.

## Prerequisites

Ensure the following tools are installed on your system:

1. **oma** (AOSC OS package manager)
2. **Python 3.11+** (can be installed via `pyenv` if not available in `oma`)
3. **Poetry** (Python dependency manager)
4. **Rust** (install via [rustup](https://rustup.rs/))
5. **cargo-fuzz** (install via `cargo install cargo-fuzz`)
6. **semgrep** (install via `pipx install semgrep`)
7. **OpenAI API Key** (see [OpenAI setup](https://platform.openai.com/docs/quickstart/account-setup?context=python))

## Steps to Set Up Fuzzomatic

1. **Clone the Repository**
   ```bash
   git clone https://github.com/kudelskisecurity/fuzzomatic fuzzomatic
   cd fuzzomatic
   ```

2. **Install Python 3.11+**
   If Python 3.11+ is not available via `oma`, install it using `pyenv`:

   ```bash
   # Install pyenv
   curl https://pyenv.run | bash

   # Add pyenv to your shell configuration
   echo -e '\nexport PYENV_ROOT="$HOME/.pyenv"\n[[ -d $PYENV_ROOT/bin ]] && export PATH="$PYENV_ROOT/bin:$PATH"\neval "$(pyenv init - bash)"\neval "$(pyenv virtualenv-init -)"' >> ~/.bashrc
   source ~/.bashrc

   # Install Python 3.11
   pyenv install 3.11.6
   ```

3. **Configure Poetry to Use Python 3.11**
   ```bash
   poetry env use $(pyenv root)/versions/3.11.6/bin/python
   ```

4. **Install Project Dependencies**
   ```bash
   poetry install
   ```

5. **Set Up Environment Variables**
   Copy the sample environment file and configure your OpenAI API key:
   ```bash
   cp settings.env.sample settings.env
   # Edit settings.env to add your OpenAI API key and other configurations
   source settings.env
   ```

6. **Verify Installation**
   Run the following command to verify the `fz` CLI is functional:
   ```bash
   poetry run fz --help
   ```

## Running Fuzzomatic

To run Fuzzomatic on a codebase:
```bash
poetry run fz <codebase_dir> --stop-on bug --max-fuzz-targets 2
```

## Additional Notes

- If you encounter issues with dependencies, ensure all prerequisites are installed and properly configured.
- Refer to the [Fuzzomatic README](https://github.com/kudelskisecurity/fuzzomatic) for more details on usage and advanced options.

---

By following these steps, you can set up and run Fuzzomatic from scratch.


poetry run fz /home/yzzhao3/sbomfuzz-fuzzomatic-eval/crate_batch_1 --stop-on building --max-fuzz-targets 20 --force

**go to fuzzomatic dir**
source settings.env && poetry run fz /home/yzzhao3/sbomfuzz-fuzzomatic-eval/crate_batch_1 --stop-on building --max-fuzz-targets 20 --force 

**revert harnesses**
python3 extract_harnesses.py crate_batch_1 --backup

# Run all targets for 24 hours (default) in a tmux session
python3 run_all_fuzz.py crate_batch_1

> **Note:** The script runs `cargo +nightly fuzz run` under the hood. Ensure the nightly toolchain is installed (`rustup toolchain install nightly`).

# Run with 4 forked workers per target to keep fuzzing across crashes
python3 run_all_fuzz.py crate_batch_1 --forks 4

# Custom session name and shorter time
python3 run_all_fuzz.py crate_batch_1 --session-name my_fuzz --time 3600

# Run and immediately attach to the session
python3 run_all_fuzz.py crate_batch_1 --attach

# Attach to existing session later
tmux attach -t fuzz_session