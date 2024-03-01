import subprocess

def run_command(command, shell=True):
    try:
        subprocess.run(command, check=True, shell=shell, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        return True
    except subprocess.CalledProcessError:
        return False

def install_rust():
    print("Installing Rust...")
    if run_command("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"):
        print("Rust/Cargo installation script executed successfully.")
        print("Please run 'source $HOME/.cargo/env' or restart your terminal to make Rust available.")
    else:
        print("There was an error running the Rust/Cargo installation script.")

if __name__ == "__main__":
    install_rust()
 