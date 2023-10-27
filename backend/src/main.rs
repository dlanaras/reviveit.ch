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
use models::mobo::{Mobo, NewMobo};
use models::ram::{Ram, NewRam};
use models::psu::{Psu, NewPsu};
use models::ssd::{Ssd, NewSsd};

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

// Motherboards
#[get("/mobos")]
async fn index_mobo(conn: DbConn) -> Result<(ContentType, String)> {
    let mobos = Mobo::all(&conn).await?;
    let json = serde_json::to_string(&mobos)?;
    Ok((ContentType::JSON, json))
}

#[post("/mobos", data="<mobo>")]
async fn create_mobo(conn: DbConn, mobo: Json<NewMobo>) -> Result<Created<()>> {
    let created_id = Mobo::insert(mobo.0, &conn).await?;
    Ok(Created::new(format!("/mobos/{}", created_id)))
}

#[put("/mobos", data="<mobo>")]
async fn edit_mobo(conn: DbConn, mobo: Json<Mobo>) -> Result<NoContent> {
    Mobo::update(mobo.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/mobos/<id>")]
async fn delete_mobo(conn: DbConn, id: i32) -> Result<NoContent> {
    Mobo::delete(id, &conn).await?;
    Ok(NoContent)
}

// RAM
#[get("/rams")]
async fn index_ram(conn: DbConn) -> Result<(ContentType, String)> {
    let rams = Ram::all(&conn).await?;
    let json = serde_json::to_string(&rams)?;
    Ok((ContentType::JSON, json))
}

#[post("/rams", data="<ram>")]
async fn create_ram(conn: DbConn, ram: Json<NewRam>) -> Result<Created<()>> {
    let created_id = Ram::insert(ram.0, &conn).await?;
    Ok(Created::new(format!("/rams/{}", created_id)))
}

#[put("/rams", data="<ram>")]
async fn edit_ram(conn: DbConn, ram: Json<Ram>) -> Result<NoContent> {
    Ram::update(ram.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/rams/<id>")]
async fn delete_ram(conn: DbConn, id: i32) -> Result<NoContent> {
    Ram::delete(id, &conn).await?;
    Ok(NoContent)
}

// PSU
#[get("/psus")]
async fn index_psu(conn: DbConn) -> Result<(ContentType, String)> {
    let psus = Psu::all(&conn).await?;
    let json = serde_json::to_string(&psus)?;
    Ok((ContentType::JSON, json))
}

#[post("/psus", data="<psu>")]
async fn create_psu(conn: DbConn, psu: Json<NewPsu>) -> Result<Created<()>> {
    let created_id = Psu::insert(psu.0, &conn).await?;
    Ok(Created::new(format!("/psus/{}", created_id)))
}

#[put("/psus", data="<psu>")]
async fn edit_psu(conn: DbConn, psu: Json<Psu>) -> Result<NoContent> {
    Psu::update(psu.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/psus/<id>")]
async fn delete_psu(conn: DbConn, id: i32) -> Result<NoContent> {
    Psu::delete(id, &conn).await?;
    Ok(NoContent)
}

// SSD
#[get("/ssds")]
async fn index_ssd(conn: DbConn) -> Result<(ContentType, String)> {
    let ssds = Ssd::all(&conn).await?;
    let json = serde_json::to_string(&ssds)?;
    Ok((ContentType::JSON, json))
}

#[post("/ssds", data="<ssd>")]
async fn create_ssd(conn: DbConn, ssd: Json<NewSsd>) -> Result<Created<()>> {
    let created_id = Ssd::insert(ssd.0, &conn).await?;
    Ok(Created::new(format!("/ssds/{}", created_id)))
}

#[put("/ssds", data="<ssd>")]
async fn edit_ssd(conn: DbConn, ssd: Json<Ssd>) -> Result<NoContent> {
    Ssd::update(ssd.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/ssds/<id>")]
async fn delete_ssd(conn: DbConn, id: i32) -> Result<NoContent> {
    Ssd::delete(id, &conn).await?;
    Ok(NoContent)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run migrations", run_migrations))
        .mount("/", routes![index_cpu, create_cpu, edit_cpu, delete_cpu, index_gpu, create_gpu, edit_gpu, delete_gpu,
            index_mobo, create_mobo, edit_mobo, delete_mobo, index_ram, create_ram, edit_ram, delete_ram,
            index_psu, create_psu, edit_psu, delete_psu, index_ssd, create_ssd, edit_ssd, delete_ssd])
}
