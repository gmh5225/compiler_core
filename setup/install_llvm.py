import subprocess
import platform
import os

def run_command(command):
    try:
        subprocess.run(command, check=True, shell=True)
        print(f"Command executed successfully: {command}")
    except subprocess.CalledProcessError as e:
        print(f"Command failed: {command}\nError: {e}")

def install_llvm_mac():
    print("Installing LLVM 17 on macOS...")
    run_command("brew install llvm@17")
    print("Exporting LLVM 17 to PATH...")
    # This change will not be reflected in the current shell session
    with open(os.path.expanduser("~/.bash_profile"), "a") as bash_profile:
        bash_profile.write('\nexport PATH="/usr/local/opt/llvm@17/bin:$PATH"\n')
    print("Please manually run 'source ~/.bash_profile' to update your PATH.")

def install_llvm_linux():
    print("Installing LLVM 17 on Linux...")
    run_command("sudo apt-get update")
    run_command("sudo apt-get install -y software-properties-common")
    run_command("wget https://apt.llvm.org/llvm.sh")
    run_command("chmod +x llvm.sh")
    run_command("sudo ./llvm.sh 17")

def main():
    os_system = platform.system()
    
    if os_system == "Darwin":
        install_llvm_mac()
    elif os_system == "Linux":
        install_llvm_linux()
    else:
        print("Unsupported operating system.")

if __name__ == "__main__":
    main()
