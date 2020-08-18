use rusqlite::{params, Connection, Result};
use std::fmt;

pub enum Priority {
    Safe,
    Euclid,
    Keter,
}
impl fmt::Display for Priority{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Priority::Safe => write!(f, "Safe") ,
            Priority::Euclid => write!(f, "Euclid") ,
            Priority::Keter => write!(f, "Keter") ,
        }

    }
}

pub enum State {
    Completed,
    OnBoard,
    Discarded,
}
impl fmt::Display for State{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
                State::Completed => write!(f, "Completed") ,
                State::OnBoard => write!(f, "OnBoard") ,
                State::Discarded => write!(f, "Discarded") ,
        }

    }
}

pub struct Entry {
    pub text: String,
    pub state: State,
    pub priority: Priority,
}

pub struct Journal {
    pub entries: Vec<Entry>,
}

fn find_state(state: String) -> State {
    match &state[..] {
        "Completed" => State::Completed,
        "OnBoard" => State::OnBoard,
        "Discarded" => State::Discarded,
        _ => State::Discarded,
    }
}

fn find_priority(priority: String) -> Priority {
    match &priority[..] {
        "Safe" => Priority::Safe,
        "Euclid" => Priority::Euclid,
        "Keter" => Priority::Keter,
        _ => Priority::Safe,
    }
}
pub fn init() -> Connection {
    let connection = Connection::open("bullet.sql").unwrap();
    connection.execute(
    "create table if not exists Journal (
        id integer primary key,
        text text not null, 
        state text not null,
        priority text not null
        )",
        params![],
        ).expect("Err: Cannot create or connect to db.");

    connection
}

pub fn add_entry(conn: &Connection, msg: String, priority: String){

    conn.execute("INSERT INTO Journal (text, state, priority) VALUES (?1, ?2 ,?3)", params![msg, "OnBoard", priority])
        .expect("Error writing to DB");

}



pub fn load_journal(conn: &Connection) -> Journal {
    let mut stmt = conn
        .prepare("SELECT id, text, state, priority FROM Journal")
        .unwrap();
    let journal_iter = stmt
        .query_map(params![], |row| {
            Ok(Entry {
                text: row.get(1)?,
                state: find_state(row.get(2).unwrap_or("Completed".to_string())),
                priority: find_priority(row.get(3).unwrap_or("Safe".to_string())),
            })
        })
        .unwrap();

    let mut journal: Journal = Journal { entries: vec![] };
    for entry in journal_iter {
        journal.entries.push(entry.unwrap());
    }
    journal
}
