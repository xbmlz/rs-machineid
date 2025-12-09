mod errors;
mod macos;
mod utils;
mod windows;

use errors::MachineIdError;

#[cfg(target_os = "macos")]
use macos::get_machine_id;

#[cfg(target_os = "windows")]
use windows::get_machine_id;

/// Get the machine ID
pub struct MachineId;

impl MachineId {
    /// Obtain the unique identifier of the machine.
    /// Return a unique ID in string form.
    pub fn get() -> Result<String, MachineIdError> {
        get_machine_id()
    }
}

#[cfg(test)]
mod tests {
    use super::MachineId;

    #[test]
    fn test_get() {
        let id = MachineId::get();
        assert!(id.is_ok());
        println!("{}", id.unwrap());
    }
}
