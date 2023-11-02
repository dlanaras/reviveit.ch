use diesel::{self, prelude::*, result::QueryResult};
use serde::{Deserialize, Serialize};

mod schema {
    table! {
        ssds {
            id -> Nullable<Integer>,
            vendor -> VarChar,
            vendor_model -> VarChar,
            storage -> Integer,
            form_factor -> VarChar,
            read_speed -> Integer,
            write_speed -> Integer
        }
    }
}

use self::schema::ssds;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = ssds)]
pub struct Ssd {
    pub id: Option<i32>,
    pub vendor: String,
    pub vendor_model: String,
    pub storage: i32,
    pub form_factor: String,
    pub read_speed: i32,
    pub write_speed: i32,
}

#[derive(Deserialize)]
pub struct NewSsd {
    pub vendor: String,
    pub vendor_model: String,
    pub storage: i32,
    pub form_factor: String,
    pub read_speed: i32,
    pub write_speed: i32,
}

impl Ssd {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Ssd>> {
        conn.run(|c| ssds::table.order(ssds::id.desc()).load(c))
            .await
    }

    pub async fn insert(ssd: NewSsd, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let c = Ssd {
                id: None,
                vendor: ssd.vendor,
                vendor_model: ssd.vendor_model,
                storage: ssd.storage,
                form_factor: ssd.form_factor,
                read_speed: ssd.read_speed,
                write_speed: ssd.write_speed,
            };
            diesel::insert_into(ssds::table).values(&c).execute(conn)
        })
        .await
    }

    pub async fn update(ssd: Ssd, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_ssd = diesel::update(ssds::table.filter(ssds::id.eq(ssd.id)));
            updated_ssd
                .set((
                    ssds::vendor.eq(ssd.vendor),
                    ssds::vendor_model.eq(ssd.vendor_model),
                    ssds::storage.eq(ssd.storage),
                    ssds::form_factor.eq(ssd.form_factor),
                    ssds::read_speed.eq(ssd.read_speed),
                    ssds::write_speed.eq(ssd.write_speed),
                ))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(ssds::table)
                .filter(ssds::id.eq(id))
                .execute(c)
        })
        .await
    }
}
