use std::sync::LazyLock;

use sqlx::{Connection, SqliteConnection};

const DATABASE_URL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("MATE_DB_URL").expect("Couldn't read MATE_DB_URL environment variable")
});

const DB_CONN: LazyLock<SqliteConnection> = LazyLock::new(|| {
    let path = DATABASE_URL;
    let mut db = futures::executor::block_on(SqliteConnection::connect(&path))
        .expect("Couldn't open Database");
    futures::executor::block_on(sqlx::migrate!().run(&mut db))
        .expect("Couldn't run migrations on database");
    db
});

pub trait MateDatabase {
    async fn register_user(&self, username: String, password: String) -> Result<(), String>;
    async fn check_login(&self, username: String, password: String) -> Result<bool, String>;
    async fn increase_counter(
        &self,
        to_increase: String,
        to_decrease: String,
    ) -> Result<i32, String>;
    async fn decrease_counter(
        &self,
        to_decrease: String,
        to_increase: String,
    ) -> Result<i32, String>;
    async fn send_friendship_request(&self, from: String, to: String) -> Result<(), String>;
    async fn accept_friendship_request(&self, me: String, from: String) -> Result<(), String>;
    async fn decline_friendship_request(&self, me: String, from: String) -> Result<(), String>;
}

impl MateDatabase for SqliteConnection {
    async fn register_user(&self, username: String, password: String) -> Result<(), String> {
        todo!()
    }
    async fn check_login(&self, username: String, password: String) -> Result<bool, String> {
        todo!()
    }
    async fn increase_counter(
        &self,
        to_increase: String,
        to_decrease: String,
    ) -> Result<i32, String> {
        todo!()
    }

    async fn decrease_counter(
        &self,
        to_decrease: String,
        to_increase: String,
    ) -> Result<i32, String> {
        todo!()
    }

    async fn send_friendship_request(&self, from: String, to: String) -> Result<(), String> {
        todo!()
    }

    async fn accept_friendship_request(&self, me: String, from: String) -> Result<(), String> {
        todo!()
    }

    async fn decline_friendship_request(&self, me: String, from: String) -> Result<(), String> {
        todo!()
    }
}
