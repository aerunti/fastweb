// reserved for frontend controllers
// store tera template in application state

use actix_web_lab::respond::Html;
use crate::prelude::*;
pub async fn index(
    tmpl: web::Data<tera::Tera>,
    query: web::Query<HashMap<String, String>>,
    session: Session
) -> Result<impl Responder, Error> {
    // access the session state
    if let Some(count) = session.get::<i32>("counter")? {
        println!("SESSION value: {}", count);
        // modify the session state
        session.insert("counter", count + 1)?;
    } else {
        session.insert("counter", 1)?;
    }

    let s = if let Some(name) = query.get("name") {
        // submitted form
        let mut ctx = tera::Context::new();
        ctx.insert("name", name);
        ctx.insert("text", "Welcome!");
        tmpl.render("user.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    } else {
        tmpl.render("index.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };

    Ok(Html(s))
}

pub async fn blog(tera: web::Data<tera::Tera>) -> Result<impl Responder, Error> {

    Ok(Html(    tera.render("blog-home.html", &tera::Context::new())
    .map_err(|_| error::ErrorInternalServerError("Template error"))?))
}


pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/").to(index));
}