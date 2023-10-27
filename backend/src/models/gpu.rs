use diesel::{self, prelude::*, result::QueryResult};
use serde::{Serialize, Deserialize};

mod schema {
    table! {
        gpus {
            id -> Nullable<Integer>,
            model -> VarChar,
            vendor -> VarChar,
            vendor_model -> VarChar,
            base_frequency -> Integer,
            boost_frequency -> Integer,
            memory -> Integer,
            memory_type -> VarChar,
            additional_features -> Nullable<VarChar>,
        }
    }
}

use self::schema::gpus;

use crate::DbConn;

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = gpus)]
pub struct Gpu {
    pub id: Option<i32>,
    pub model: String,
    pub vendor: String,
    pub vendor_model: String,
    pub base_frequency: i32,
    pub boost_frequency: i32,
    pub memory: i32,
    pub memory_type: String,
    pub additional_features: Option<String>
}

#[derive(Deserialize)]
pub struct NewGpu {
    pub model: String,
    pub vendor: String,
    pub vendor_model: String,
    pub base_frequency: i32,
    pub boost_frequency: i32,
    pub memory: i32,
    pub memory_type: String,
    pub additional_features: Option<String>
}

impl Gpu {
    pub async fn all(conn: &DbConn) -> QueryResult<Vec<Gpu>> {
        conn.run(|c| gpus::table.order(gpus::id.desc()).load(c))
            .await
    }

    pub async fn insert(gpu: NewGpu, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |conn| {
            let g = Gpu {
                id: None,
                additional_features: gpu.additional_features,
                base_frequency: gpu.base_frequency,
                boost_frequency: gpu.boost_frequency,
                memory: gpu.memory,
                memory_type: gpu.memory_type,
                model: gpu.model,
                vendor: gpu.vendor,
                vendor_model: gpu.vendor_model
            };
            diesel::insert_into(gpus::table).values(&g).execute(conn)
        })
        .await
    }

    pub async fn update(gpu: Gpu, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| {
            let updated_gpu = diesel::update(gpus::table.filter(gpus::id.eq(gpu.id)));
            updated_gpu
                .set((gpus::model.eq(gpu.model), gpus::base_frequency.eq(gpu.base_frequency), gpus::boost_frequency.eq(gpu.boost_frequency), gpus::vendor.eq(gpu.vendor), gpus::memory.eq(gpu.memory), gpus::memory_type.eq(gpu.memory_type), gpus::additional_features.eq(gpu.additional_features), gpus::vendor_model.eq(gpu.vendor_model)))
                .execute(c)
        })
        .await
    }

    pub async fn delete(id: i32, conn: &DbConn) -> QueryResult<usize> {
        conn.run(move |c| diesel::delete(gpus::table).filter(gpus::id.eq(id)).execute(c))
            .await
    }
}
