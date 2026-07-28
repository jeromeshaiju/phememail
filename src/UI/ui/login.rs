use std::io::{self, BufRead,Write};
use crate::account;
use crate::storage::accountsdbms::adduser;
use crate::storage::accountsdbms::userinfofomdb;




pub fn login_interface(){
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
            let password = password.trim();
            let username = username.trim();

            println!("You entered email: {}, password: {}, username: {}", email, password, username);
            let account = account::Account::new(email.to_string(),username.to_string(), password.to_string());
            adduser(&account.email, &account.name, &account.password).expect("Failed to add user");

}
pub fn userinfo(){
    userinfofomdb();
}