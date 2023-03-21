
use actix_files as fs;
// use actix_web_lab::respond::Html;
// use actix_session::{Session, SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::cookie::Key;
use fastweb::prelude::*;
use tera::Tera;
use fastweb::controllers;
use sqlx::{postgres::PgPoolOptions};
use  dotenv_codegen::dotenv;
use fastweb::models::AppState;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    // The secret key would usually be read from a configuration file/environment variables.
    let secret_key = Key::generate();
    let redis_connection_string = "127.0.0.1:6379";
    let database_url = dotenv!("DATABASE_URL");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    println!("Listening on: 127.0.0.1:8080, open browser and visit have a try!");
    HttpServer::new(move || {
        // todo: move config template directory to a .env file
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/", "modern-business","/**/*")).unwrap();

        


        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone(), tera: tera }))
            // .app_data(web::Data::new(tera))
            .wrap(middleware::Logger::default()) // enable logger
            // Add session management to your application using Redis for session state storage
            .wrap(
                SessionMiddleware::new(
                    RedisActorSessionStore::new(redis_connection_string),
                    secret_key.clone()
                )
            )

            .service(
                fs::Files::new("/static", "./static")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .configure(controllers::frontend::configure)
            .configure(controllers::auth::configure)
            .configure(controllers::errors::configure)
            
            

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

