use diesel::{self, prelude::*, result::QueryResult};
use serde::{Serialize, Deserialize};

mod schema {
    table! {
        fans {
            id -> Nullable<Integer>,
            vendor -> VarChar,
            vendor_model -> VarChar,
            rpm -> Integer,
            dimension -> Integer,
            max_noise -> Integer
        }
    }
}

use self::schema::fans;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = fans)]
pub struct Fan {
    pub id: Option<i32>,
    pub vendor: String,
    pub vendor_model: String,
    pub rpm: i32,
    pub dimension: i32,
    pub max_noise: i32
}

#[derive(Deserialize)]
pub struct NewFan {
    pub vendor: String,
    pub vendor_model: String,
    pub rpm: i32,
    pub dimension: i32,
    pub max_noise: i32
}

impl Fan {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Fan>> {
        conn.run(|c| fans::table.order(fans::id.desc()).load(c))
            .await
    }

    pub async fn insert(fan: NewFan, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let c = Fan {
                id: None,
                vendor: fan.vendor,
                vendor_model: fan.vendor_model,
                rpm: fan.rpm,
                dimension: fan.dimension,
                max_noise: fan.max_noise
            };
            diesel::insert_into(fans::table).values(&c).execute(conn)
        })
        .await
    }

    pub async fn update(fan: Fan, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_fan = diesel::update(fans::table.filter(fans::id.eq(fan.id)));
            updated_fan
                .set((fans::vendor.eq(fan.vendor), fans::vendor_model.eq(fan.vendor_model), fans::rpm.eq(fan.rpm), fans::dimension.eq(fan.dimension), fans::max_noise.eq(fan.max_noise)))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(fans::table).filter(fans::id.eq(id)).execute(c))
            .await
    }
}
