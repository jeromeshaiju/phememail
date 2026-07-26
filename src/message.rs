#![allow(dead_code,unused_variables,unused_imports)]

use chrono::{DateTime, Utc};
//fro now im not implementing the attatchement system
//assuming mailbox 0 is sent folder
pub struct Message {
    pub id: u32,
    pub mailbox_id: u32,
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub date: DateTime<Utc>,
    pub starred: bool,
    pub answered: bool,
    pub flagged: bool,
    pub draft: bool,
    pub size: usize,
    pub viewed: bool,
}
impl Message {
    pub fn new(
        from: String,
        to: Vec<String>,
        subject: String,
        body: String,
        cc: Vec<String>,
        bcc: Vec<String>,
        draft: bool,
    ) -> Self {
        let id = 0; // Placeholder, should be generated or assigned appropriately
        let mailbox_id = 0; 
        let date = Utc::now();
        let starred = false;
        let answered = false;
        let flagged = false;
        let attachment_count = 0;
        let size = body.len()+subject.len()+cc.join("").len()+bcc.join("").len();
        let viewed = true;
        Message {
            id,
            mailbox_id,
            from,
            to,
            subject,
            body,
            cc,
            bcc,
            date,
            starred,
            answered,
            flagged,
            draft,
            size,
            viewed,
        }
    }
}