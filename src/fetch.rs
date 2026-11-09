use std::env;

pub fn distro() -> Option<String> {
    let contents = std::fs::read_to_string("/etc/os-release").ok()?;

    for l in contents.lines() {
        if let Some((key, val)) = l.split_once('=')
            && key == "PRETTY_NAME"
        {
            return Some(val.trim_matches('"').to_string());
        }
    }

    None
}

pub fn shell() -> Option<String> {
    std::env::var("SHELL").ok()
}

pub fn wm() -> Option<String> {
    std::env::var("XDG_CURRENT_DESKTOP").ok()
}

pub fn name() -> Option<String> {
    std::env::var("LOGNAME").ok()
}
