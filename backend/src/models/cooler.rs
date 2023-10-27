use diesel::{self, prelude::*, result::QueryResult};
use serde::{Serialize, Deserialize};

mod schema {
    table! {
        coolers {
            id -> Nullable<Integer>,
            vendor -> VarChar,
            vendor_model -> VarChar,
            cooler_type -> VarChar,
            fan_count -> Integer,
            fan_dimension -> Integer
        }
    }
}

use self::schema::coolers;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = coolers)]
pub struct Cooler {
    pub id: Option<i32>,
    pub vendor: String,
    pub vendor_model: String,
    pub cooler_type: String,
    pub fan_count: i32,
    pub fan_dimension: i32
}

#[derive(Deserialize)]
pub struct NewCooler {
    pub vendor: String,
    pub vendor_model: String,
    pub cooler_type: String,
    pub fan_count: i32,
    pub fan_dimension: i32
}

impl Cooler {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Cooler>> {
        conn.run(|c| coolers::table.order(coolers::id.desc()).load(c))
            .await
    }

    pub async fn insert(cooler: NewCooler, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let c = Cooler {
                id: None,
                vendor: cooler.vendor,
                vendor_model: cooler.vendor_model,
                cooler_type: cooler.cooler_type,
                fan_count: cooler.fan_count,
                fan_dimension: cooler.fan_dimension
            };
            diesel::insert_into(coolers::table).values(&c).execute(conn)
        })
        .await
    }

    pub async fn update(cooler: Cooler, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_cooler = diesel::update(coolers::table.filter(coolers::id.eq(cooler.id)));
            updated_cooler
                .set((coolers::vendor.eq(cooler.vendor), coolers::vendor_model.eq(cooler.vendor_model), coolers::cooler_type.eq(cooler.cooler_type), coolers::fan_count.eq(cooler.fan_count), coolers::fan_dimension.eq(cooler.fan_dimension)))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(coolers::table).filter(coolers::id.eq(id)).execute(c))
            .await
    }
}
