use std::{env, fs, io};

fn main() {
    if let Ok(distro) = distro() {
        println!("{distro}");
    }
}

pub fn distro() -> io::Result<String> {
    let file = std::fs::read_to_string("/etc/os-release")?;

    for f in file.lines() {
        let mut key = Vec::<u8>::new();

        let mut i = 0;
        while f.as_bytes()[i] != b'=' {
            key.push(f.as_bytes()[i]);
            i += 1;
        }

        if String::from_utf8(key).unwrap() == "PRETTY_NAME" {
            return 

        }
    }

    Ok(String::new())
}
