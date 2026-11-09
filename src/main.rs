use std::{env, fs, io};

mod fetch;
use fetch::*;

fn main() {
    let mut output = String::new();

    if let Some(distro) = distro() {
        output.push_str(" OS: {distro}");
    }

    if let Some(shell) = shell() {
        output.push_str("Shell: {shell}");
    }

    if let Some(name) = name() {
        output.push_str("Name: {name}")
    }

    if let Some(wm) = wm() {
        output.push_str("WM: {wm}")
    }

    println!("{output}");
}
