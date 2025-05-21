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
        to_increase: u32,
        to_decrease: String,
    ) -> Result<i32, String>;
    async fn decrease_counter(
        &mut self,
        to_decrease: u32,
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
        to_increase: u32,
        to_decrease: String,
    ) -> Result<i32, String> {
        let decrease_id: u32 = match sqlx::query!("SELECT id from user where name=$1", to_decrease)
            .fetch_one(&mut *self)
            .await
        {
            Ok(x) => x.id.expect("Couldn't unwrap id") as u32,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let mut query = sqlx::query_as::<Sqlite, (i32, u32)>(
            "SELECT mate_count,status_id FROM friendship WHERE id_1=$1 AND id_2=$2",
        );

        if to_increase < decrease_id {
            query = query.bind(to_increase).bind(decrease_id);
        } else {
            query = query.bind(decrease_id).bind(to_increase);
        }

        let (current_count, status_id) = match query.fetch_one(&mut *self).await {
            Ok(x) => x,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let status_name: String = match sqlx::query!(
            "SELECT description FROM friendship_statuses WHERE id=$1",
            status_id
        )
        .fetch_one(&mut *self)
        .await
        {
            Ok(x) => x
                .description
                .expect("Couldn't unwrap record for friendship description"),
            Err(e) => return Err(e.to_string()),
        };

        let new_count = if to_increase < decrease_id {
            current_count + 1
        } else {
            current_count - 1
        };

        if status_name == "FRIENDS" {
            match sqlx::query!(
                "UPDATE friendship SET mate_count=$1 WHERE (id_1=$2 AND id_2=$3) OR (id_1=$3 AND id_2=$2)",
                new_count,
                to_increase,
                decrease_id
            )
            .execute(&mut *self)
            .await {
            Ok(_x) => Ok(new_count),
            Err(e) => {
                return Err(e.to_string())
            }
        }
        } else {
            Err(String::from(
                "Not allowed to set a mate counter if a friendship is not active",
            ))
        }
    }

    async fn decrease_counter(
        &mut self,
        to_decrease: u32,
        to_increase: String,
    ) -> Result<i32, String> {
        let increase_id: u32 = match sqlx::query!("SELECT id from user where name=$1", to_increase)
            .fetch_one(&mut *self)
            .await
        {
            Ok(x) => x.id.expect("Couldn't unwrap id") as u32,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let mut query = sqlx::query_as::<Sqlite, (i32, u32)>(
            "SELECT mate_count,status_id FROM friendship WHERE id_1=$1 AND id_2=$2",
        );

        if to_decrease < increase_id {
            query = query.bind(to_increase).bind(increase_id);
        } else {
            query = query.bind(increase_id).bind(to_decrease);
        }

        let (current_count, status_id) = match query.fetch_one(&mut *self).await {
            Ok(x) => x,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        let status_name: String = match sqlx::query!(
            "SELECT description FROM friendship_statuses WHERE id=$1",
            status_id
        )
        .fetch_one(&mut *self)
        .await
        {
            Ok(x) => x
                .description
                .expect("Couldn't unwrap record for friendship description"),
            Err(e) => return Err(e.to_string()),
        };

        let new_count = if to_decrease < increase_id {
            current_count - 1
        } else {
            current_count + 1
        };

        if status_name == "FRIENDS" {
            match sqlx::query!(
                "UPDATE friendship SET mate_count=$1 WHERE (id_1=$2 AND id_2=$3) OR (id_1=$3 AND id_2=$2)",
                new_count,
                to_decrease,
                increase_id
            )
            .execute(&mut *self)
            .await {
            Ok(_x) => Ok(new_count),
            Err(e) => {
                return Err(e.to_string())
            }
        }
        } else {
            Err(String::from(
                "Not allowed to set a mate counter if a friendship is not active",
            ))
        }
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
