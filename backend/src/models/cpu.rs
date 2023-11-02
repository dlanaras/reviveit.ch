use diesel::{self, prelude::*, result::QueryResult};
use serde::{Deserialize, Serialize};

mod schema {
    table! {
        cpus {
            id -> Nullable<Integer>,
            model -> VarChar,
            base_frequency -> Integer,
            boost_frequency -> Integer,
            cores -> Integer
        }
    }
}

use self::schema::cpus;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = cpus)]
pub struct Cpu {
    pub id: Option<i32>,
    pub model: String,
    pub base_frequency: i32,
    pub boost_frequency: i32,
    pub cores: i32,
}

#[derive(Deserialize)]
pub struct NewCpu {
    pub model: String,
    pub base_frequency: i32,
    pub boost_frequency: i32,
    pub cores: i32,
}

impl Cpu {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Cpu>> {
        conn.run(|c| cpus::table.order(cpus::id.desc()).load(c))
            .await
    }

    pub async fn insert(cpu: NewCpu, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let c = Cpu {
                id: None,
                model: cpu.model,
                base_frequency: cpu.base_frequency,
                boost_frequency: cpu.boost_frequency,
                cores: cpu.cores,
            };
            diesel::insert_into(cpus::table).values(&c).execute(conn)
        })
        .await
    }

    pub async fn update(cpu: Cpu, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_cpu = diesel::update(cpus::table.filter(cpus::id.eq(cpu.id)));
            updated_cpu
                .set((
                    cpus::model.eq(cpu.model),
                    cpus::base_frequency.eq(cpu.base_frequency),
                    cpus::boost_frequency.eq(cpu.boost_frequency),
                    cpus::cores.eq(cpu.cores),
                ))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            diesel::delete(cpus::table)
                .filter(cpus::id.eq(id))
                .execute(c)
        })
        .await
    }
}
