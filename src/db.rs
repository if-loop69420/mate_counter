use std::sync::LazyLock;

use sha2::{Digest, Sha512};
use sqlx::{Connection, Sqlite, SqliteConnection};

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
    async fn register_user(&mut self, username: String, password: String) -> Result<(), String>;
    async fn check_login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<Option<u32>, String>;
    async fn increase_counter(
        &mut self,
        to_increase: String,
        to_decrease: String,
    ) -> Result<i32, String>;
    async fn decrease_counter(
        &mut self,
        to_decrease: String,
        to_increase: String,
    ) -> Result<i32, String>;
    async fn send_friendship_request(&mut self, from: String, to: String) -> Result<(), String>;
    async fn accept_friendship_request(&mut self, me: String, from: String) -> Result<(), String>;
    async fn decline_friendship_request(&mut self, me: String, from: String) -> Result<(), String>;
}

impl MateDatabase for SqliteConnection {
    async fn register_user(&mut self, username: String, password: String) -> Result<(), String> {
        let mut password_digest = Sha512::new();
        password_digest.update(password);
        let hashed_pw = password_digest.finalize();
        let hashed_pw_slice = hashed_pw.as_slice();

        match sqlx::query!(
            "INSERT INTO user (name, password) VALUES ($1,$2)",
            username,
            hashed_pw_slice
        )
        .execute(self)
        .await
        {
            Ok(_x) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
    async fn check_login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<Option<u32>, String> {
        let mut password_digest = Sha512::new();
        password_digest.update(password);
        let hashed_pw = password_digest.finalize();
        let hashed_pw_slice = hashed_pw.as_slice();

        match sqlx::query_as::<Sqlite, (u32, String)>("SELECT id, password FROM user where name=$1")
            .bind(username)
            .fetch_one(self)
            .await
        {
            Ok((id, password_hash)) => {
                if hashed_pw_slice == password_hash.as_bytes() {
                    Ok(Some(id))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
    async fn increase_counter(
        &mut self,
        to_increase: String,
        to_decrease: String,
    ) -> Result<i32, String> {
        todo!()
    }

    async fn decrease_counter(
        &mut self,
        to_decrease: String,
        to_increase: String,
    ) -> Result<i32, String> {
        todo!()
    }

    async fn send_friendship_request(&mut self, from: String, to: String) -> Result<(), String> {
        todo!()
    }

    async fn accept_friendship_request(&mut self, me: String, from: String) -> Result<(), String> {
        todo!()
    }

    async fn decline_friendship_request(&mut self, me: String, from: String) -> Result<(), String> {
        todo!()
    }
}
