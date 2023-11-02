use diesel::{self, prelude::*, result::QueryResult};
use serde::{Deserialize, Serialize};

mod schema {
    table! {
        psus {
            id -> Nullable<Integer>,
            vendor -> VarChar,
            vendor_model -> VarChar,
            wattage -> Integer,
            eighty_plus -> VarChar,
            form_factor -> VarChar
        }
    }
}

use self::schema::psus;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = psus)]
pub struct Psu {
    pub id: Option<i32>,
    pub vendor: String,
    pub vendor_model: String,
    pub wattage: i32,
    pub eighty_plus: String,
    pub form_factor: String,
}

#[derive(Deserialize)]
pub struct NewPsu {
    pub vendor: String,
    pub vendor_model: String,
    pub wattage: i32,
    pub eighty_plus: String,
    pub form_factor: String,
}

impl Psu {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Psu>> {
        conn.run(|c| psus::table.order(psus::id.desc()).load(c))
            .await
    }

    pub async fn insert(psu: NewPsu, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let c = Psu {
                id: None,
                vendor: psu.vendor,
                vendor_model: psu.vendor_model,
                wattage: psu.wattage,
                eighty_plus: psu.eighty_plus,
                form_factor: psu.form_factor,
            };
            diesel::insert_into(psus::table).values(&c).execute(conn)
        })
        .await
    }

    pub async fn update(psu: Psu, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_psu = diesel::update(psus::table.filter(psus::id.eq(psu.id)));
            updated_psu
                .set((
                    psus::vendor.eq(psu.vendor),
                    psus::vendor_model.eq(psu.vendor_model),
                    psus::wattage.eq(psu.wattage),
                    psus::eighty_plus.eq(psu.eighty_plus),
                    psus::form_factor.eq(psu.form_factor),
                ))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(psus::table)
                .filter(psus::id.eq(id))
                .execute(c)
        })
        .await
    }
}
