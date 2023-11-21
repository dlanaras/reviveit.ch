use anyhow::{bail, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::{self, prelude::*, result::QueryResult};
use serde::{Deserialize, Serialize};
use std::env;

mod schema {
    table! {
        admins {
            id -> Nullable<Integer>,
            username -> VarChar,
            password_hash -> VarChar,
        }
    }
}

use self::schema::admins;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = admins)]
pub struct Admin {
    pub id: Option<i32>,
    pub username: String,
    pub password_hash: String,
}

#[derive(Deserialize, Clone)]
pub struct NewAdmin {
    pub username: String,
    pub password: String,
    pub create_key: String,
}

impl Admin {
    pub async fn by_username(username: String, conn: &DbConn) -> QueryResult<Option<Admin>> {
        conn.run(move |c| {
            admins::table
                .filter(admins::username.eq(username))
                .limit(1)
                .load(c)
        })
        .await
        .map(|a| a.into_iter().next()) // get first item, can't use .first() because we need to own
                                       // the admin
    }

    pub fn check_password(&self, password: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(&self.password_hash)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub async fn insert(admin: NewAdmin, conn: &DbConn) -> Result<usize> {
        // Check create_key against env var
        let Ok(create_key) = env::var("CREATE_ADMIN_KEY") else {
            bail!("CREATE_ADMIN_KEY env var not set")
        };
        if admin.create_key != create_key {
            bail!("wrong create_key");
        }

        if (Admin::by_username(admin.username.clone(), conn).await?).is_some() {
            bail!("user called {} already exists", admin.username);
        }

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let password_hash = argon2.hash_password(admin.password.as_bytes(), &salt)?.to_string();

        Ok(conn
            .run(move |c| {
                let admin = Admin {
                    id: None,
                    username: admin.username,
                    password_hash: password_hash, // TODO: hash password
                };
                diesel::insert_into(admins::table).values(&admin).execute(c)
            })
            .await?)
    }
}
