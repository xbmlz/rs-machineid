#![cfg(target_os = "windows")]

use crate::{errors::MachineIdError, utils::exec};
use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_64KEY};

pub fn get_machine_id() -> Result<String, MachineIdError> {
    // Try to get the MachineGuid from the registry
    if let Ok(rkey) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(
        "SOFTWARE\\Microsoft\\Cryptography",
        KEY_READ | KEY_WOW64_64KEY,
    ) {
        if let Ok(id) = rkey.get_value::<String, _>("MachineGuid") {
            return Ok(id);
        }
    }
    // Try to get the MachineGuid from PowerShell
    if let Some(id) = exec(
        "powershell.exe -ExecutionPolicy bypass -command (Get-CimInstance -Class Win32_ComputerSystemProduct).UUID",
    ) {
        let id = id.trim().to_string();
        if !id.is_empty() {
            return Ok(id);
        }
    }

    if let Some(out) = exec("wmic csproduct get uuid") {
        if out.matches('\n').count() > 1 {
            let id = out
                .lines()
                .nth(2)
                .map(|s| s.trim())
                .unwrap_or_default()
                .to_string();
            if !id.is_empty() {
                return Ok(id);
            }
        }
    }

    Err(MachineIdError::NotFound)
}
