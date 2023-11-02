#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

use models::case::{Case, NewCase};
use models::cooler::{Cooler, NewCooler};
use models::cpu::{Cpu, NewCpu};
use models::fan::{Fan, NewFan};
use models::gpu::{Gpu, NewGpu};
use models::hdd::{Hdd, NewHdd};
use models::mobo::{Mobo, NewMobo};
use models::psu::{NewPsu, Psu};
use models::ram::{NewRam, Ram};
use models::ssd::{NewSsd, Ssd};
use rocket::{
    fairing::AdHoc,
    http::{ContentType, Cookie, CookieJar, Status},
    outcome::IntoOutcome,
    request::{self, FromRequest, Request},
    response::status::{BadRequest, Created, NoContent, Unauthorized},
    serde::json::Json,
    Build, Rocket,
};
use rocket_anyhow::Result;
use serde::Deserialize;

pub mod models;
pub mod rocket_anyhow;

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

pub struct AdminUser(usize);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r rocket::Request<'_>) -> request::Outcome<Self, Self::Error> {
        request
            .cookies()
            .get_private("auth")
            .and_then(|c| c.value().parse().ok())
            .map(|c| AdminUser(c))
            .or_forward(())
    }
}

#[derive(Deserialize)]
pub struct LoginData {
    pub id: i32,
    pub user_name: String,
    pub password: String,
}

// CPUS
#[get("/cpus")]
async fn index_cpu(conn: DbConn, _admin: AdminUser) -> Result<(ContentType, String)> {
    let cpus = Cpu::all(&conn).await?;
    let json = serde_json::to_string(&cpus)?;
    Ok((ContentType::JSON, json))
}

#[post("/cpus", data = "<cpu>")]
async fn create_cpu(conn: DbConn, cpu: Json<NewCpu>, _admin: AdminUser) -> Result<Created<()>> {
    let created_id = Cpu::insert(cpu.0, &conn).await?;
    Ok(Created::new(format!("/cpus/{}", created_id)))
}

#[put("/cpus", data = "<cpu>")]
async fn edit_cpu(conn: DbConn, cpu: Json<Cpu>, _admin: AdminUser) -> Result<NoContent> {
    Cpu::update(cpu.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/cpus/<id>")]
async fn delete_cpu(conn: DbConn, id: i32, _admin: AdminUser) -> Result<NoContent> {
    Cpu::delete(id, &conn).await?;
    Ok(NoContent)
}

// GPUS
#[get("/gpus")]
async fn index_gpu(conn: DbConn, _admin: AdminUser) -> Result<(ContentType, String)> {
    let gpus = Gpu::all(&conn).await?;
    let json = serde_json::to_string(&gpus)?;
    Ok((ContentType::JSON, json))
}

#[post("/gpus", data = "<gpu>")]
async fn create_gpu(conn: DbConn, gpu: Json<NewGpu>, _admin: AdminUser) -> Result<Created<()>> {
    let created_id = Gpu::insert(gpu.0, &conn).await?;
    Ok(Created::new(format!("/gpus/{}", created_id)))
}

#[put("/gpus", data = "<gpu>")]
async fn edit_gpu(conn: DbConn, gpu: Json<Gpu>, _admin: AdminUser) -> Result<NoContent> {
    Gpu::update(gpu.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/gpus/<id>")]
async fn delete_gpu(conn: DbConn, id: i32, _admin: AdminUser) -> Result<NoContent> {
    Gpu::delete(id, &conn).await?;
    Ok(NoContent)
}

// Motherboards
#[get("/mobos")]
async fn index_mobo(conn: DbConn, _admin: AdminUser) -> Result<(ContentType, String)> {
    let mobos = Mobo::all(&conn).await?;
    let json = serde_json::to_string(&mobos)?;
    Ok((ContentType::JSON, json))
}

#[post("/mobos", data = "<mobo>")]
async fn create_mobo(conn: DbConn, mobo: Json<NewMobo>, _admin: AdminUser) -> Result<Created<()>> {
    let created_id = Mobo::insert(mobo.0, &conn).await?;
    Ok(Created::new(format!("/mobos/{}", created_id)))
}

#[put("/mobos", data = "<mobo>")]
async fn edit_mobo(conn: DbConn, mobo: Json<Mobo>, _admin: AdminUser) -> Result<NoContent> {
    Mobo::update(mobo.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/mobos/<id>")]
async fn delete_mobo(conn: DbConn, id: i32, _admin: AdminUser) -> Result<NoContent> {
    Mobo::delete(id, &conn).await?;
    Ok(NoContent)
}

// RAM
#[get("/rams")]
async fn index_ram(conn: DbConn, _admin: AdminUser) -> Result<(ContentType, String)> {
    let rams = Ram::all(&conn).await?;
    let json = serde_json::to_string(&rams)?;
    Ok((ContentType::JSON, json))
}

#[post("/rams", data = "<ram>")]
async fn create_ram(conn: DbConn, ram: Json<NewRam>, _admin: AdminUser) -> Result<Created<()>> {
    let created_id = Ram::insert(ram.0, &conn).await?;
    Ok(Created::new(format!("/rams/{}", created_id)))
}

#[put("/rams", data = "<ram>")]
async fn edit_ram(conn: DbConn, ram: Json<Ram>, _admin: AdminUser) -> Result<NoContent> {
    Ram::update(ram.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/rams/<id>")]
async fn delete_ram(conn: DbConn, id: i32, _admin: AdminUser) -> Result<NoContent> {
    Ram::delete(id, &conn).await?;
    Ok(NoContent)
}

// PSU
#[get("/psus")]
async fn index_psu(conn: DbConn, _admin: AdminUser) -> Result<(ContentType, String)> {
    let psus = Psu::all(&conn).await?;
    let json = serde_json::to_string(&psus)?;
    Ok((ContentType::JSON, json))
}

#[post("/psus", data = "<psu>")]
async fn create_psu(conn: DbConn, psu: Json<NewPsu>, _admin: AdminUser) -> Result<Created<()>> {
    let created_id = Psu::insert(psu.0, &conn).await?;
    Ok(Created::new(format!("/psus/{}", created_id)))
}

#[put("/psus", data = "<psu>")]
async fn edit_psu(conn: DbConn, psu: Json<Psu>, _admin: AdminUser) -> Result<NoContent> {
    Psu::update(psu.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/psus/<id>")]
async fn delete_psu(conn: DbConn, id: i32, _admin: AdminUser) -> Result<NoContent> {
    Psu::delete(id, &conn).await?;
    Ok(NoContent)
}

// SSD
#[get("/ssds")]
async fn index_ssd(conn: DbConn, _admin: AdminUser) -> Result<(ContentType, String)> {
    let ssds = Ssd::all(&conn).await?;
    let json = serde_json::to_string(&ssds)?;
    Ok((ContentType::JSON, json))
}

#[post("/ssds", data = "<ssd>")]
async fn create_ssd(conn: DbConn, ssd: Json<NewSsd>, _admin: AdminUser) -> Result<Created<()>> {
    let created_id = Ssd::insert(ssd.0, &conn).await?;
    Ok(Created::new(format!("/ssds/{}", created_id)))
}

#[put("/ssds", data = "<ssd>")]
async fn edit_ssd(conn: DbConn, ssd: Json<Ssd>, _admin: AdminUser) -> Result<NoContent> {
    Ssd::update(ssd.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/ssds/<id>")]
async fn delete_ssd(conn: DbConn, id: i32, _admin: AdminUser) -> Result<NoContent> {
    Ssd::delete(id, &conn).await?;
    Ok(NoContent)
}

// HDD
#[get("/hdds")]
async fn index_hdd(conn: DbConn, _admin: AdminUser) -> Result<(ContentType, String)> {
    let hdds = Hdd::all(&conn).await?;
    let json = serde_json::to_string(&hdds)?;
    Ok((ContentType::JSON, json))
}

#[post("/hdds", data = "<hdd>")]
async fn create_hdd(conn: DbConn, hdd: Json<NewHdd>, _admin: AdminUser) -> Result<Created<()>> {
    let created_id = Hdd::insert(hdd.0, &conn).await?;
    Ok(Created::new(format!("/hdds/{}", created_id)))
}

#[put("/hdds", data = "<hdd>")]
async fn edit_hdd(conn: DbConn, hdd: Json<Hdd>, _admin: AdminUser) -> Result<NoContent> {
    Hdd::update(hdd.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/hdds/<id>")]
async fn delete_hdd(conn: DbConn, id: i32, _admin: AdminUser) -> Result<NoContent> {
    Hdd::delete(id, &conn).await?;
    Ok(NoContent)
}

// Fans
#[get("/fans")]
async fn index_fan(conn: DbConn, _admin: AdminUser) -> Result<(ContentType, String)> {
    let fans = Fan::all(&conn).await?;
    let json = serde_json::to_string(&fans)?;
    Ok((ContentType::JSON, json))
}

#[post("/fans", data = "<fan>")]
async fn create_fan(conn: DbConn, fan: Json<NewFan>, _admin: AdminUser) -> Result<Created<()>> {
    let created_id = Fan::insert(fan.0, &conn).await?;
    Ok(Created::new(format!("/fans/{}", created_id)))
}

#[put("/fans", data = "<fan>")]
async fn edit_fan(conn: DbConn, fan: Json<Fan>, _admin: AdminUser) -> Result<NoContent> {
    Fan::update(fan.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/fans/<id>")]
async fn delete_fan(conn: DbConn, id: i32, _admin: AdminUser) -> Result<NoContent> {
    Fan::delete(id, &conn).await?;
    Ok(NoContent)
}

// Coolers
#[get("/coolers")]
async fn index_cooler(conn: DbConn, _admin: AdminUser) -> Result<(ContentType, String)> {
    let coolers = Cooler::all(&conn).await?;
    let json = serde_json::to_string(&coolers)?;
    Ok((ContentType::JSON, json))
}

#[post("/coolers", data = "<cooler>")]
async fn create_cooler(
    conn: DbConn,
    cooler: Json<NewCooler>,
    _admin: AdminUser,
) -> Result<Created<()>> {
    let created_id = Cooler::insert(cooler.0, &conn).await?;
    Ok(Created::new(format!("/coolers/{}", created_id)))
}

#[put("/coolers", data = "<cooler>")]
async fn edit_cooler(conn: DbConn, cooler: Json<Cooler>, _admin: AdminUser) -> Result<NoContent> {
    Cooler::update(cooler.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/coolers/<id>")]
async fn delete_cooler(conn: DbConn, id: i32, _admin: AdminUser) -> Result<NoContent> {
    Cooler::delete(id, &conn).await?;
    Ok(NoContent)
}

// Cases
#[get("/cases")]
async fn index_case(conn: DbConn, _admin: AdminUser) -> Result<(ContentType, String)> {
    let cases = Case::all(&conn).await?;
    let json = serde_json::to_string(&cases)?;
    Ok((ContentType::JSON, json))
}

#[post("/cases", data = "<case>")]
async fn create_case(conn: DbConn, case: Json<NewCase>, _admin: AdminUser) -> Result<Created<()>> {
    let created_id = Case::insert(case.0, &conn).await?;
    Ok(Created::new(format!("/cases/{}", created_id)))
}

#[put("/cases", data = "<case>")]
async fn edit_case(conn: DbConn, case: Json<Case>, _admin: AdminUser) -> Result<NoContent> {
    Case::update(case.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/cases/<id>")]
async fn delete_case(conn: DbConn, id: i32, _admin: AdminUser) -> Result<NoContent> {
    Case::delete(id, &conn).await?;
    Ok(NoContent)
}

// Computers

// Login

#[post("/auth", data = "<login>")]
async fn login(conn: DbConn, cookies: &CookieJar<'_>, login: Json<LoginData>) -> Result<Status> {
    todo!("Check login details in db")
    // cookies.add_private(Cookie::new("auth", "1"));
    // return Ok(Status::NoContent);

    // Ok(Status::Unauthorized)
}

#[post("/logout")]
async fn logout(cookies: &CookieJar<'_>) -> Result<NoContent> {
    cookies.remove_private(Cookie::named("auth"));
    Ok(NoContent)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run migrations", run_migrations))
        .mount(
            "/",
            routes![
                index_cpu,
                create_cpu,
                edit_cpu,
                delete_cpu,
                index_gpu,
                create_gpu,
                edit_gpu,
                delete_gpu,
                index_mobo,
                create_mobo,
                edit_mobo,
                delete_mobo,
                index_ram,
                create_ram,
                edit_ram,
                delete_ram,
                index_psu,
                create_psu,
                edit_psu,
                delete_psu,
                index_ssd,
                create_ssd,
                edit_ssd,
                delete_ssd,
                index_hdd,
                create_hdd,
                edit_hdd,
                delete_hdd,
                index_fan,
                create_fan,
                edit_fan,
                delete_fan,
                index_cooler,
                create_cooler,
                edit_cooler,
                delete_cooler,
                index_case,
                create_case,
                edit_case,
                delete_case,
                login,
                logout
            ],
        )
}
