import subprocess
import platform

def run_command(command):
    try:
        subprocess.run(command, check=True, shell=True)
        return True
    except subprocess.CalledProcessError:
        return False

def install_on_linux():
    print("Installing perf on Linux...")
    if run_command('sudo apt-get update') and run_command('sudo apt-get install -y linux-tools-$(uname -r) linux-tools-generic'):
        print("Installed version:")
        run_command('perf --version')
    else:
        print("Failed to install perf on Linux.")

def install_on_mac():
    print("Checking for Instruments on macOS...")
    if not run_command('xcode-select -p'):
        print("Installing Xcode Command Line Tools...")
        run_command('xcode-select --install')
    else:
        print("Xcode Command Line Tools already installed.")
    print("Instruments is part of Xcode and should be already installed.")
    print("Installed version:")
    run_command('xcodebuild -version')

def main():
    os_system = platform.system()
    
    if os_system == "Linux":
        install_on_linux()
    elif os_system == "Darwin":
        install_on_mac()
    else:
        print("Unsupported operating system for this script.")
        print("For Windows, consider using Visual Studio's Performance Profiler.")

if __name__ == "__main__":
    main()
