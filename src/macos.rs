use crate::errors::MachineIdError;
use crate::utils::exec;

pub fn get_machine_id() -> Result<String, MachineIdError> {
    // ioreg -d2 -c IOPlatformExpertDevice | awk -F\\\" '/IOPlatformUUID/{print $(NF-1)}'
    exec("ioreg -d2 -c IOPlatformExpertDevice | awk -F\\\" '/IOPlatformUUID/{print $(NF-1)}'")
        .ok_or(MachineIdError::NotFound)
}
