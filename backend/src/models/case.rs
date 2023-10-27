use diesel::{self, prelude::*, result::QueryResult};
use serde::{Serialize, Deserialize};

mod schema {
    table! {
        cases {
            id -> Nullable<Integer>,
            vendor -> VarChar,
            vendor_model -> VarChar,
            form_factor -> VarChar,
            fan_slots -> Integer,
            installed_fans -> Integer,
            hdd_slots -> Integer,
            ssd_slots -> Integer
        }
    }
}

use self::schema::cases;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = cases)]
pub struct Case {
    pub id: Option<i32>,
    pub vendor: String,
    pub vendor_model: String,
    pub form_factor: String,
    pub fan_slots: i32,
    pub installed_fans: i32,
    pub hdd_slots: i32,
    pub ssd_slots: i32
}

#[derive(Deserialize)]
pub struct NewCase {
    pub vendor: String,
    pub vendor_model: String,
    pub form_factor: String,
    pub fan_slots: i32,
    pub installed_fans: i32,
    pub hdd_slots: i32,
    pub ssd_slots: i32
}

impl Case {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Case>> {
        conn.run(|c| cases::table.order(cases::id.desc()).load(c))
            .await
    }

    pub async fn insert(case: NewCase, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let c = Case {
                id: None,
                vendor: case.vendor,
                vendor_model: case.vendor_model,
                form_factor: case.form_factor,
                fan_slots: case.fan_slots,
                installed_fans: case.installed_fans,
                hdd_slots: case.hdd_slots,
                ssd_slots: case.ssd_slots 
            };
            diesel::insert_into(cases::table).values(&c).execute(conn)
        })
        .await
    }

    pub async fn update(case: Case, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_case = diesel::update(cases::table.filter(cases::id.eq(case.id)));
            updated_case
                .set((cases::vendor.eq(case.vendor), cases::vendor_model.eq(case.vendor_model), cases::form_factor.eq(case.form_factor), 
                  cases::fan_slots.eq(case.fan_slots), cases::installed_fans.eq(case.installed_fans), cases::hdd_slots.eq(case.hdd_slots), cases::ssd_slots.eq(case.ssd_slots)))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(cases::table).filter(cases::id.eq(id)).execute(c))
            .await
    }
}
