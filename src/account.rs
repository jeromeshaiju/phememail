#![allow(dead_code,unused_variables,unused_imports)]

/// IMAP -Connects to remote servers over the internet to fetch and sync mail in real time.
/// Maildir - Stores mail locally on the user's device, allowing for offline access and management of messages.
pub enum BackendType {
    Imap,
    Maildir,
}

/// Password-Standard username and password authentication (uses SASL PLAIN or LOGIN).
/// OAuth2- Modern token-based authentication (uses SASL XOAUTH2 for Gmail).
pub enum Authmethod{
    Password,
    OAuth2,
}

pub struct Account{
    pub id: u32,
    pub email: String,
    pub name: String,
    pub backend_type: BackendType,
    pub auth_method: Authmethod
}
impl Account{
    pub fn new(
        id: u32,
        email: String,
        name: String,
        backend_type: BackendType,
        auth_method: Authmethod
    ) -> Self {
        Account {
            id,
            email,
            name,
            backend_type,
            auth_method
        }
    }
}