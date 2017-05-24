#[macro_use]
extern crate clap;
extern crate toml;
extern crate libreauth;

mod manager;

use clap::App;

fn main() {
    let yaml = load_yaml!("../settings.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    if let Some(new) = matches.subcommand_matches("add") {
        let name = new.value_of("name").unwrap();
        let secret = new.value_of("secret").unwrap();
        
        match manager::insert_secret(&name, &secret) {
            Ok(_) => println!("Code was addedd"),
            Err(_) => println!("Cannot add new code"),
        }
    }

    if let Some(get) = matches.subcommand_matches("get") {
        let name = get.value_of("name").unwrap();

        match manager::print_code(name) {
            Ok(code) => println!("Code for {} is {}", name, code),
            Err(e) => println!("Error: {}", e),
        }
    }

    if let Some(_) = matches.subcommand_matches("show") {
        match manager::print_codes() {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        }
    }
}

