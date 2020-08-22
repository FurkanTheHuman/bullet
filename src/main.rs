use colored::*;
use rusqlite::{Connection, Result};
use std::process::exit;
//use std::env;

use clap::{App, Arg};
//use crate::storage::ConvertEnum;

mod storage;
/*
#[derive(Debug)]
struct Entry {
    text: String,
    state: String
}*/

/*
    * bullet --head  -> displays shell header
    * bullet list -> list current bullets (with priority and state)
    * bullet add -m "Kill the old gods" -p keter -> add new entry state is by default OnBoard
    * bullet delete 5 -> kill entry with procedure id 5
    * bullet discard 5 -> discard entry with procedure id 5
    * bullet complete 5
    * bullet onboard 5
    * bullet priority 5 euclid
    bullet migrate -> remove the completed, discarded and safe entries. Start the new circle


    NOTE: Might change "onboard" with "active"

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

fn list_bullets(conn: &Connection, id: u32) {
    let journal = storage::load_journal(conn, id);
    for (id, entry) in journal {
        let priority = entry.priority.to_string().yellow();
        let state;
        if entry.state.to_string().to_lowercase() == "discarded" {
            state = entry.state.to_string().truecolor(128, 128, 128);
        } else {
            state = entry.state.to_string().bright_green();
        }
        let msg;
        if entry.priority.to_string().to_lowercase() == "keter" {
            msg = entry.text.bright_red();
        } else {
            msg = entry.text.bright_blue();
        }
        println!("PROC-{} [{}]/[{}] \n\t {}\n", id, state, priority, msg);
    }
}

fn add_bullet(conn: &Connection, msg: &str, priority: &str, id:u32) {
    storage::add_entry(&conn, msg.to_string(), priority.to_string(), id);
}

fn delete_bullet(conn: &Connection, proc_id: u32) {
    if storage::delete_entry(conn, proc_id) {
        println!("Deleted succesfuly.");
    } else {
        println!("Entry not found!");
    }
}

/*
fn migrate(){}
*/

fn main() -> Result<()> {
    // let args: Vec<String> = env::args().collect();
    let (conn, mut migration_id) = storage::init_connection();
    let matches = App::new("Bullet")
        .version("1.0")
        .author("Furkan A. <aksoyfurkan45@gmail.com>")
        .about("Simple bullet list")
        .arg( Arg::new("head").about("Prints console header").takes_value(false).long("--head"))
        .subcommand(
            App::new("list")
                    .about("List current bullets"))
        .subcommand(App::new("add")
                .about("Add new bullet to the list")
                .arg(Arg::with_name("text")
                    .short('t')
                    .takes_value(true)
                    .about("Takes Bullet entry").required(true))
                .arg(Arg::with_name("priority")
                    .short('p')
                    .takes_value(true)
                    .about("Takes Bullet priority")))       
        .subcommand(App::new("delete")
                .about("Delete bullet from the list")
                .arg(Arg::new("id")
                    .takes_value(true)
                    .about("Id of bullet to delete").required(true)))
        .subcommand(App::new("discard")
                .about("Discard bullet from the list")
                .arg(Arg::new("id")
                    .takes_value(true)
                    .about("Id of bullet to discard").required(true)))  
        .subcommand(App::new("complete")
                .about("Complete bullet from the list")
                .arg(Arg::new("id")
                    .takes_value(true)
                    .about("Id of bullet to complete").required(true)))  
        .subcommand(App::new("onboard")
                .about("Activate bullet from the list")
                .arg(Arg::new("id")
                    .takes_value(true)
                    .about("Id of bullet to activate").required(true)))  
        .subcommand(App::new("migrate").about("Start new spring"))    
        .get_matches();

    if matches.is_present("head") {
        let (normal, keter) = storage::get_header_contents(&conn, migration_id);
        let bullets = format!("💎 [{} bullets]", normal).bright_blue();
        let keter = format!("🔥 {} is keter", keter).yellow();
        println!(r#"{} - {}"#, bullets, keter);
        exit(0);
    }

    match matches.subcommand() {
        ("list", Some(_)) => list_bullets(&conn, migration_id),
        ("add", Some(add_matches)) => add_bullet(
            &conn,
            add_matches.value_of("text").unwrap(),
            add_matches.value_of("priority").unwrap_or("Euclid"),
            migration_id
        ),
        ("delete", Some(delete_matches)) => delete_bullet(
            &conn,
            delete_matches
                .value_of("id")
                .unwrap()
                .to_string()
                .parse::<u32>()
                .expect("Err: Not a valid number"),
        ),
        ("discard", Some(id)) => {
            storage::change_state(
                &conn,
                storage::State::Discarded,
                id.value_of("id").unwrap().parse::<u32>().unwrap(),
            );
            println!("Discarded {}", id.value_of("id").unwrap())
        }
        ("complete", Some(id)) => {
            storage::change_state(
                &conn,
                storage::State::Completed,
                id.value_of("id").unwrap().parse::<u32>().unwrap(),
            );
            println!("Completed {}", id.value_of("id").unwrap())
        }
        ("onboard", Some(id)) => {
            storage::change_state(
                &conn,
                storage::State::OnBoard,
                id.value_of("id").unwrap().parse::<u32>().unwrap(),
            );
            println!("Activated {}", id.value_of("id").unwrap())
        }
        ("update", Some(_update_matches)) => println!("Updated"),
        ("migrate", Some(_migrate_matches)) => {
            migration_id = storage::migrate(&conn, migration_id);
            println!("Migration {} active", migration_id);
        },

        (t, _) => println!("None {}::", t), /*"discard" => ,
                                            "update" => ,
                                            "migrate" => ,*/
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
