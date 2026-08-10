use std::result;

use rusqlite::{params, Connection, Result};
use crate::UI::ui::is_logged;

#[derive(Debug)]
struct mailbox {
    id: i32,
    email: String,
    name: String,
    count: usize,
    unseen: u32,
}

struct current_mailbox {
    email: String,
    name: String,
}

pub fn mailboxdb_creation() -> Result<()> {
    let conn = Connection::open("mailbox.db")?;

    conn.execute(
        "CREATE TABLE if not exists mailbox (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            count INTEGER NOT NULL,
            unseen INTEGER NOT NULL
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

pub fn current_mailbox(name: String)-> Result<()>{
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS current_user (
            email TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL
        )",
        [],
    )?;
    let me = current_mailbox{
        email: String::new(),
        name: name.to_string(),
    };
    conn.execute(
        "INSERT OR IGNORE INTO current_user (email, name) VALUES (?1, ?2)",
        (&me.email, &me.name),
    )?;
    is_logged();
    Ok(())

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