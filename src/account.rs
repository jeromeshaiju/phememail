#![allow(dead_code,unused_variables,unused_imports)]



pub struct CurrentUser {
    id: i32,
    email: String,
    name: String,
}
impl CurrentUser{
    pub fn new(id: i32, email: String, name: String) -> Self {
        CurrentUser { id, email, name }
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

//has not implemented the authmethod and backend type yet, so they are commented out for now


// /// IMAP -Connects to remote servers over the internet to fetch and sync mail in real time.
// /// Maildir - Stores mail locally on the user's device, allowing for offline access and management of messages.
// pub enum BackendType {
//     Imap,
//     Maildir,
// }

// /// Password-Standard username and password authentication (uses SASL PLAIN or LOGIN).
// /// OAuth2- Modern token-based authentication (uses SASL XOAUTH2 for Gmail).
// pub enum Authmethod{
//     Password,
//     OAuth2,
// }

pub struct Account{
    pub id: u32,
    pub email: String,
    pub password: String,
    pub name: String,
    // pub backend_type: BackendType,
    // pub auth_method: Authmethod
}
impl Account{
    pub fn new(
        email: String,
        password: String,
        name: String,
        // backend_type: BackendType,
        // auth_method: Authmethod
    ) -> Self {
        let id = 0; // Placeholder, should be generated or assigned appropriately
        Account {
            id,
            email,
            password,
            name,
            // backend_type,
            // auth_method
        }
    }
}
