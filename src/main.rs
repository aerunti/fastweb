
use actix_files as fs;
// use actix_web_lab::respond::Html;
// use actix_session::{Session, SessionMiddleware, storage::RedisActorSessionStore};
use actix_web::cookie::Key;
use fastweb::prelude::*;
use tera::Tera;
use fastweb::controllers;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    // The secret key would usually be read from a configuration file/environment variables.
    let secret_key = Key::generate();
    let redis_connection_string = "127.0.0.1:6379";
    

    println!("Listening on: 127.0.0.1:8080, open browser and visit have a try!");
    HttpServer::new(move || {
        // todo: move config template directory to a .env file
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/modern-business/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
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
            .configure(controllers::errors::configure)
            

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

