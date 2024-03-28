import subprocess
import os
import sys

def run_command(command):
    try:
        result = subprocess.run(command, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True)
        return True, result.stdout.decode('utf-8')
    except subprocess.CalledProcessError as e:
        return False, e.stderr.decode('utf-8')

# Adjust the working directory to the project root if the script is in compiler_core/setup/
os.chdir(os.path.join(os.getcwd(), '..'))

# Build the project using Cargo
build_status, build_output = run_command('cargo build --release')
if not build_status:
    print("Build failed. Please check for any compilation errors.")
    sys.exit(1)

INSTALL_DIR = "/usr/local/bin"

# Check if the installation directory exists, and create it if it does not
if not os.path.isdir(INSTALL_DIR):
    create_dir_status, create_dir_output = run_command(f'sudo mkdir -p {INSTALL_DIR}')
    if not create_dir_status:
        print("Failed to create the installation directory. Please check permissions.")
        sys.exit(1)

BINARY_PATH = os.path.join(os.getcwd(), "target/release/compiler_core")

# Create a symbolic link in the installation directory
link_status, link_output = run_command(f'sudo ln -sf {BINARY_PATH} {INSTALL_DIR}/charge')
if not link_status:
    print(f"Failed to create a symbolic link to {BINARY_PATH} in {INSTALL_DIR}. Please check permissions.")
    sys.exit(1)

print("Installation complete. You can now use the 'charge' command.")
