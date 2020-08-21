use rusqlite::{params, Connection, Error};
use std::fmt;

pub enum Priority {
    Safe,
    Euclid,
    Keter,
}
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Priority::Safe => write!(f, "Safe"),
            Priority::Euclid => write!(f, "Euclid"),
            Priority::Keter => write!(f, "Keter"),
        }
    }
}

pub enum State {
    Completed,
    OnBoard,
    Discarded,
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            State::Completed => write!(f, "Completed"),
            State::OnBoard => write!(f, "OnBoard"),
            State::Discarded => write!(f, "Discarded"),
        }
    }
}

pub trait ConvertEnum {
    type Item;
    type Item2;
    fn convert_to_state(&self) -> Option<Self::Item>;
    fn convert_to_priority(&self) -> Option<Self::Item2>;
}

impl ConvertEnum for String {
    type Item = State;
    fn convert_to_state(&self) -> Option<Self::Item> {
        match &self[..].to_lowercase().trim()[..] {
            "completed" => Some(State::Completed),
            "onboard" => Some(State::OnBoard),
            "discarded" => Some(State::Discarded),
            _ => None,
        }
    }
    type Item2 = Priority;
    fn convert_to_priority(&self) -> Option<Self::Item2> {
        match &self[..].to_lowercase().trim()[..] {
            "safe" => Some(Priority::Safe),
            "euclid" => Some(Priority::Euclid),
            "keter" => Some(Priority::Keter),
            _ => None,
        }
    }
}
pub struct Entry {
    pub text: String,
    pub state: State,
    pub priority: Priority,
}

pub struct Journal {
    pub entries: Vec<(i32, Entry)>,
}

pub fn init() -> Connection {
    let connection = Connection::open("bullet.sql").unwrap();
    connection
        .execute(
            "create table if not exists Journal (
        id integer primary key,
        text text not null, 
        state text not null,
        priority text not null
        )",
            params![],
        )
        .expect("Err: Cannot create or connect to db.");

    connection
}

pub fn metadata(conn: &Connection) -> (u32, u32) {
    /*    struct Tmp {
        name :String
    }
    let mut q = conn.prepare("SELECT priority FROM Journal").unwrap();
    //MappedRows<|&Row| -> Result<String, Error>>
    let p_iter = q.query_map(params![], |row| {
        Ok(Tmp{
            name : row.get(0).unwrap()
        })
    }).unwrap();
    */
    // TODO: This is a better aproach
    let mut x = conn.prepare("SELECT priority FROM Journal").unwrap();
    let mut rows = x.query(params![]).unwrap();

    let mut names: Vec<String> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        names.push(row.get(0).unwrap());
    }

    let mut general_count = 0;
    let mut prior_count = 0;

    for c in names {
        general_count = general_count + 1;
        if c.to_lowercase().trim() == "keter" {
            prior_count = prior_count + 1;
        }
    }
    (general_count, prior_count)
}

pub fn add_entry(conn: &Connection, msg: String, priority: String) {
    if let Some(_n) = priority.convert_to_priority() {
        conn.execute(
            "INSERT INTO Journal (text, state, priority) VALUES (?1, ?2 ,?3)",
            params![msg, "OnBoard", priority],
        )
        .expect("Error writing to DB");
        return;
    }

    panic!("FUCK");
}

pub fn delete_entry(conn: &Connection, proc_id: u32) -> bool {
    let query = format!("DELETE FROM Journal WHERE id = {};", proc_id);
    match conn.execute(&query[..], params![]) {
        Ok(n) => n > 0,
        Err(_) => panic!("Fatal Error: query failed"),
    }
}

pub fn change_state(conn: &Connection, state: State, proc_id: u32) {
    let query = format!(
        "UPDATE Journal SET state='{}' WHERE id={};",
        state.to_string().to_lowercase().trim(),
        proc_id
    );
    conn.execute(&query[..], params![]).unwrap();
}

pub fn load_journal(conn: &Connection) -> Journal {
    let mut stmt = conn
        .prepare("SELECT id, text, state, priority FROM Journal")
        .unwrap();
    let journal_iter = stmt
        .query_map(params![], |row| {
            Ok((
                row.get(0)?,
                Entry {
                    text: row.get(1)?,
                    state: match row
                        .get(2)
                        .unwrap_or("Completed".to_string())
                        .convert_to_state()
                    {
                        Some(n) => n,
                        None => panic!("Not a valid state. DB corrupted!"),
                    },
                    priority: match row
                        .get(3)
                        .unwrap_or("Safe".to_string())
                        .convert_to_priority()
                    {
                        Some(n) => n,
                        None => panic!("Not a valid priority. DB corrupted!"),
                    },
                },
            ))
        })
        .unwrap();

    let mut journal: Journal = Journal { entries: vec![] };
    for entry in journal_iter {
        journal.entries.push(entry.unwrap());
    }
    journal
}
