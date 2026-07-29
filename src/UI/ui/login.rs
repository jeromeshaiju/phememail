use std::io::{self, BufRead,Write};
use crate::account;
use crate::storage::accountsdbms::adduser;
use crate::storage::accountsdbms::userinfofomdb;
use crate::storage::accountsdbms::getemails;
use crate::storage::accountsdbms::password_check;
use crate::storage::accountsdbms::current_user;
use crate::storage::accountsdbms::get_name;




pub fn login_interface(){
    println!("=======================L=O=G=I=N=======================");
    println!("choose one of the following options:");
    let emails =getemails().expect("Failed to get emails from database");
    println!("enter 0 to create a new account or select an existing account by number:");

    if emails.is_empty() {
        println!("No users found in the database. Please create a new account.");
    }
    else {
        for (i, email) in emails.iter().enumerate() {
            println!("{}. {}", i + 1, email);
        }
    }
    println!("enter exit to exit the login interface.");
    loop {
        print!("-> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("you have requested:{}",input);
        let command = input.trim();
        match command {
            "exit" => {
                println!("exit the login interface.");
                break;
            }
            "0" => {
                println!("Email address: ");
                let mut email = String::new();
                io::stdin().read_line(&mut email).expect("Failed to read line");
                println!("Password: ");
                let mut password = String::new();
                io::stdin().read_line(&mut password).expect("Failed to read line");
                println!("username: ");
                let mut username = String::new();
                io::stdin().read_line(&mut username).expect("Failed to read line");

                let email = email.trim();
                let username = username.trim();
                let password = password.trim();

                println!("You entered email: {}, username: {},password: {},", email,username,password);
                let account = account::Account::new(email.to_string(),password.to_string(),username.to_string());
                adduser(&account.email, &account.name, &account.password).expect("Failed to add user");
                current_user(account.email, account.name).expect("Failed to set current user");
                println!("Account created successfully!");
                break
            }
            _ => {
                // the command doesnt work while entering other values and exits the application, so I will erorr chekk later
                println!("you have selected the account: {}",emails.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()));
                println!("please enter the password for the selected account:");
                print!("-> ");
                io::stdout().flush().expect("Failed to flush stdout");
                let mut password = String::new();
                io::stdin().read_line(&mut password).expect("Failed to read line");
                let password = password.trim();
                if password_check(emails.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()), password).unwrap_or(false){
                    println!("you have logged in with the account: {}",emails.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()));
                }else{
                    println!("Incorrect password for the account: {}",emails.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()));
                }
                let email = emails.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()).to_string();
                current_user(email.clone(), get_name(email).unwrap_or_else(|_e| "Unknown".to_string())).expect("Failed to set current user");
                break
            }
        }
    }
}
pub fn userinfo(){
    userinfofomdb();
}
