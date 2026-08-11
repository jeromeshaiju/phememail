use std::io::{self, BufRead,Write};
use crate::storage::mailboxdbms::addmailbox;
use crate::storage::mailboxdbms::mailboxinfofromdb;
use crate::storage::mailboxdbms::getmailboxes;
use crate::storage::mailboxdbms::current_mailbox;
use crate::storage::mailboxdbms::drop_mailbox_from_db;
use crate::storage::mailboxdbms::get_email;





pub fn mailbox_interface(email:&str){
    println!("choose one of the following options:");
    let mailboxes =getmailboxes(email).expect("Failed to get mailboxes from database");
    println!("enter 0 to create a new mailbox or select an existing mailbox by number:");

    if mailboxes.is_empty() {
        println!("No mailboxes found in the database. Please create a new mailbox.");
    }
    else {
        for (i, mailbox_item) in mailboxes.iter().enumerate() {
            println!("{}. {}", i + 1, mailbox_item);
        }
    }
    println!("enter exit to exit the mailbox interface.");
    loop {
        print!("-> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim();
        match command {
            "exit" => {
                println!("exit the mailbox selection interface.");
                break;
            }
            input => match input.parse::<usize>() {
                Ok(0)=>{
                println!("name of the new mailbox: ");
                let mut name: String = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                
                let name = name.trim();
                println!("You entered name: {},", name);


                addmailbox(email,name).expect("Failed to add mailbox");
                current_mailbox(email.to_string(), name.to_string());
                println!("mailbox created successfully!");
                break
            },
            Ok(num)=> match mailboxes.get(num - 1){
                Some(email) =>{
                println!("you have selected the mailbox: {}",mailboxes.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()));
                let name_param = mailboxes.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()).to_string();
                let name_item=name_param.clone();
                current_mailbox(get_email(name_param).unwrap_or_else(|_e| "Unknown".to_string()),name_item);
                break
            }
            None=>{
                println!("Invalid selection")
            }
            },
            _=>{
                println!("Invalid option")
            }
        }
        }
    }
}
pub fn mailboxinfo(){
    mailboxinfofromdb();
}
// pub fn drop_account(){
//     println!("=======================D=R=O=P=A=C=C=O=U=N=T=======================");
//     println!("choose one of the following options:");
//     let emails =getemails().expect("Failed to get emails from database");
//     println!("enter 0 to create a new account or select an existing account by number:");
//     if emails.is_empty() {
//         println!("No users found in the database. Please create a new account.");
//     }
//     else {
//         for (i, email) in emails.iter().enumerate() {
//             println!("{}. {}", i + 1, email);
//         }
//     }
//     io::stdout().flush().expect("Failed to flush stdout");
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");
//     let command = input.trim();
//      match command {
//             input => match input.parse::<usize>() {
//             Ok(num)=> match emails.get(num - 1){
//                 Some(email) =>{
//                 println!("you have selected the account: {}",emails.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()));
//                 println!("please enter the password for the selected account:");
//                 print!("-> ");
//                 io::stdout().flush().expect("Failed to flush stdout");
//                 let mut password = String::new();
//                 io::stdin().read_line(&mut password).expect("Failed to read line");
//                 let password = password.trim();
//                 if password_check(emails.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()), password).unwrap_or(false){
//                     println!("you have dropped the account: {}",emails.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()));
//                     let email = emails.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()).to_string();
//                     drop_account_from_db(email.clone()).expect("Failed to drop account");
//                 }else{
//                     println!("Incorrect password for the account: {}",emails.get(command.parse::<usize>().unwrap_or(0) - 1).unwrap_or(&"Invalid selection".to_string()));
//                 }
//             }
//             None=>{
//                 println!("Invalid selection")
//             }
//             },
//             _=>{
//                 println!("Invalid option")
//             }
//         }
//         }
// }
