// COPYRIGHT: Hclient Rust Version, a CLI multi tool rust program for general tech purposes.    Copyright (C) 2025  Kevin De Togni, MKlabs
// LICENSE: distribuited on the GNU general public license 3.0v license
// CRATES
use std::io;
use rand::prelude::*;
use std::process::Command;

// MAIN
fn main() {
    
    // LOGO, CREDITS AND FIST PROMPT
    println!("Hclient Rust Edition, a CLI multi tool rust program for general tech purposes.    Copyright (C) 2025  Kevin De Togni, MKlabs");
    println!("distribuited on the GNU general public license 3.0v license");
    println!("-------------------------------------------------------------------------------------------------------------------------------------------------");
    println!();
    println!("          _________    _________");
    println!("         /        /   /        /");
    println!("        /        /   /        /");
    println!("       /        /   /        /");
    println!("      /        /___/        / _________   ____       ____  _________   ____     ____  _____________           by MKlabs developer team");
    println!("     /                     / /        /  /   /      /   / /   _____/  /    |   /   / /            /           devs: MrMaxX");
    println!("    /        ____         / /   _____/  /   /      /   / /   /       /     |  /   / /____    ____/");
    println!("   /        /   /        / /   /       /   /      /   / /   /____   /      | /   /      /   /                 official website:");
    println!("  /        /   /        / /   /_____  /   /      /   / /    ____/  /   /|  |/   /      /   /                  officialmklabsveneto.netlify.app");
    println!(" /        /   /        / /         / /   /____  /   / /    /____  /   / |      /      /   /");
    println!("/________/   /_______ / /_________/ /________/ /___/ /_________/ /___/  |_____/      /___/                    version: 1.1.0 APP");
    println!();
    println!("                                        ---|Rust Version|---");
    println!("-------------------------------------------------------------------------------------------------------------------------------------------------");
    println!();
    println!("NOTE: some features only work on linux!");
    println!();
    println!("select an option: 1 = passgen, 2 = btcadressgen, 3 = nmap local ip scan, 4 = system information, 5 = pkg updater");
    println!();    
    let mut user_first_prompt_choice = String::new();
    io::stdin().read_line(&mut user_first_prompt_choice).expect("failed to read line");
    let user_first_prompt_choice_f: i32 = user_first_prompt_choice.trim().parse().unwrap();
    println!();
    
    // PASSGEN
    if user_first_prompt_choice_f == 1 {
        
        // PASSGEN - TYPE OF PASSWORDS
        println!("what type of passwords do you want to generate? 1 = numbers only, 2 = letters only, 3 = numbers and letters");
        println!();
        let mut user_passgen_choice: String = String::new();
        io::stdin().read_line(&mut user_passgen_choice).expect("failed to read line");
        let user_passgen_choice_f: i32 = user_passgen_choice.trim().parse().unwrap();
        
        // PASSGEN - NUMBER OF PASSWORDS
        println!();
        println!("how many passwords do you want to generate?");
        println!();
        let mut user_passgen_password_number_choice = String::new();
        io::stdin().read_line(&mut user_passgen_password_number_choice).expect("failed to read input");
        let user_passgen_password_number_choice_f: i32 = user_passgen_password_number_choice.trim().parse().unwrap();
        println!();
        
        // PASSGEN - LENGTH OF PASSWORDS
        println!("how long do you want them to be?");
        println!();
        let mut user_passgen_password_lenght_choice = String::new();
        io::stdin().read_line(&mut user_passgen_password_lenght_choice).expect("failed to read line");
        let user_passgen_password_lenght_choice_f = user_passgen_password_lenght_choice.trim().parse().unwrap();
        println!();
        
        // PASSGEN - NUMBERS ONLY
        if user_passgen_choice_f == 1 {
            let mut rng = rand::rng();
            for _i in 0..user_passgen_password_number_choice_f {
                for _ in 0..user_passgen_password_lenght_choice_f {
                    print!("{}", rng.sample(rand::distr::Alphanumeric) as i32);
                }
                println!();
            }
        }
        
        // PASSGEN - LETTERS ONLY
        else if user_passgen_choice_f == 2 {
            let mut rng = rand::rng();
            println!();
            for _i in 0..user_passgen_password_number_choice_f {
                for _ in 0..user_passgen_password_lenght_choice_f {
                    print!("{}", rng.sample(rand::distr::Alphabetic) as char);
                }
                println!();
            }
        }
        
        // PASSGEN - NUMBERS AND LETTERS
        else if user_passgen_choice_f == 3 {
            let mut rng = rand::rng();
            println!();
            for _i in 0..user_passgen_password_number_choice_f {
                for _ in 0..user_passgen_password_lenght_choice_f {
                    print!("{}", rng.sample(rand::distr::Alphanumeric) as char);
                }
                println!();
            }
        }
    }
    
    // BTCADDRESSGEN
    else if user_first_prompt_choice_f == 2 {
        
        // BTCADDRESSGEN - NUMBER OF ADDRESSES
        println!("how many adresses do you want to generate?");
        println!();
        let mut user_btcaddressgen_password_number_choice = String::new();
        io::stdin().read_line(&mut user_btcaddressgen_password_number_choice).expect("failed to read line");
        let user_btcaddressgen_password_number_choice_f: i32 = user_btcaddressgen_password_number_choice.trim().parse().unwrap();
        let mut rng = rand::rng();
        println!();
        
        // BTCADDRESSGEN - ADDRESS TYPE
        println!("what type do you wanna use? 1 = Legacy (P2PKH), 2 = P2SH, 3 = Bech32 (P2WPKH), 4 = Bech32 (P2WSH), 5 = Taproot");
        println!();
        let mut user_bitcoin_type_choice = String::new();
        io::stdin().read_line(&mut user_bitcoin_type_choice).expect("failed to read line");
        let user_bitcoin_type_choice_f:i32 = user_bitcoin_type_choice.trim().parse().unwrap();
        println!();
        
        // BTCADDRESSGEN - LEGACY (P2PKH)
        if user_bitcoin_type_choice_f == 1 {
            for _i in 0..user_btcaddressgen_password_number_choice_f {
                print!("1");
                for _ in 0..=33 {
                    print!("{}", rng.sample(rand::distr::Alphanumeric) as char);
                }
                println!();
            }
        }
        
        // BTCADDRESSGEN - P2SH
        if user_bitcoin_type_choice_f == 2 {
            for _i in 0..user_btcaddressgen_password_number_choice_f {
                print!("3");
                for _ in 0..=33 {
                    print!("{}", rng.sample(rand::distr::Alphanumeric) as char);
                }
                println!();
            }
        }
        
        // BTCADDRESSGEN - BECH32 (P2WPKH)
        if user_bitcoin_type_choice_f == 3 {
            for _i in 0..user_btcaddressgen_password_number_choice_f {
                print!("bc1q");
                for _ in 0..=38 {
                    print!("{}", rng.sample(rand::distr::Alphanumeric) as char);
                }
                println!();
            }
        }
        
        // BTCADDRESSGEN - BECH32 (P2WSH)
        if user_bitcoin_type_choice_f == 4 {
            for _i in 0..user_btcaddressgen_password_number_choice_f {
                print!("bc1q");
                for _ in 0..=58 {
                    print!("{}", rng.sample(rand::distr::Alphanumeric) as char);
                }
                println!();
            }
        }
        
        // BTCADDRESSGEN - TAPROOT
        if user_bitcoin_type_choice_f == 5 {
            for _i in 0..user_btcaddressgen_password_number_choice_f {
                print!("bc1p");
                for _ in 0..58 {
                    print!("{}", rng.sample(rand::distr::Alphanumeric) as char);
                }
                println!();
            }
        }
    }
    
    // NMAP LOCAL IP SCAN
    else if user_first_prompt_choice_f == 3 {
        
        // NMAP - PORT
        println!("select port: 1 = 24, 2 = 16 (the port 16 takes longer to scan than the port 24");
        println!();
        let mut user_nmap_port_selection = String::new();
        io::stdin().read_line(&mut user_nmap_port_selection).expect("failed to read line");
        let user_nmap_port_selection_f:i32 = user_nmap_port_selection.trim().parse().unwrap();
        println!();
        
        // NMAP - PORT 24
        if user_nmap_port_selection_f == 1 {
            let mut sh = Command::new("sh");
            sh.arg("nmap24.sh");
            match sh.output() {
                Ok(o) => {
                    unsafe {
                        println!("{}", String::from_utf8_unchecked(o.stdout));
                    }
                }
                Err(e) => {
                    println!("{}",e);
                }
            }
        }
        
        // NMAP - PORT 16
        else if user_nmap_port_selection_f == 2 {
            let mut sh = Command::new("sh");
            sh.arg("nmap16.sh");
            match sh.output() {
                Ok(o) => {
                    unsafe {
                        println!("{}", String::from_utf8_unchecked(o.stdout));
                    }
                }
                Err(e) => {
                    println!("{}",e);
                }
            }
        }
    }

    // SYSTEM INFORMATION
    else if user_first_prompt_choice_f == 4 {
        
        // SYSTEM INFORMATION - FETCH TYPE
        println!("select fetch type: 1 = neofetch, 2 = fastfetch, 3 = macchina, 4 = hyfetch");
        println!();
        let mut user_fetch_type = String::new();
        io::stdin().read_line(&mut user_fetch_type).expect("failed to read line");
        let user_fetch_type_f: i32 = user_fetch_type.trim().parse().unwrap();
        println!();
        
        // SYSTEM INFORMATION - NEOFETCH
        if user_fetch_type_f == 1 {
            let mut sh = Command::new("sh");
            sh.arg("neofetch.sh");
            match sh.output() {
                Ok(o) => {
                    unsafe {
                        println!("{}", String::from_utf8_unchecked(o.stdout));
                    }
                }
                Err(e) => {
                    println!("{}",e);
                }
            }
        }
        
        // SYSTEM INFORMATION - FASTFETCH
        else if user_fetch_type_f == 2 {
            let mut sh = Command::new("sh");
            sh.arg("fastfetch.sh");
            match sh.output() {
                Ok(o) => {
                    unsafe {
                        println!("{}", String::from_utf8_unchecked(o.stdout));
                    }
                }
                Err(e) => {
                    println!("{}",e);
                }
            }
        }
        
        // SYSTEM INFORMATION - MACCHINA
        else if user_fetch_type_f == 3 {
            let mut sh = Command::new("sh");
            sh.arg("macchina.sh");
            match sh.output() {
                Ok(o) => {
                    unsafe {
                        println!("{}", String::from_utf8_unchecked(o.stdout));
                    }
                }
                Err(e) => {
                    println!("{}",e);
                }
            }
        } 
        
        // SYSTEM INFORMATION - HYFETCH
        else if user_fetch_type_f == 4 {
            let mut sh = Command::new("sh");
            sh.arg("hyfetch.sh");
            match sh.output() {
                Ok(o) => {
                    unsafe {
                        println!("{}", String::from_utf8_unchecked(o.stdout));
                    }
                }
                Err(e) => {
                    println!("{}",e);
                }
            }
        }
    }
    
    // PACKAGE UPDATER
    else if user_first_prompt_choice_f == 5 {
        
        // PACKAGE UPDATER - PACKAGE MANAGER
        println!("select package manager: 1 = apt, 2 = dnf, 3 = pacman");
        println!();
        let mut user_package_manager = String::new();
        io::stdin().read_line(&mut user_package_manager).expect("failed to read line");
        let user_package_manager_f: i32 = user_package_manager.trim().parse().unwrap();
        println!();
        
        // PACKAGE UPDATER - DEBIAN
        if user_package_manager_f == 1 {
            let mut sh = Command::new("sh");
            sh.arg("apt.sh");
            match sh.output() {
                Ok(o) => {
                    unsafe {
                        println!("{}", String::from_utf8_unchecked(o.stdout));
                    }
                }
                Err(e) => {
                    println!("{}",e);
                }
            }
        }
        
        // PACKAGE UPDATER - FEDORA
        else if user_package_manager_f == 2 {
            let mut sh = Command::new("sh");
            sh.arg("dnf.sh");
            match sh.output() {
                Ok(o) => {
                    unsafe {
                        println!("{}", String::from_utf8_unchecked(o.stdout));
                    }
                }
                Err(e) => {
                    println!("{}",e);
                }
            }
        }
        
        // PACKAGE UPDATER - ARCH
        else if user_package_manager_f == 3 {
            let mut sh = Command::new("sh");
            sh.arg("pacman.sh");
            match sh.output() {
                Ok(o) => {
                    unsafe {
                        println!("{}", String::from_utf8_unchecked(o.stdout));
                    }
                }
                Err(e) => {
                    println!("{}",e);
                }
            }
        }
    }
    
    // END
    println!();
    println!("press ENTER to exit");
    let mut end = String::new();
    io::stdin().read_line(&mut end).expect("failed to read line");
}