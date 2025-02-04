# Universal Audio Driver  

A Rust-based tool for Windows that auto-detects audio hardware (headphones, speakers, microphones, etc) and installs the required drivers. Designed to replace manual driver searches with a one-click solution.  

## Download  
1. Go to the [Releases](https://github.com/muzondie/universal-audio-driver/releases) tab.  
2. Download the latest `.zip` file.  
3. Unzip the file and run `UniversalAudioDriver.exe`.  

## Usage  
1. **Run the app** after unzipping (no installation needed).  
2. Wait for the tool to **auto-scan** your connected audio devices.  
3. Click **Install** to apply the correct drivers.  
4. Restart your device if prompted.  

## Features  
- **Automatic Detection:** Identifies speakers, headphones, microphones, sound cards, and more.  
- **Offline Database:** Works without internet as well, includes common drivers (Realtek, Intel, AMD, etc.).  
- **Silent Installation:** No pop-ups or interruptions during setup.  
- **Lightweight:** Minimal system resource usage.  
- **Error Logging:** Generates logs for troubleshooting failed installations.  
- **Windows 10/11 Support:** Optimized for the latest OS versions.  
- **GUI Interface:** Simple progress bars and status updates.  

## Build from Source  
1. Install [Rust](https://www.rust-lang.org/tools/install).  
2. Clone the repository:  
   ```bash  
   git clone https://github.com/muzondie/universal-audio-driver.git  
   ```  
3. Build the project:  
   ```bash  
   cd universal-audio-driver  
   cargo build --release  
   ```  
4. The executable will be in `target/release/`.  

## Contributing  
Contributions are currently closed due to limited maintenance capacity.  

## License  
MIT License. See [LICENSE](LICENSE) for details.
