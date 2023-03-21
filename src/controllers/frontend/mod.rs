// reserved for frontend controllers
// store tera template in application state

use actix_web_lab::respond::Html;
use crate::prelude::*;
pub async fn index(
    tmpl: web::Data<tera::Tera>,
    // query: web::Query<HashMap<String, String>>,
    session: Session
) -> Result<impl Responder, Error> {
    // access the session state
    if let Some(user) = session.get::<i32>("user")? {
        println!("SESSION value: {}", user);
        // modify the session state
        // session.insert("user", count + 1)?;
    } else {
        // session.insert("", 1)?;
        println!("No user in session");
    }
    let mut ctx = tera::Context::new();
    if let Some(_user) = session.get::<i32>("user")? {
        // submitted form
        
        ctx.insert("name", "Alexandre");
        ctx.insert("text", "Welcome!");
    } else {
        // let user = 0;
        
        ctx.insert("name", "Anonimous");
        ctx.insert("text", "Welcome!");

    };
    let s = tmpl.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(Html(s))
}

pub async fn blog(tera: web::Data<tera::Tera>) -> Result<impl Responder, Error> {

    Ok(Html(    tera.render("blog-home.html", &tera::Context::new())
    .map_err(|_| error::ErrorInternalServerError("Template error"))?))
}


pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/").to(index))
        .service(resource("/blog").to(blog));
}
