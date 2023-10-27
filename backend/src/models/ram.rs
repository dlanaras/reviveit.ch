use diesel::{self, prelude::*, result::QueryResult};
use serde::{Serialize, Deserialize};

mod schema {
    table! {
        rams {
            id -> Nullable<Integer>,
            vendor -> VarChar,
            vendor_model -> VarChar,
            size -> Integer,
            frequency -> Integer,
            cl -> Integer,
            mem_type -> VarChar,
            form_factor -> VarChar
        }
    }
}

use self::schema::rams;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = rams)]
pub struct Ram {
    pub id: Option<i32>,
    pub vendor: String,
    pub vendor_model: String,
    pub size: i32,
    pub frequency: i32,
    pub cl: i32,
    pub mem_type: String,
    pub form_factor: String
}

#[derive(Deserialize)]
pub struct NewRam {
    pub vendor: String,
    pub vendor_model: String,
    pub size: i32,
    pub frequency: i32,
    pub cl: i32,
    pub mem_type: String,
    pub form_factor: String
}

impl Ram {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Ram>> {
        conn.run(|c| rams::table.order(rams::id.desc()).load(c))
            .await
    }

    pub async fn insert(ram: NewRam, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let c = Ram {
                id: None,
                vendor: ram.vendor,
                vendor_model: ram.vendor_model,
                size: ram.size,
                frequency: ram.frequency,
                cl: ram.cl,
                mem_type: ram.mem_type,
                form_factor: ram.form_factor
            };
            diesel::insert_into(rams::table).values(&c).execute(conn)
        })
        .await
    }

    pub async fn update(ram: Ram, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_ram = diesel::update(rams::table.filter(rams::id.eq(ram.id)));
            updated_ram
                .set((rams::vendor.eq(ram.vendor), rams::vendor_model.eq(ram.vendor_model), rams::size.eq(ram.size),
                   rams::frequency.eq(ram.frequency), rams::cl.eq(ram.cl), rams::mem_type.eq(ram.mem_type), 
                   rams::form_factor.eq(ram.form_factor)))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(rams::table).filter(rams::id.eq(id)).execute(c))
            .await
    }
}
