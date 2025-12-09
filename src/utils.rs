use std::process::Command;

pub fn exec(cmd: &str) -> Option<String> {
    #[cfg(unix)]
    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .ok()?;

    #[cfg(windows)]
    let output = Command::new("cmd")
        .arg("/C")
        .arg(cmd)
        .output()
        .ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
    } else {
        None
    }
}
