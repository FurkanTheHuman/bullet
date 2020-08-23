use rusqlite::{params, Connection, Error};
use std::fmt;
//use base64;

pub enum Priority {
    Safe,
    Euclid,
    Keter,
}

pub enum State {
    Completed,
    Active,
    Discarded,
}

// this two Displays are for so that I can transform enum's to strings
impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Priority::Safe => write!(f, "Safe"),
            Priority::Euclid => write!(f, "Euclid"),
            Priority::Keter => write!(f, "Keter"),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            State::Completed => write!(f, "Completed"),
            State::Active => write!(f, "Active"),
            State::Discarded => write!(f, "Discarded"),
        }
    }
}
// this trait is for turning strings into enums back
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
            "active" => Some(State::Active),
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

/*
    
    migrations done but now we can not revert them!!
    this is the last thing. 
    after than version one can be released.

*/

// it is ensured that vec will always be bigger that len 0. See get_last_migration_id
fn find_latest(n: Vec<u32>) -> u32{
    let mut number = 0;
    for i in n{
        if number < i{
            number = i;
        }
    }
    number
}

fn get_last_migration_id(conn: &Connection) -> Result<u32, rusqlite::Error>{
    let selection = conn.prepare("SELECT migration_count FROM metadata;");
    let mut id: Vec<u32> = Vec::new();
    match selection {
        Ok(mut rows) => {
            let mut q = rows.query(params![]).unwrap();
            while let Some(n) =q.next().unwrap() {
                id.push(n.get(0).unwrap());
            }
            if id.len() < 1{
                Err(Error::QueryReturnedNoRows)
            } else {
                Ok(find_latest(id))
            }
        },
        Err(n) => panic!(format!("PANIC: Unresolved DB problems! -- {}", n))

    }

}

fn prepare_migrations(conn: &Connection) -> u32{
    conn.execute("INSERT INTO metadata (migration_count) VALUES (?1)", params![0]).unwrap();
    0
}

pub fn init_connection() -> (Connection, u32) {
    let connection = Connection::open("/home/foucault/.config/bullet.sql").unwrap();
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS journal (
        id INTEGER PRIMARY KEY,
        text TEXT NOT NULL, 
        state TEXT NOT NULL,
        priority TEXT NOT NULL,
        migration INTEGER NOT NULL,
        migrated_at DEFAULT CURRENT_TIMESTAMP
        )",
            params![],
        )
        .expect("Err: Cannot create or connect to db.");

        //contains last migration date, migration count
        connection
        .execute(
            "CREATE TABLE IF NOT EXISTS metadata (
        id INTEGER PRIMARY KEY,
        migration_date DEFAULT CURRENT_TIMESTAMP,
        migration_count INTEGER NOT NULL
        )",
            params![],
        )
        .expect("Err: Cannot create or connect to db.");

        let id = get_last_migration_id(&connection);
        let id = match id {
            Ok(n) => n,
            Err(_) => prepare_migrations(&connection)
        };

        (connection, id)
}




pub fn get_header_contents(conn: &Connection, migration_id: u32) -> (u32, u32) {
    let mut x = conn.prepare("SELECT priority, migration FROM Journal").unwrap();
    let mut rows = x.query(params![]).unwrap();

    let mut names: Vec<(String, u32)> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        names.push((row.get(0).unwrap(),row.get(1).unwrap()));
    }

    let mut general_count = 0;
    let mut prior_count = 0;

    for (p, id) in names {
        if id == migration_id {
            general_count = general_count + 1;
        }
        if p.to_lowercase().trim() == "keter" && id == migration_id{
            prior_count = prior_count + 1;
        }
    }
    (general_count, prior_count)
}

fn is_okay_for_new_migration(entry: &Entry) -> bool{
    //but when run migrate based on safe, discarded and completed
    if entry.priority.to_string().to_lowercase() == "safe"{
        return false
    } 
    if entry.state.to_string().to_lowercase() == "completed" || entry.state.to_string().to_lowercase() == "discarded"{
        return false
    } 
    true    
}

pub fn migrate(conn: &Connection, id: u32) -> u32{
    let current = load_journal(conn, id);
    let mut counter = 0;
    conn.execute("INSERT INTO metadata (migration_count) VALUES (?1)", params![id+1]).unwrap();
    for (primary_id, entry) in &current{
        if is_okay_for_new_migration(entry){
            conn.execute("UPDATE Journal SET migration=(?1) WHERE id=(?2);", params![id+1, primary_id]).unwrap();
            counter = counter +1;
        }
    }
    println!("Migrated {} elements", counter);
    id+1
}

pub fn add_entry(conn: &Connection, msg: String, priority: String, id: u32) -> bool {
    if let Some(_n) = priority.convert_to_priority() {
        conn.execute(
            "INSERT INTO Journal (text, state, priority, migration) VALUES (?1, ?2 ,?3, ?4)",
            params![msg, "active", priority, id],
        )
        .expect("Error writing to DB");
        return true;
    }
    println!("Priority name is not valid. Valid values are {}, {}, {}", Priority::Safe, Priority::Euclid,Priority::Keter);
    false
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

pub fn print_all(conn: &Connection) -> Vec<(u32, Entry)> {
    let mut stmt = conn.prepare("SELECT id, text, state, priority FROM Journal ORDER BY id DESC").unwrap();
    let mut rows = stmt.query(params![]).unwrap();

    let mut names: Vec<(u32, Entry)> = Vec::new();
    while let Some(row) = rows.next().unwrap(){
        names.push((row.get(0).unwrap(), 
            Entry{
                text: row.get(1).unwrap(), 
                state: match row.get(2).unwrap_or("Err".to_string()).convert_to_state(){
                    Some(n) => n,
                    None => panic!("DB Error")
                },
                priority: match row.get(3).unwrap_or("Err".to_string()).convert_to_priority(){
                    Some(n) => n,
                    None => panic!("DB Error")
                }
        }));

    }
    for (_, i) in &names{
        println!("VEC {:?}", i.text);

    }
    names

}

pub fn load_journal(conn: &Connection, migration_id: u32) -> Vec<(u32, Entry)> {
    let mut stmt = conn
        .prepare(&format!("SELECT id, text, state, priority FROM Journal WHERE migration={}", migration_id)[..])
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

    let mut journal =  vec![];
    for entry in journal_iter {
        journal.push(entry.unwrap());
    }
    journal
}
