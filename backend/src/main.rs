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
use models::hdd::{Hdd, NewHdd};
use models::fan::{Fan, NewFan};
use models::cooler::{Cooler, NewCooler};
use models::case::{Case, NewCase};

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

// HDD
#[get("/hdds")]
async fn index_hdd(conn: DbConn) -> Result<(ContentType, String)> {
    let hdds = Hdd::all(&conn).await?;
    let json = serde_json::to_string(&hdds)?;
    Ok((ContentType::JSON, json))
}

#[post("/hdds", data="<hdd>")]
async fn create_hdd(conn: DbConn, hdd: Json<NewHdd>) -> Result<Created<()>> {
    let created_id = Hdd::insert(hdd.0, &conn).await?;
    Ok(Created::new(format!("/hdds/{}", created_id)))
}

#[put("/hdds", data="<hdd>")]
async fn edit_hdd(conn: DbConn, hdd: Json<Hdd>) -> Result<NoContent> {
    Hdd::update(hdd.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/hdds/<id>")]
async fn delete_hdd(conn: DbConn, id: i32) -> Result<NoContent> {
    Hdd::delete(id, &conn).await?;
    Ok(NoContent)
}

// Fans
#[get("/fans")]
async fn index_fan(conn: DbConn) -> Result<(ContentType, String)> {
    let fans = Fan::all(&conn).await?;
    let json = serde_json::to_string(&fans)?;
    Ok((ContentType::JSON, json))
}

#[post("/fans", data="<fan>")]
async fn create_fan(conn: DbConn, fan: Json<NewFan>) -> Result<Created<()>> {
    let created_id = Fan::insert(fan.0, &conn).await?;
    Ok(Created::new(format!("/fans/{}", created_id)))
}

#[put("/fans", data="<fan>")]
async fn edit_fan(conn: DbConn, fan: Json<Fan>) -> Result<NoContent> {
    Fan::update(fan.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/fans/<id>")]
async fn delete_fan(conn: DbConn, id: i32) -> Result<NoContent> {
    Fan::delete(id, &conn).await?;
    Ok(NoContent)
}

// Coolers
#[get("/coolers")]
async fn index_cooler(conn: DbConn) -> Result<(ContentType, String)> {
    let coolers = Cooler::all(&conn).await?;
    let json = serde_json::to_string(&coolers)?;
    Ok((ContentType::JSON, json))
}

#[post("/coolers", data="<cooler>")]
async fn create_cooler(conn: DbConn, cooler: Json<NewCooler>) -> Result<Created<()>> {
    let created_id = Cooler::insert(cooler.0, &conn).await?;
    Ok(Created::new(format!("/coolers/{}", created_id)))
}

#[put("/coolers", data="<cooler>")]
async fn edit_cooler(conn: DbConn, cooler: Json<Cooler>) -> Result<NoContent> {
    Cooler::update(cooler.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/coolers/<id>")]
async fn delete_cooler(conn: DbConn, id: i32) -> Result<NoContent> {
    Cooler::delete(id, &conn).await?;
    Ok(NoContent)
}

// Cases
#[get("/cases")]
async fn index_case(conn: DbConn) -> Result<(ContentType, String)> {
    let cases = Case::all(&conn).await?;
    let json = serde_json::to_string(&cases)?;
    Ok((ContentType::JSON, json))
}

#[post("/cases", data="<case>")]
async fn create_case(conn: DbConn, case: Json<NewCase>) -> Result<Created<()>> {
    let created_id = Case::insert(case.0, &conn).await?;
    Ok(Created::new(format!("/cases/{}", created_id)))
}

#[put("/cases", data="<case>")]
async fn edit_case(conn: DbConn, case: Json<Case>) -> Result<NoContent> {
    Case::update(case.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/cases/<id>")]
async fn delete_case(conn: DbConn, id: i32) -> Result<NoContent> {
    Case::delete(id, &conn).await?;
    Ok(NoContent)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run migrations", run_migrations))
        .mount("/", routes![index_cpu, create_cpu, edit_cpu, delete_cpu, index_gpu, create_gpu, edit_gpu, delete_gpu,
            index_mobo, create_mobo, edit_mobo, delete_mobo, index_ram, create_ram, edit_ram, delete_ram,
            index_psu, create_psu, edit_psu, delete_psu, index_ssd, create_ssd, edit_ssd, delete_ssd,
            index_hdd, create_hdd, edit_hdd, delete_hdd, index_fan, create_fan, edit_fan, delete_fan,
            index_cooler, create_cooler, edit_cooler, delete_cooler, index_case, create_case, edit_case, delete_case])
}
