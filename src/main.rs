use std::{env, fs, io};

fn main() {
    if let Some(s) = distro() {
        println!("{s}");
    }
}

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
