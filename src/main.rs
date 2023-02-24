use clap::{arg, value_parser, Command};
use dotenv;
use oracle::{Connection, Error, Version};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

const USER: &str = "ORACLE_USER";
const PASSWORD: &str = "ORACLE_PASSWORD";
const ADRES: &str = "ORACLE_ADRES";

fn main() {
    let (conn, query) = init();

    let mut stmt = match conn.statement(&query).build() {
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("E! {e}");
            process::exit(4);
        }
    };

    let rows = match stmt.query(&[]) {
        Ok(rows) => rows,
        Err(e) => {
            eprintln!("E! {e}");
            process::exit(4);
        }
    };

    // Get the column names
    rows.column_info().iter().enumerate().for_each(|(i, row)| {
        if i == rows.column_info().len() - 1 {
            println!("{}", row.name());
        } else {
            print!("{},", row.name());
        }
    });

    // Print rows
    rows.into_iter().for_each(|row_result| {
        let row_result = row_result.as_ref().unwrap().sql_values();

        row_result.iter().enumerate().for_each(|(i, val)| {
            if i == row_result.len() - 1 {
                println!("{val}");
            } else {
                print!("{},", val);
            }
        });
    });
    conn.close().unwrap();
}

fn init() -> (Connection, String) {
    // Parsing arguments
    let matches = Command::new("sql_printer")
        .arg(
            arg!(
                -c --config <FILE> ".env file, which should contain ORACLE_USER, ORACLE_PASSWORD AND ORACLE_ADRES environment variables"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -f --file <FILE> "SQL query file"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let path = matches.get_one::<PathBuf>("file").unwrap();
    let query = match read_query(path) {
        Ok(query) => query,
        Err(e) => {
            eprintln!("E! [{}]: {e}", path.display());
            process::exit(1);
        }
    };

    // Connecting to DB
    let path = matches.get_one::<PathBuf>("config");
    let conn = match connect_db(path) {
        Ok(conn) => {
            println!("I! Successfully connected to DB.");
            conn
        }
        Err(e) => {
            eprintln!("E! {e}");
            process::exit(2);
        }
    };

    // Info about Oracle Client
    match Version::client() {
        Ok(version) => {
            println!("Oracle Client Version: {version}");
        }
        Err(e) => {
            eprintln!("W! Couldn't get Oracle Client Version: {e}");
        }
    }

    (conn, query)
}

fn read_query(path: &PathBuf) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn connect_db(path_env: Option<&PathBuf>) -> Result<Connection, Error> {
    if let Some(path_env) = path_env {
        env::remove_var(USER);
        env::remove_var(PASSWORD);
        env::remove_var(ADRES);
        dotenv::from_path(path_env).unwrap_or_else(|e| {
            eprintln!("E! In loading environment variables: {e}");
            process::exit(1);
        });
    }

    let get_env_var = |var_name: &str| {
        env::var(var_name).unwrap_or_else(|_| {
            eprintln!("E! \"{var_name}\" not found in environment variables");
            process::exit(1);
        })
    };

    let user = get_env_var(USER);
    let password = get_env_var(PASSWORD);
    let adres = get_env_var(ADRES);

    Connection::connect(user, password, adres)
}
