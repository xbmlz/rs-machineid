#![cfg(target_os = "macos")]

use crate::errors::MachineIdError;
use crate::utils::exec;

pub fn get_machine_id() -> Result<String, MachineIdError> {
    if let Some(out) =
        exec(r#"ioreg -d2 -c IOPlatformExpertDevice | awk -F\" '/IOPlatformUUID/{print $(NF-1)}'"#)
    {
        let id = out.trim().to_string();
        if !id.is_empty() {
            return Ok(id);
        }
    }

    if let Some(out) =
        exec(r#"system_profiler SPHardwareDataType | awk '/Hardware UUID/ {print $3}'"#)
    {
        let id = out.trim().to_string();
        if !id.is_empty() {
            return Ok(id);
        }
    }
    Err(MachineIdError::NotFound)
}
