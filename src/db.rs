extern crate rusqlite;

use rusqlite::Connection;
use rusqlite::NO_PARAMS;

pub struct DataConect {
    pub conn: Connection
}

pub struct Todo {
    pub title: String,
    pub completed: bool
}

impl DataConect {
    pub fn new(db_name: &str) -> DataConect {
        let conn = Connection::open(db_name).unwrap();
        conn.execute(
            "create table if not exists todos (
                id integer primary key,
                title text not null,
                completed boolean DEFAULT false
            )",
            NO_PARAMS,
        ).unwrap();
        
        return DataConect { conn: conn };
    }

    pub fn add(&self, title: &str) -> bool {
        self.conn.execute(
            "INSERT INTO todos (title) values (?1)"
            , &[title]
        ).unwrap();

        return true
    }

    pub fn list(&self) -> Vec<Todo> {
        let mut stmt = self.conn.prepare("SELECT title, completed FROM todos").unwrap();
        let todos = stmt.query_map(NO_PARAMS, |row| 
            Ok( 
                Todo {
                    title: row.get(0).unwrap(),
                    completed: row.get(1).unwrap(),
                }
            )
        ).unwrap();

        let a: Vec<_> = todos.map(|res| res.unwrap()).collect();

        return a;
    }
}