use std::io::{self, Write};
mod vault;

fn main() {
    vault::setup_database();
    loop {
        show_menu();
        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("failed to read the option");

        match selection.trim().parse::<u8>() {
            Ok(n) => match n {
                1 => handle_save_password(),
                2 => handle_search_password(),
                3 => handle_show_all_passwords(),
                4 => handle_delete_password(),
                5 => {
                    println!("Exited...see you later monkey");
                    std::process::exit(1);
                }
                _ => println!("i was right! Monkeys can't read...or do they?"),
            },
            Err(_) => println!("Invalid input, please try again."),
        }
    }
}

fn show_menu() {
    println!("");
    println!("ruspass: an interactive cli password manager");
    println!("1. add password");
    println!("2. search for password");
    println!("3. list all passwords");
    println!("4. delete password");
    println!("5. exit");
    println!("");
    println!("what you want to do man ? ");
}

fn handle_save_password() {
    println!("To add a password, ruspass would want website/note, username/email/phone, password.");
    println!("Please enter the website(ex: google.com) or note: ");

    // website
    io::stdout().flush().unwrap();
    let mut website = String::new();
    io::stdin()
        .read_line(&mut website)
        .expect("Failed to read website or note.");

    // username
    print!("Please enter the username/email/phone: ");
    io::stdout().flush().unwrap();
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read username/email/phone.");

    // password
    print!("Please enter the password: ");
    io::stdout().flush().unwrap();
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read password.");

    io::stdout().flush().unwrap();
    vault::add(website, username, password);
}

fn handle_search_password() {
    print!("Please enter the website for which you want the username and password: ");
    io::stdout().flush().unwrap();
    let mut website = String::new();
    io::stdin()
        .read_line(&mut website)
        .expect("Failed to read the input.");

    match vault::search(website.trim()) {
        Some(secret) => {
            println!("Username: {}", secret.username);
            println!("Password: {}", secret.password);
        }
        None => {
            eprintln!("No secret found.");
        }
    }
}

fn handle_show_all_passwords() {
    let string_pass = vault::get_all();
    println!("{}", string_pass);
}

fn handle_delete_password() {
    print!("Please enter the website for which you want delete the password: ");
    io::stdout().flush().unwrap();
    let mut website = String::new();
    io::stdin()
        .read_line(&mut website)
        .expect("Failed to read the input.");

    vault::delete(&website.trim());

    println!("Success: password deleted for {}", website.trim());
}
