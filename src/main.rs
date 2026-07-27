#![allow(dead_code,unused_variables,unused_imports)]
mod message;
mod mailbox;
mod account;
mod UI;
mod storage;
fn main(){
  storage::accountsdbms::accountsdb_creation().expect("Failed to create accounts database");
  UI::ui::main_interface();
}