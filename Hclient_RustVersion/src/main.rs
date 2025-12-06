// COPYRIGHT: Hclient Rust Version, a CLI multi tool rust program for general tech purposes.    Copyright (C) 2025  Kevin De Togni, MKlabs
// LICENSE: distribuited on the GNU general public license 3.0v license
// CRATES
use std::io;
// MAIN
fn main() {
    // LOGO, CREDIT AND FIST PROMPT
    println!("Hclient Rust Edition, a CLI multi tool rust program for general tech purposes.    Copyright (C) 2025  Kevin De Togni, MKlabs");
    println!("distribuited on the GNU general public license 3.0v license");
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
    println!("/________/   /_______ / /_________/ /________/ /___/ /_________/ /___/  |_____/      /___/                    version: 0.2 BETA");
    println!("                                          Rust Version");
    println!();
    println!("some features only work on linux and its recommended to run Hclient as root or some feature will not work!");
    println!();
    println!("slect an option: 1 = passgen, 2 = btcadressgen, 3 = cardnumbergen, 4 = nmap local ip scan, 5 =  system information, 6 = pkg updater");
    println!();    
    let mut user_firstPrompt_choice: String = String::new();
    io::stdin().read_line(&mut user_firstPrompt_choice);
    // PASSGEN
    if user_firstPrompt_choice == "1" {
        let pgletters: &str = "qwertyuiopasdfghjklzxcvbnm";
        let pglettersandnumbers: &str = "qwertyuiopasdfghjklzxcvbnm1234567890";
        //PASSGEN - TYPE OF PASSWORDS
        println!("what type of passwords do you want to generate? 1 = numbers only, 2 = letters only, 3 = numbers and letters");
        println!();
        let mut user_passgen_choice: String = String::new();
        io::stdin().read_line(&mut user_passgen_choice);
        //PASSGEN - NUMBER OF PASSWORDS
        println!("how many passwords do you want to generate?");
        println!();
        let mut user_passgen_password_number_choice: String = String::new();
        io::stdin().read_line(&mut user_passgen_password_number_choice);
    }
}