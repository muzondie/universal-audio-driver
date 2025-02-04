use windows::Win32::Devices::DeviceAndDriverInstallation::{
    SetupDiGetClassDevsW, DIGCF_PRESENT, SP_DEVINFO_DATA,
};
use windows::Win32::Devices::Properties::DEVPKEY_Device_DeviceDesc;
use windows::core::PWSTR;
use std::ptr;

pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub manufacturer: String,
    pub driver_status: DriverStatus,
}

#[derive(Debug)]
pub enum DriverStatus {
    Installed,
    Missing,
    Outdated,
}

pub fn detect_audio_devices() -> Result<Vec<AudioDevice>, crate::error::Error> {
    let mut devices = Vec::new();
    unsafe {
        let hdev = SetupDiGetClassDevsW(
            None,
            None,
            None,
            DIGCF_PRESENT
        )?;
        
        let mut device_info = SP_DEVINFO_DATA::default();
        device_info.cbSize = std::mem::size_of::<SP_DEVINFO_DATA>() as u32;
        
        let mut index = 0;
        while SetupDiEnumDeviceInfo(hdev, index, &mut device_info).is_ok() {
            let mut buffer = [0u16; 256];
            let mut size = 0;
            
            windows::Win32::Devices::DeviceAndDriverInstallation::SetupDiGetDevicePropertyW(
                hdev,
                &mut device_info,
                &DEVPKEY_Device_DeviceDesc,
                &mut size,
                Some(buffer.as_mut_ptr()),
                buffer.len() as u32,
                None,
                0
            )?;
            
            let name = String::from_utf16_lossy(&buffer);
            devices.push(AudioDevice {
                id: format!("DEV{:08X}", device_info.DevInst),
                name,
                manufacturer: "Unknown".into(),
                driver_status: DriverStatus::Missing,
            });
            
            index += 1;
        }
    }
    Ok(devices)
}