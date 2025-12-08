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
    println!("/________/   /_______ / /_________/ /________/ /___/ /_________/ /___/  |_____/      /___/                    version: 0.0.1 ALPHA");
    println!();
    println!("                                           Rust Version");
    println!("-------------------------------------------------------------------------------------------------------------------------------------------------");
    println!();
    println!("some features only work on linux and its recommended to run Hclient as root or some feature will not work!");
    println!();
    println!("slect an option: 1 = passgen, 2 = btcadressgen, 3 = cardnumbergen, 4 = nmap local ip scan, 5 =  system information, 6 = pkg updater");
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
        let _user_passgen_choice_f: i32 = user_passgen_choice.trim().parse().unwrap();
        
        // PASSGEN - NUMBER OF PASSWORDS
        println!();
        println!("how many passwords do you want to generate?");
        println!();
        let mut user_passgen_password_number_choice = String::new();
        io::stdin().read_line(&mut user_passgen_password_number_choice).expect("failed to read input");
        let _user_passgen_password_number_choice_f: i32 = user_passgen_password_number_choice.trim().parse().unwrap();
        
        // PASSGEN - NUMBERS ONLY
        if _user_passgen_choice_f == 1 {
            let mut _rng = rand::rng();
            println!();
            for _i in 0.._user_passgen_password_number_choice_f {
                print!("---|{}", _rng.sample(rand::distr::Alphanumeric) as i128);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as i128);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as i128);
                println!("{}|---", _rng.sample(rand::distr::Alphanumeric) as i128);
            }
        }
        
        // PASSGEN - LETTERS ONLY
        else if _user_passgen_choice_f == 2 {
            let mut _rng = rand::rng();
            println!();
            for _i in 0.._user_passgen_password_number_choice_f {
                print!("---|{}", _rng.sample(rand::distr::Alphabetic) as char);
                print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
                print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
                print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
                print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
                print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
                print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
                print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
                print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
                println!("{}|---", _rng.sample(rand::distr::Alphabetic) as char);
            }
        }
        
        // PASSGEN - NUMBERS AND LETTERS
        else if _user_passgen_choice_f == 3 {
            let mut _rng = rand::rng();
            println!();
            for _i in 0.._user_passgen_password_number_choice_f {
                print!("---|{}", _rng.sample(rand::distr::Alphanumeric) as char);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as char);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as char);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as char);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as char);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as char);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as char);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as char);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as char);
                print!("{}", _rng.sample(rand::distr::Alphanumeric) as char);
                println!("{}|---", _rng.sample(rand::distr::Alphanumeric) as char);
            }
        }
    }
    
    // BTCADDRESSGEN
    else if user_first_prompt_choice_f == 2 {
        println!("how many adresses do you want to generate?");
        println!();
        let mut _user_btcaddressgen_password_number_choice = String::new();
        io::stdin().read_line(&mut _user_btcaddressgen_password_number_choice).expect("failed to read line");
        let _user_btcaddressgen_password_number_choice_f: i32 = _user_btcaddressgen_password_number_choice.trim().parse().unwrap();
        let mut _rng = rand::rng();
        println!();
        for _i in 0.._user_btcaddressgen_password_number_choice_f {
            print!("---|{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            print!("{}", _rng.sample(rand::distr::Alphabetic) as char);
            println!("{}|.---", _rng.sample(rand::distr::Alphabetic) as char);
        }
    }

    // CARDNUMBERGEN
    else if user_first_prompt_choice_f == 3 {
        println!("wich card do you want to generate: 1 = Visa, 2 = Mastercard, 3 = American Express");
        println!();
        let mut user_cardnumbergen_card_selection = String::new();
        io::stdin().read_line(&mut user_cardnumbergen_card_selection).expect("failed to read line");
        let user_cardnumbergen_card_selection_f: i32 = user_cardnumbergen_card_selection.trim().parse().unwrap();
        println!();
        println!("how many cards do you want to generate?");
        let mut user_cardnumbergen_card_number = String::new();
        io::stdin().read_line(& mut user_cardnumbergen_card_number).expect("failed to read line");
        let _user_cardnumbergen_card_number_f: i32 = user_cardnumbergen_card_number.trim().parse().unwrap();
        println!();
        
        // VISA
        if user_cardnumbergen_card_selection_f == 1 {
            for _i in 0.._user_cardnumbergen_card_number_f {
                println!("VISA");
                println!();
                
            }
        }
    }
    
    // NMAP LOCAL IP SCAN
    else if user_first_prompt_choice_f == 4 {
        let mut _final_ip = "192.168";
        println!("enter target local ip. example: 192.168.0.1");
        println!();
        let mut user_target_ip = String::new();
        io::stdin().read_line(&mut user_target_ip).expect("failed to read line");
        println!();
        Command::new("sh").arg("nmap -sS");
    }

    // SYSTEM INFORMATION
    else if user_first_prompt_choice_f == 5 {
        println!("select fetch type: 1 = neofetch, 2 = fastfetch, 3 = macchina, 4 = hyfetch");
        println!();
        let mut user_fetch_type = String::new();
        io::stdin().read_line(&mut user_fetch_type).expect("failed to read line");
        let _user_fetch_type_f: i32 = user_fetch_type.trim().parse().unwrap();
        println!();
        if _user_fetch_type_f == 1 {
            Command::new("sh").arg("neofetch").output().expect("failed to execute command");
        }
        else if _user_fetch_type_f == 2 {
            Command::new("sh").arg("fastfetch").output().expect("failed to execute command");
        }
        else if _user_fetch_type_f == 3 {
            Command::new("sh").arg("macchina").output().expect("failed to execute command");
        } 
        else if _user_fetch_type_f == 4 {
            Command::new("sh").arg("hyfetch").output().expect("failed to execute command");
        }
    }
    
    // PACKAGE UPDATER
    else if user_first_prompt_choice_f == 6 {
        println!("select package manager: 1 = apt, 2 = dnf, 3 = pacman");
        println!();
        let mut user_package_manager = String::new();
        io::stdin().read_line(&mut user_package_manager).expect("failed to read line");
        let user_package_manager_f: i32 = user_package_manager.trim().parse().unwrap();
        
        // PACKAGE UPDATER - DEBIAN
        if user_package_manager_f == 1 {
            Command::new("sh").arg("sudo apt update && upgrade");
        }
        
        // PACKAGE UPDATER - FEDORA
        else if user_package_manager_f == 2 {
            Command::new("sh").arg("sudo dnf update");
        }
        
        // PACKAGE UPDATER - ARCH
        else if user_package_manager_f == 3 {
            Command::new("sh").arg("sudo pacman -Syu");
        }
    }
    let mut end = String::new();
    io::stdin().read_line(&mut end).expect("failed to read line");
}