use std::{path::PathBuf, sync::Mutex};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub eid: Option<i64>,
    date_created: i64,
    content: String,
    mood: i8,
}

pub struct EntryDao {
    conn: Mutex<Connection>,
}

impl EntryDao {
    pub fn new(db_path: PathBuf) -> Self {
        let conn = Mutex::new(Connection::open(db_path).expect("Failed to connect to database"));
        conn.lock()
            .unwrap()
            .execute(
                "CREATE TABLE IF NOT EXISTS entries (
                eid INTEGER PRIMARY KEY,
                date_created INTEGER NOT NULL,
                content TEXT NOT NULL,
                mood INTEGER NOT NULL
            )",
                (), // empty list of parameters.
            )
            .expect("Failed to create entry table");
        Self { conn }
    }

    pub fn get_all(&self) -> Vec<Entry> {
        // SELECT * FROM entry ORDER BY date_created DESC
        let connection = self.conn.lock().unwrap();
        let mut stmt = connection
            .prepare("SELECT * FROM entries")
            .expect("Error selecting all entries");
        stmt.query_map([], |row| {
            Ok(Entry {
                eid: row.get(0).expect("failed parsing eid"),
                date_created: row.get(1).expect("failed parsing date"),
                content: row.get(2).expect("failed parsing content"),
                mood: row.get(3).expect("failed parsing mood"),
            })
        })
        .expect("Failed creating entries list")
        .map(|entry_result| entry_result.unwrap())
        .collect::<Vec<Entry>>()
    }

    pub fn get_entry_by_id(&self, id: i64) -> Option<Entry> {
        // SELECT * FROM entry WHERE eid = :id LIMIT 1
        let connection = self.conn.lock().unwrap();
        let mut stmt = connection
            .prepare("SELECT * FROM entries WHERE eid = ?1 LIMIT 1")
            .expect("Error selecting entry");
        stmt.query_map([id], |row| {
            Ok(Entry {
                eid: row.get(0).expect("failed parsing eid"),
                date_created: row.get(1).expect("failed parsing date"),
                content: row.get(2).expect("failed parsing content"),
                mood: row.get(3).expect("failed parsing mood"),
            })
        })
        .expect("Failed creating entries list")
        .map(|entry_result| entry_result.unwrap())
        .collect::<Vec<Entry>>()
        .first()
        .cloned()
    }

    pub fn delete_entry(&self, entry: Entry) {
        self.conn
            .lock()
            .unwrap()
            .execute("DELETE FROM entries WHERE eid = ?1", [&entry.eid.unwrap()])
            .expect("Error deleting entry");
    }

    pub fn insert_entry(&self, entry: Entry) {
        self.conn.lock().unwrap().execute(
            "INSERT OR REPLACE INTO entries (eid, date_created, content, mood) VALUES (?1, ?2, ?3, ?4)",
            (&entry.eid, &entry.date_created, &entry.content, &entry.mood),
        ).expect("Error inserting entry");
    }

    pub fn insert_entry_no_id(&self, entry: Entry) {
        self.conn
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO entries (date_created, content, mood) VALUES (?1, ?2, ?3)",
                (&entry.date_created, &entry.content, &entry.mood),
            )
            .expect("Error inserting entry");
    }

    pub fn load_by_month_and_year(
        &self,
        start_month_in_secs: i64,
        end_month_in_secs: i64,
    ) -> Vec<Entry> {
        // SELECT * FROM entry WHERE date_created >= :testMonthSecs AND date_created < :nextMonthSecs ORDER BY date_created DESC
        let connection = self.conn.lock().unwrap();
        let mut stmt = connection
            .prepare("SELECT * FROM entries WHERE date_created >= ?1 AND date_created < ?2 ORDER BY date_created DESC")
            .expect("Error selecting entry range");
        stmt.query_map([start_month_in_secs, end_month_in_secs], |row| {
            Ok(Entry {
                eid: row.get(0).expect("failed parsing eid"),
                date_created: row.get(1).expect("failed parsing date"),
                content: row.get(2).expect("failed parsing content"),
                mood: row.get(3).expect("failed parsing mood"),
            })
        })
        .expect("Failed creating entries list")
        .map(|entry_result| entry_result.unwrap())
        .collect::<Vec<Entry>>()
    }

    pub fn search_ascending(&self, search_string: String) -> Vec<Entry> {
        // SELECT * FROM entry WHERE content LIKE :text ORDER BY date_created ASC
        let connection = self.conn.lock().unwrap();
        let mut stmt = connection
            .prepare("SELECT * FROM entries WHERE content LIKE ?1 ORDER BY date_created ASC")
            .expect("Error selecting all entries");
        stmt.query_map([format!("%{}%", search_string)], |row| {
            Ok(Entry {
                eid: row.get(0).expect("failed parsing eid"),
                date_created: row.get(1).expect("failed parsing date"),
                content: row.get(2).expect("failed parsing content"),
                mood: row.get(3).expect("failed parsing mood"),
            })
        })
        .expect("Failed creating entries list")
        .map(|entry_result| entry_result.unwrap())
        .collect::<Vec<Entry>>()
    }

    pub fn search_descending(&self, search_string: String) -> Vec<Entry> {
        // SELECT * FROM entry WHERE content LIKE :text ORDER BY date_created DESC
        let connection = self.conn.lock().unwrap();
        let mut stmt = connection
            .prepare("SELECT * FROM entries WHERE content LIKE ?1 ORDER BY date_created DESC")
            .expect("Error selecting all entries");
        stmt.query_map([format!("%{}%", search_string)], |row| {
            Ok(Entry {
                eid: row.get(0).expect("failed parsing eid"),
                date_created: row.get(1).expect("failed parsing date"),
                content: row.get(2).expect("failed parsing content"),
                mood: row.get(3).expect("failed parsing mood"),
            })
        })
        .expect("Failed creating entries list")
        .map(|entry_result| entry_result.unwrap())
        .collect::<Vec<Entry>>()
    }
}
