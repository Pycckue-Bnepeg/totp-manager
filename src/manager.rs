use libreauth::oath::TOTPBuilder;
use std::io::prelude::*;
use std::fs::{OpenOptions, File};
use std::fmt;

use toml::Value;

pub fn insert_secret(name: &str, secret: &str) -> Result<(), TOTPError> {
    open_file()
        .and_then(|mut file| file.write_fmt(format_args!("{} = \"{}\"\n", name, secret)).map_err(|_| TOTPError::OpenFile))
}

pub fn print_code(name: &str) -> Result<String, TOTPError> {
    let secret = get_secret(name)?;
    get_code(&secret)
}

pub fn print_codes() -> Result<(), TOTPError> {
    let values = read_file()?;
    let table = values.as_table().ok_or(TOTPError::GetTOTP)?;

    for (key, value) in table {
        let secret = value.as_str().ok_or(TOTPError::GetTOTP)?;
        let code = get_code(&secret.to_owned())?;

        println!("Code for {} is {}", key, code);
    }

    Ok(())
}

fn get_code(secret: &String) -> Result<String, TOTPError> {
    let totp = TOTPBuilder::new()
        .base32_key(&secret)
        .finalize()
        .map_err(|_| TOTPError::GetTOTP)?;

    Ok(totp.generate())
}

fn get_secret(name: &str) -> Result<String, TOTPError> {
    let table = read_file()?;
    
    table.get(name)
        .and_then(|value| value.as_str())
        .and_then(|value| Some(value.to_owned()))
        .ok_or(TOTPError::NoRow(name.to_owned()))
}

fn read_file() -> Result<Value, TOTPError> {
    let mut content = String::new();

    open_file()
        .and_then(|mut file| file.read_to_string(&mut content).map_err(|_| TOTPError::ReadFile))?;

    content.parse::<Value>().map_err(|_| TOTPError::Parse)
}

fn open_file() -> Result<File, TOTPError> {
    OpenOptions::new().read(true).append(true).create(true)
        .open("keys.toml")
        .map_err(|_| TOTPError::OpenFile)
}

pub enum TOTPError {
    OpenFile,
    ReadFile,
    Parse,
    GetTOTP,
    NoRow(String)
}

impl fmt::Display for TOTPError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
            TOTPError::OpenFile => write!(f, "Cannot open file"),
            TOTPError::ReadFile => write!(f, "Cannot read file"),
            TOTPError::Parse => write!(f, "Cannot parse toml file"),
            TOTPError::GetTOTP => write!(f, "Cannot generate totp code"),
            TOTPError::NoRow(ref name) => write!(f, "There is no secret code for {}", name),
		}
	}    
}