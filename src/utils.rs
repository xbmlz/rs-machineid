use std::process::Command;

pub fn exec(cmd: &str) -> Option<String> {
    #[cfg(unix)]
    let output = Command::new("sh").arg("-c").arg(cmd).output().ok()?;

    #[cfg(windows)]
    let output = Command::new("cmd").arg("/C").arg(cmd).output().ok()?;

    if output.status.success() {
        String::from_utf8(output.stdout)
            .ok()
            .map(|s| s.trim().to_string())
    } else {
        None
    }
}

#[cfg(target_os = "linux")]
pub fn read_file(path: &str) -> Option<String> {
    match std::fs::read_to_string(path) {
        Ok(content) => Some(content.trim().to_string()),
        Err(_) => None,
    }
}

pub fn sanitize(id: &str) -> String {
    id.chars()
        .filter(|c| c.is_ascii_graphic() || c.is_ascii_punctuation() || c.is_ascii_alphanumeric())
        .collect()
}
