use diesel::{self, prelude::*, result::QueryResult};
use serde::{Serialize, Deserialize};

mod schema {
    table! {
        hdds {
            id -> Nullable<Integer>,
            vendor -> VarChar,
            vendor_model -> VarChar,
            storage -> Integer,
            form_factor -> VarChar,
            rpm -> Integer
        }
    }
}

use self::schema::hdds;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = hdds)]
pub struct Hdd {
    pub id: Option<i32>,
    pub vendor: String,
    pub vendor_model: String,
    pub storage: i32,
    pub form_factor: String,
    pub rpm: i32
}

#[derive(Deserialize)]
pub struct NewHdd {
    pub vendor: String,
    pub vendor_model: String,
    pub storage: i32,
    pub form_factor: String,
    pub rpm: i32
}

impl Hdd {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Hdd>> {
        conn.run(|c| hdds::table.order(hdds::id.desc()).load(c))
            .await
    }

    pub async fn insert(hdd: NewHdd, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let c = Hdd {
                id: None,
                vendor: hdd.vendor,
                vendor_model: hdd.vendor_model,
                storage: hdd.storage,
                form_factor: hdd.form_factor,
                rpm: hdd.rpm
            };
            diesel::insert_into(hdds::table).values(&c).execute(conn)
        })
        .await
    }

    pub async fn update(hdd: Hdd, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_hdd = diesel::update(hdds::table.filter(hdds::id.eq(hdd.id)));
            updated_hdd
                .set((hdds::vendor.eq(hdd.vendor), hdds::vendor_model.eq(hdd.vendor_model), hdds::storage.eq(hdd.storage), hdds::form_factor.eq(hdd.form_factor), hdds::rpm.eq(hdd.rpm)))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(hdds::table).filter(hdds::id.eq(id)).execute(c))
            .await
    }
}
