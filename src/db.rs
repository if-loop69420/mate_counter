use std::sync::LazyLock;

use sqlx::{Sqlite, SqliteConnection};

const DB_CONN: LazyLock<SqliteConnection> = LazyLock::new(|| {});

pub trait MateDatabase {
    pub fn register_user(username: String, password: String) -> Result<(), String>;
    pub fn check_login(username: String, password: String) -> Result<bool, String>;
    pub fn increase_counter(to_increase: String, to_decrease: String) -> Result<i32, String>;
    pub fn decrease_counter(to_decrease: String, to_increase: String) -> Result<i32, String>;
    pub fn send_friendship_request(from: String, to: String) -> Result<(), String>;
    pub fn accept_friendship_request(me: String, from: String) -> Result<(), String>;
    pub fn decline_friendship_request(me: String, from: String) -> REsult<(), String>;
}
