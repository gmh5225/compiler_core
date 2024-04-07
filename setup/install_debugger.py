import subprocess
import platform

def run_command(command, capture_output=False):
    try:
        if capture_output:
            result = subprocess.run(command, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True)
            return True, result.stdout.decode('utf-8').strip()
        else:
            subprocess.run(command, check=True, shell=True)
            return True, ""
    except subprocess.CalledProcessError:
        return False, ""

def install_on_mac():
    print("Checking for Xcode Command Line Tools on macOS...")
    installed, _ = run_command("xcode-select -p", True)
    if installed:
        print("Xcode Command Line Tools are already installed.")
    else:
        print("Xcode Command Line Tools not found. Installing...")
        run_command("xcode-select --install")

    lldb_installed, _ = run_command("lldb --version", True)
    if lldb_installed:
        print("LLDB installed.")
    else:
        print("LLDB not successfully installed. Try installing Xcode from the App Store.")

def install_on_linux():
    print("Installing GDB on Linux...")
    run_command("sudo apt-get update")
    run_command("sudo apt-get install gdb")

def main():
    os_system = platform.system()
    
    if os_system == "Darwin":
        install_on_mac()
    elif os_system == "Linux":
        install_on_linux()
    else:
        print("Unsupported operating system.")

if __name__ == "__main__":
    main()
