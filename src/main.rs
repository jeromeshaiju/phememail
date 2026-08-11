#![allow(dead_code,unused_variables,unused_imports)]
mod message;
mod UI;
mod storage;
fn main(){
  storage::accountsdbms::accountsdb_creation().expect("Failed to create accounts database");
  storage::mailboxdbms::mailboxdb_creation().expect("Failed to create mailbox database");

  UI::ui::main_interface();
}