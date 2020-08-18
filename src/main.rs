use colored::*;
use rusqlite::{params, Connection, Result};
use std::env;

mod storage;
/*
#[derive(Debug)]
struct Entry {
    text: String,
    state: String
}*/

/*
    bullet --head  -> displays shell header
    bullet list -> list current bullets (with priority and state)
    bullet add -m "Kill the old gods" -p keter -> add new entry state is by default OnBoard
    bullet delete 5 -> kill entry with procedure id 5
    bullet discard 5 -> discard entry with procedure id 5
    bullet update 5 completed -> update the procedure 5 to complete
    bullet migrate -> remove the completed, discarded and safe entries. Start the new circle


*/
/*
enum Operations {
    list,
    add,
    delete,
    discard,
    update,
    migrate,
}
*/

fn list_bullets(conn: &Connection) {
    let journal: storage::Journal = storage::load_journal(conn);
    for entry in journal.entries{
        let msg= entry.text.blue();
        let priority = entry.priority.to_string().yellow();
        let state= entry.state.to_string().bright_green();
        println!("[{}]/[{}] - {}", state, priority, msg);
    } 
}
fn add_bullet(conn: &Connection, args: &[String]){
    if args[1].eq("-m") && args[3].eq("-p") {
        storage::add_entry(&conn, args[2].to_string(), args[4].to_string());
        return 
    } 
    panic!("Args fucked up");
}
fn delete_bullet(){}
fn discard_bullet(){}
fn update_bullet(){}
fn migrate(){}



fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    let conn = storage::init();

    match &args[1][..] {
        "list" => list_bullets(&conn),
        "add" => add_bullet(&conn, &args[1..]),
        /*"delete" => ,
        "discard" => ,
        "update" => ,
        "migrate" => ,*/
        n => panic!("Argument {} is not reconized",n)
    }



    /* let entry =Entry{
        text: "Helslo".to_string(),
        state: "1".to_string()
    };


    //let conn = Connection::open("cats.db")?;
    let conn = Connection::open_in_memory()?;

    match init(&conn) {
        Some(n) => println!("{} {} {}", "Connection to DB established. Read".green().bold(), n , "bytes".green().bold()),
        None => panic!("Disconnected!") // This might be updated for e detailed raport
    }

    conn.execute("INSERT INTO Journal (text, state) VALUES (?1, ?2)", params!["entry.text", "2"])?;
    conn.execute("INSERT INTO Journal (text, state) VALUES (?1, ?2)", params!["entry.text", "2"])?;
    conn.execute("INSERT INTO Journal (text, state) VALUES (?1, ?2)", params!["entry.text", "2"])?;
    conn.execute("INSERT INTO Journal (text, state) VALUES (?1, ?2)", params!["entry.text", "2"])?;
    let num = conn.execute("INSERT INTO Journal (text, state) VALUES (?1, ?2)", params![entry.text, entry.state])?;
    println!("{}", num);

    let mut stmt = conn.prepare("SELECT id, text, state FROM Journal")?;

    let journal_iter = stmt.query_map(params![], |row| {
        Ok(Entry {
            text: row.get(1)?,
            state: row.get(2)?,
        })
    })?;

    if args[1] == "--head" {
        let bullets = "💎 [12 bullets]".bright_blue();
        let keter = "🔥 4 is keter".yellow();
        println!(r#"{} - {}"#, bullets ,keter);
    }


    for e in journal_iter {
        println!("Found entry {:?}", e.unwrap());
    }*/
    Ok(())
}
