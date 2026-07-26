#![allow(dead_code,unused_variables,unused_imports)]

pub struct Mailbox {
    pub mailbox_id: u32,
    pub name: String,
    pub email: String,
    pub count: usize,
    pub unseen: u32,
    pub path: String,
}

impl Mailbox {
    pub fn new(
     mailbox_id: u32,
     name: String,
     email: String,
     count: usize,
     unseen: u32,
     path: String,
    ) -> Self {
        let mailbox_id = 0; // Placeholder, should be assigned based on the mailbox 
        let count: usize = 0;
        let answered = false;
        let unseen: u32 = 0;
        let path: String = String::from("placeholder");
        Mailbox {
            mailbox_id,
            name,
            email,
            count,
            unseen,
            path,
        }
    }
}