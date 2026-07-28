use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct user {
    id: i32,
    email: String,
    name: String,
    password: String,
}

pub fn accountsdb_creation() -> Result<()> {
    let conn = Connection::open("accounts.db")?;

    conn.execute(
        "CREATE TABLE if not exists user (
            id   INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
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
        "INSERT INTO user (email, name, password) VALUES (?1, ?2, ?3)",
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
    conn.execute(
        "INSERT INTO user (email, name, password) VALUES (?1, ?2, ?3)",
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