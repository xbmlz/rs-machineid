use crate::errors::MachineIdError;

pub fn get_machine_id() -> Result<String, MachineIdError> {
    Ok("Windows".to_string())
}