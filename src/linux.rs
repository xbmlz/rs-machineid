#![cfg(target_os = "linux")]

use crate::{
    errors::MachineIdError,
    utils::{exec, read_file},
};

pub fn get_machine_id() -> Result<String, MachineIdError> {
    // /var/lib/dbus/machine-id
    if let Some(out) = read_file("/var/lib/dbus/machine-id") {
        let id = out.trim();
        if !id.is_empty() {
            return Ok(id.to_string());
        }
    }

    // /etc/machine-id
    if let Some(out) = read_file("/etc/machine-id") {
        let id = out.trim();
        if !id.is_empty() {
            return Ok(id.to_string());
        }
    }

    // docker - cgroup
    if let Some(out) = read_file("/proc/self/cgroup")
        && !out.trim().is_empty()
        && out.contains("docker")
        && let Some(id) = exec(r#"head -1 /proc/self/cgroup | cut -d/ -f3"#)
    {
        return Ok(id);
    }

    // docker - mountinfo
    if let Some(out) = read_file("/proc/self/mountinfo")
        && !out.trim().is_empty()
        && out.contains("docker")
        && let Some(id) = exec(
            r#"grep -oP '(?<=docker/containers/)([a-f0-9]+)(?=/hostname)' /proc/self/mountinfo'"#,
        )
    {
        return Ok(id);
    }

    // WSL
    if let Some(out) = exec("uname -a")
        && (out.contains("Microsoft") || out.contains("microsoft"))
        && let Some(id) = exec(
            r#"powershell.exe -ExecutionPolicy bypass -command '(Get-CimInstance -Class Win32_ComputerSystemProduct).UUID'"#,
        )
    {
        return Ok(id);
    }

    Err(MachineIdError::NotFound)
}
