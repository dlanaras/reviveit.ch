#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

use models::cpu::{Cpu, NewCpu};
use rocket::{Rocket,Build, fairing::AdHoc, http::ContentType, response::status::{Created, NoContent}, serde::json::Json};

pub mod rocket_anyhow;
pub mod models;

use rocket_anyhow::Result;

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

#[get("/cpus")]
async fn index(conn: DbConn) -> Result<(ContentType, String)> {
    let cpus = Cpu::all(&conn).await?;
    let json = serde_json::to_string(&cpus)?;
    Ok((ContentType::JSON, json))
}

#[post("/cpus", data="<cpu>")]
async fn create(conn: DbConn, cpu: Json<NewCpu>) -> Result<Created<()>> {
    let created_id = Cpu::insert(cpu.0, &conn).await?;
    Ok(Created::new(format!("/cpus/{}", created_id)))
}

#[put("/cpus", data="<cpu>")]
async fn edit(conn: DbConn, cpu: Json<Cpu>) -> Result<NoContent> {
    Cpu::update(cpu.0, &conn).await?;
    Ok(NoContent)
}

#[delete("/cpus/<id>")]
async fn delete(conn: DbConn, id: i32) -> Result<NoContent> {
    Cpu::delete(id, &conn).await?;
    Ok(NoContent)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run migrations", run_migrations))
        .mount("/", routes![index, create, edit, delete])
}
