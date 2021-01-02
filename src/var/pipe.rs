use crate::scheme::*;
use crate::helper;
use std::io::{self, Read, BufRead};
use anyhow::{Context, Result};
use serde_json::{Deserializer, Value};

pub fn concatinate(scheme: &mut SCHEME) {

    if atty::isnt(atty::Stream::Stdin) {
        if let Ok(input) = read_stdin() {
            if let Ok(sh) = make_scheme(input) {
                *scheme = sh;
            }
        }
    }
}

fn read_stdin() -> Result<String> {
    let mut input = String::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        input.push_str(&line.unwrap());
    }
    Ok(input)
}

fn make_scheme(data: String) -> Result<SCHEME> {
    let scheme: SCHEME = serde_json::from_str(&data)?;
    Ok(scheme)
}
