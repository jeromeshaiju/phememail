use std::result;

use rusqlite::{params, Connection, Result};
use crate::UI::ui::is_logged;

#[derive(Debug)]
struct user {
    id: i32,
    email: String,
    name: String,
    password: String,
}
struct current_user {
    id: i32,
    email: String,
    name: String,
}

pub fn accountsdb_creation() -> Result<()> {
    let conn = Connection::open("accounts.db")?;

    conn.execute(
        "CREATE TABLE if not exists user (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )?;
    let me = user{
        id: 0,
        email: "steven@example.com".to_string(),
        name: "Steve".to_string(),
        password: "root".to_string(),
    };
    conn.execute(
        "INSERT OR IGNORE INTO user(email, name, password) VALUES (?1, ?2, ?3)",
        (&me.email, &me.name, &me.password),
    )?;

    let mut stmt = conn.prepare("SELECT id, email, name, password FROM user")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(user {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            password: row.get(3)?,
        })
    })?;

    for user in user_iter {
        println!("Found user {:?}", user?);
    }
    Ok(())
}

pub fn adduser(email: &str, name: &str, password: &str) -> Result<()> {
    let conn = Connection::open("accounts.db")?;
    println!("Adding user: email={}, name={}, password={}", email, name, password);
    conn.execute(
        "INSERT OR IGNORE INTO user (email, name, password) VALUES (?1, ?2, ?3)",
        params![email, name, password],
    )?;
    println!("User added to database");
    Ok(())
}

pub fn userinfofomdb() -> Result<()> {
    let conn = Connection::open("accounts.db")?;


    let mut stmt = conn.prepare("SELECT id, email, name, password FROM user")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(user {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            password: row.get(3)?,
        })
    })?;

    for user in user_iter {
        
            println!("User info: {:?}", user?);
        }
    Ok(())
}

pub fn getemails()->Result<Vec<String>>{
 let conn = Connection::open("accounts.db")?;


    let mut stmt = conn.prepare("SELECT id, email, name, password FROM user")?;
    let user_iter = stmt.query_map([], |row| {
        Ok(user {
            id: row.get(0)?,
            email: row.get(1)?,
            name: row.get(2)?,
            password: row.get(3)?,
        })
    })?;
    let mut emails = Vec::new();
    for user in user_iter {
        emails.push(user?.email.to_string());
    }
    Ok(emails)
}
pub fn password_check(email: &str, password: &str) -> Result<bool> {
    let conn = Connection::open("accounts.db")?;

    let mut stmt = conn.prepare("SELECT password FROM user WHERE email = ?1")?;
    let mut rows = stmt.query(params![email])?;

    if let Some(row) = rows.next()? {
        let stored_password: String = row.get(0)?;
        Ok(stored_password == password)
    } else {
        Ok(false) // Email not found
    }
}
pub fn get_name(email: String) -> Result<String> {
    let conn = Connection::open("accounts.db")?;

    let mut stmt = conn.prepare("SELECT name FROM user WHERE email = ?1")?;
    let mut rows = stmt.query(params![email])?;
    let name: String = rows.next()?.unwrap().get(0)?;
    Ok(name)
}
pub fn current_user(email: String,name: String)-> Result<()>{
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS current_user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            name TEXT NOT NULL
        )",
        [],
    )?;
    let me = current_user{
        id: 0,
        email: email.to_string(),
        name: name.to_string(),
    };
    conn.execute(
        "INSERT OR IGNORE INTO current_user (email, name) VALUES (?1, ?2)",
        (&me.email, &me.name),
    )?;
    is_logged();
    Ok(())

}