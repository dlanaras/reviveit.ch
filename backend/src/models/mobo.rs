use diesel::{self, prelude::*, result::QueryResult};
use serde::{Serialize, Deserialize};

mod schema {
    table! {
        mobos {
            id -> Nullable<Integer>,
            vendor -> VarChar,
            vendor_model -> VarChar,
            chipset -> VarChar,
            socket -> VarChar,
            memory_type -> VarChar,
            max_memory -> Integer,
            memory_slots -> Integer,
            form_factor -> VarChar,
            additional_features -> Nullable<VarChar>,
        }
    }
}

use self::schema::mobos;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = mobos)]
pub struct Mobo {
    pub id: Option<i32>,
    pub vendor: String,
    pub vendor_model: String,
    pub chipset: String,
    pub socket: String,
    pub memory_type: String,
    pub max_memory: i32,
    pub memory_slots: i32,
    pub form_factor: String,
    pub additional_features: Option<String>
}

#[derive(Deserialize)]
pub struct NewMobo {
    pub vendor: String,
    pub vendor_model: String,
    pub chipset: String,
    pub socket: String,
    pub memory_type: String,
    pub max_memory: i32,
    pub memory_slots: i32,
    pub form_factor: String,
    pub additional_features: Option<String>
}

impl Mobo {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Mobo>> {
        conn.run(|c| mobos::table.order(mobos::id.desc()).load(c))
            .await
    }

    pub async fn insert(mobo: NewMobo, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let g = Mobo {
                id: None,
                vendor: mobo.vendor,
                vendor_model: mobo.vendor_model,
                chipset: mobo.chipset,
                socket: mobo.socket,
                memory_type: mobo.memory_type,
                max_memory: mobo.max_memory,
                memory_slots: mobo.memory_slots,
                form_factor: mobo.form_factor,
                additional_features: mobo.additional_features
            };
            diesel::insert_into(mobos::table).values(&g).execute(conn)
        })
        .await
    }

    pub async fn update(mobo: Mobo, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_mobo = diesel::update(mobos::table.filter(mobos::id.eq(mobo.id)));
            updated_mobo
                .set((mobos::vendor.eq(mobo.vendor), mobos::vendor_model.eq(mobo.vendor_model), mobos::chipset.eq(mobo.chipset), mobos::socket.eq(mobo.socket),
                  mobos::memory_type.eq(mobo.memory_type), mobos::max_memory.eq(mobo.max_memory), mobos::memory_slots.eq(mobo.memory_slots),
                  mobos::form_factor.eq(mobo.form_factor), mobos::additional_features.eq(mobo.additional_features)))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(mobos::table).filter(mobos::id.eq(id)).execute(c))
            .await
    }
}
