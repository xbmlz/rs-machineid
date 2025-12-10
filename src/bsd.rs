#![cfg(any(
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
))]

use crate::{
    errors::MachineIdError,
    utils::{exec, read_file},
};

pub fn get_machine_id() -> Result<String, MachineIdError> {
    if let Some(out) = read_file("/etc/hostid") {
        let id = out.trim().to_string();
        if !id.is_empty() {
            return Ok(id);
        }
    }
    if let Some(out) = exec(r#"kenv -q smbios.system.uuid"#) {
        let id = out.trim().to_string();
        if !id.is_empty() {
            return Ok(id);
        }
    }
    Err(MachineIdError::NotFound)
}
