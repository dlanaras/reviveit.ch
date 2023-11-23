use diesel::{self, prelude::*, result::QueryResult};
use serde::{Deserialize, Serialize};
use super::cpu::Cpu;
use super::gpu::Gpu;
use super::case::Case;
use super::psu::Psu;
use super::mobo::Mobo;
use super::cooler::Cooler;

mod schema {
    table! {
        computers {
            id -> Nullable<Integer>,
            name -> VarChar,
            description -> VarChar,
            cpu_id -> Integer,
            gpu_id -> Nullable<Integer>,
            mobo_id -> Integer,
            psu_id -> Integer,
            cooler_id -> Integer,
            case_id -> Integer
        }
    }

    table! {
        computer_fans(fan_id, computer_id) {
            fan_id -> Integer,
            computer_id -> Integer
        }
    }

    table! {
        computer_ssds(ssd_id, computer_id) {
            ssd_id -> Integer,
            computer_id -> Integer
        }
    }

    table! {
        computer_hdds(hdd_id, computer_id) {
            hdd_id -> Integer,
            computer_id -> Integer
        }
    }

    table! {
        computer_rams(ram_id, computer_id) {
            ram_id -> Integer,
            computer_id -> Integer
        }
    }

   // diesel::joinable!(computers -> computer_fans(computer_id));

    diesel::allow_tables_to_appear_in_same_query!(
        computers,
        computer_fans,
        computer_ssds,
        computer_hdds,
        computer_rams
    );
}

use self::schema::{computers, computer_fans, computer_hdds, computer_rams, computer_ssds};

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone, Associations)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = computer_fans, belongs_to(Computer))]
pub struct ComputerFans {
    computer_id: i32,
    fan_id: i32

}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone, Associations)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = computer_ssds, belongs_to(Computer))]
pub struct ComputerSsds {
    computer_id: i32,
    ssd_id: i32
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone, Associations)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = computer_hdds, belongs_to(Computer))]
pub struct ComputerHdds {
    computer_id: i32,
    hdd_id: i32
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone, Associations)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = computer_rams, belongs_to(Computer))]
pub struct ComputerRams {
    computer_id: i32,
    ram_id: i32
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone, Associations)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = computers, belongs_to(Cpu), belongs_to(Gpu), belongs_to(Mobo), belongs_to(Psu), belongs_to(Cooler), belongs_to(Case))]
pub struct Computer {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub cpu_id: i32,
    pub gpu_id: Option<i32>,
    pub mobo_id: i32,
    pub psu_id: i32,
    pub cooler_id: i32,
    pub case_id: i32,
}

#[derive(Deserialize)]
pub struct NewComputer {
    pub name: String,
    pub description: String,
    pub cpu_id: i32,
    pub gpu_id: i32,
    pub mobo_id: i32,
    pub psu_id: i32,
    pub cooler_id: i32,
    pub case_id: i32,
    pub fan_ids: Vec<i32>,
    pub ssd_ids: Vec<i32>,
    pub hdd_ids: Vec<i32>,
    pub ram_ids: Vec<i32>,
}

impl Computer {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Computer>> {
        conn.run(|c| computers::table.order(computers::id.desc()).load(c))
            .await
    }

    /*pub async fn insert(computer: NewComputer, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let c = Computer {
                id: None,

            };
            diesel::insert_into(computers::table).values(&c).execute(conn)
        })
        .await
    }*/

    /*pub async fn update(computer: Computer, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_computer = diesel::update(computers::table.filter(computers::id.eq(computer.id)));
            updated_computer
                .set((
                    computers::vendor.eq(computer.vendor),
                    computers::vendor_model.eq(computer.vendor_model),
                    computers::form_factor.eq(computer.form_factor),
                    computers::fan_slots.eq(computer.fan_slots),
                    computers::installed_fans.eq(computer.installed_fans),
                    computers::hdd_slots.eq(computer.hdd_slots),
                    computers::ssd_slots.eq(computer.ssd_slots),
                ))
                .execute(c)
        })
        .await
    }*/

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(computers::table)
                .filter(computers::id.eq(id))
                .execute(c)
        })
        .await
    }
}
