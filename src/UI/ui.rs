use std::io::{self, BufRead, Write};
mod login;


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
            "exit" => {
                println!("Goodbye!");
                break;
            }
            "login" => {
                login::login_interface();
            }
            "infome" => {
                println!("============================I=N=F=O============================");
                login::userinfo();

            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}