use std::io::{self, BufRead,Write};
enum StartCommands {
    Login,
    Exit,
}

pub fn main_interface() {
    println!("====PHEMEMAIL====");

    loop {
        print!("-> ");
        io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("you have requested:{}",input);
        let command = input.trim();
        if command == "exit" {
            println!("Goodbye!");
            break;
        } else if command == "login" {
            println!("Login functionality is not implemented yet.");
        } else {
            println!("Unknown command: {}", command);
        }


        }
        

}