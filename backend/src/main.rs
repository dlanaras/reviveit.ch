#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

use rocket::{Rocket,Build, fairing::AdHoc, http::ContentType, response::status::{Created, NoContent}, serde::json::Json};
use rocket_anyhow::Result;
use models::cpu::{Cpu, NewCpu};
use models::gpu::{Gpu, NewGpu};

pub mod rocket_anyhow;
pub mod models;

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    DbConn::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("diesel migrations");
        })
        .await;

    rocket
}

// CPUS
#[get("/cpus")]
async fn index_cpu(conn: DbConn) -> Result<(ContentType, String)> {
    let cpus = Cpu::all(&conn).await?;
    let json = serde_json::to_string(&cpus)?;
    Ok((ContentType::JSON, json))
}

#[post("/cpus", data="<cpu>")]
async fn create_cpu(conn: DbConn, cpu: Json<NewCpu>) -> Result<Created<()>> {
    let created_id = Cpu::insert(cpu.0, &conn).await?;
    Ok(Created::new(format!("/cpus/{}", created_id)))
}

#[put("/cpus", data="<cpu>")]
async fn edit_cpu(conn: DbConn, cpu: Json<Cpu>) -> Result<NoContent> {
    Cpu::update(cpu.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/cpus/<id>")]
async fn delete_cpu(conn: DbConn, id: i32) -> Result<NoContent> {
    Cpu::delete(id, &conn).await?;
    Ok(NoContent)
}

// GPUS
#[get("/gpus")]
async fn index_gpu(conn: DbConn) -> Result<(ContentType, String)> {
    let gpus = Gpu::all(&conn).await?;
    let json = serde_json::to_string(&gpus)?;
    Ok((ContentType::JSON, json))
}

#[post("/gpus", data="<gpu>")]
async fn create_gpu(conn: DbConn, gpu: Json<NewGpu>) -> Result<Created<()>> {
    let created_id = Gpu::insert(gpu.0, &conn).await?;
    Ok(Created::new(format!("/gpus/{}", created_id)))
}

#[put("/gpus", data="<gpu>")]
async fn edit_gpu(conn: DbConn, gpu: Json<Gpu>) -> Result<NoContent> {
    Gpu::update(gpu.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/gpus/<id>")]
async fn delete_gpu(conn: DbConn, id: i32) -> Result<NoContent> {
    Gpu::delete(id, &conn).await?;
    Ok(NoContent)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run migrations", run_migrations))
        .mount("/", routes![index_cpu, create_cpu, edit_cpu, delete_cpu, index_gpu, create_gpu, edit_gpu, delete_gpu])
}
