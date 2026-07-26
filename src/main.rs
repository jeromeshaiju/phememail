#![allow(dead_code,unused_variables,unused_imports)]
mod message;
mod mailbox;
mod account;
mod UI;

fn main(){
  UI::ui::main_interface();
}