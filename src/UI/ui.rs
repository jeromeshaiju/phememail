use std::io::{self, BufRead, Write};
mod login;
mod mailboxui;

use crate::storage::accountsdbms;

//commands
const  exit:&str = "exit";
const login:&str = "login";
const infome: &str = "infome";
const dropacct: &str = "dropacct";
const mailbox:&str = "mailbox";

pub static mut IS_LOGGED_IN: bool = false;

pub fn is_logged() {
    unsafe {
        IS_LOGGED_IN = true;
    }
}

    
pub fn main_interface(){
    println!("=======================P=H=E=M=E=M=A=I=L=======================");

    loop {
        print!("-> ");
        io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("you have requested:{}",input);
        let command = input.trim();
        match command {
            exit => {
                println!("Goodbye!");
                break;
            }
           login=> {
                login::login_interface();
            }
            infome => {
                println!("============================I=N=F=O============================");
                if unsafe{IS_LOGGED_IN} {
                    login::userinfo();
                } else {
                    println!("You are not logged in. Please log in first.");
                }

            }
            dropacct=> {
                login::drop_account();
            }
            mailbox => {
            println!("============================M=A=I=L=B=O=X============================");
            if unsafe{IS_LOGGED_IN} {
                    let email = accountsdbms::current_user_email();
                    mailboxui::mailbox_interface(email.as_ref().unwrap());
                } else {
                    println!("You are not logged in. Please log in first.");
                }
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}

