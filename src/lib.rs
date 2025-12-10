mod bsd;
mod errors;
mod linux;
mod macos;
mod utils;
mod windows;

use errors::MachineIdError;

#[cfg(target_os = "macos")]
use macos::get_machine_id;

#[cfg(target_os = "windows")]
use windows::get_machine_id;

#[cfg(target_os = "linux")]
use linux::get_machine_id;

#[cfg(any(
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "dragonfly",
    target_os = "macos",
    target_os = "ios"
))]
use bsd::get_machine_id;

use crate::utils::sanitize;

/// Get the machine ID
pub struct MachineId;

impl MachineId {
    /// Obtain the unique identifier of the machine.
    /// Return a unique ID in string form.
    pub fn get() -> Result<String, MachineIdError> {
        let id = get_machine_id();
        match id {
            Ok(id) => Ok(sanitize(&id)),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MachineId;

    #[test]
    fn test_get() {
        let id = MachineId::get();
        assert!(id.is_ok());

        let machine_id = id.unwrap();
        println!("Generated Machine ID: {}", machine_id);
        println!("Length: {}", machine_id.len());

        assert!(!machine_id.is_empty());
        assert!(machine_id.len() >= 6);
    }

    #[test]
    fn test_id_uniqueness() {
        let id1 = MachineId::get().unwrap();
        let id2 = MachineId::get().unwrap();
        let id3 = MachineId::get().unwrap();

        println!("ID 1: {}", id1);
        println!("ID 2: {}", id2);
        println!("ID 3: {}", id3);

        assert_eq!(id1, id2);
        assert_eq!(id2, id3);
    }
}
