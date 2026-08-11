use std::result;

use rusqlite::{params, Connection, Result};
use crate::UI::ui::is_logged;

pub struct CurrentMailbox {
    id: i32,
    email: String,
    name: String,
}
impl CurrentMailbox{
    pub fn new(id: i32, email: String, name: String) -> Self {
        CurrentMailbox { id, email, name }
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub static mut  CURRENT_MAILBOX: Option<CurrentMailbox> = None;

#[derive(Debug)]
struct mailbox {
    id: i32,
    email: String,
    name: String,
    count: usize,
    unseen: u32,
}

pub fn mailboxdb_creation() -> Result<()> {
    let conn = Connection::open("mailbox.db")?;

    conn.execute(
        "CREATE TABLE if not exists mailbox (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            name TEXT NOT NULL,
            count INTEGER NOT NULL,
            unseen INTEGER NOT NULL,
            UNIQUE(email, name)
        )",
        (), // empty list of parameters.
    )?;
    let me = mailbox{
        id: 0,
        email: "steven@example.com".to_string(),
        name: "inbox".to_string(),
        count: 0,
        unseen: 0,
    };
    conn.execute(
        "INSERT OR IGNORE INTO mailbox(email, name, count, unseen) VALUES (?1, ?2, ?3, ?4)",
        (&me.email, &me.name, &me.count, &me.unseen),
    )?;

    let mut stmt = conn.prepare("SELECT id, email, name, count, unseen FROM mailbox")?;
    let mailbox_iter = stmt.query_map([], |row| {
        Ok(mailbox {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            count: row.get(3)?,
            unseen: row.get(4)?,
        })
    })?;

    for mailbox in mailbox_iter {
        println!("Found mailbox {:?}", mailbox?);
    }
    Ok(())
}

pub fn addmailbox(email: &str, name: &str) -> Result<()> {
    let conn = Connection::open("mailbox.db")?;
    println!("Adding mailbox: email={}, name={}", email, name);
    conn.execute(
        "INSERT OR IGNORE INTO mailbox (email, name, count, unseen) VALUES (?1, ?2, ?3, ?4)",
        params![email, name, 0, 0],
    )?;
    println!("Mailbox added to database");
    Ok(())
}

pub fn mailboxinfofromdb() -> Result<()> {
    let conn = Connection::open("mailbox.db")?;


    let mut stmt = conn.prepare("SELECT id, email, name,count,unseen FROM mailbox")?;
    let mailbox_iter = stmt.query_map([], |row| {
        Ok(mailbox {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            count: row.get(3)?,
            unseen: row.get(4)?,
        })
    })?;

    for mailbox in mailbox_iter {
        
            println!("Mailbox info: {:?}", mailbox?);
        }
    Ok(())
}

pub fn getmailboxes(given_email: &str)->Result<Vec<String>> {
 let conn = Connection::open("mailbox.db")?;


    let mut stmt = conn.prepare("SELECT id, email, name, count, unseen FROM mailbox where email = ?1")?;
    let mailbox_iter = stmt.query_map([given_email], |row| {
        Ok(mailbox {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            count: row.get(3)?,
            unseen: row.get(4)?,
        })
    })?;
    let mut mailboxes = Vec::new();
    for mailbox in mailbox_iter {
        let m = mailbox?;
        let name = m.name.to_string();
        let email = m.email.to_string();
        if (email == given_email.to_string()){
        mailboxes.push(name);
        }
    }
    Ok(mailboxes)
}
pub fn get_email(name: String) -> Result<String> {
    let conn = Connection::open("mailbox.db")?;

    let mut stmt = conn.prepare("SELECT email FROM user WHERE name = ?1")?;
    let mut rows = stmt.query(params![name])?;
    let email: String = rows.next()?.unwrap().get(0)?;
    Ok(email)
}

pub fn current_mailbox(email: String,name: String){
    is_logged();
    let current = CurrentMailbox::new(0,email,name);
    unsafe {
        CURRENT_MAILBOX = Some(current);
    }
}
pub fn current_mailbox_name() ->Option<String> {
    unsafe {
        let ptr = std::ptr::addr_of!(CURRENT_MAILBOX);
        (*ptr).as_ref().map(|u| u.get_name().to_string())
    }
}

pub fn drop_mailbox_from_db(email: String,name: String) -> Result<()> {
    let conn = Connection::open("mailbox.db")?;
    conn.execute(
        "DELETE FROM mailbox WHERE email = ?1 AND name = ?2",
        params![email, name],
    )?;
    println!("Mailbox with email {} has been dropped.", email);
    Ok(())
}