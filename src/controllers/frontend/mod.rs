// reserved for frontend controllers
// store tera template in application state

use actix_web_lab::respond::Html;
use crate::prelude::*;
use crate::models::AppState;
use crate::models::user::User;

pub async fn index(
    data: web::Data<AppState>,
    // query: web::Query<HashMap<String, String>>,
    session: Session
) -> Result<impl Responder, Error> {
    // session.purge();
    // access the session state
    let mut ctx = tera::Context::new();
    if let Some(user) = session.get::<User>("user")? {
        println!("SESSION value: {:?}", user);
        ctx.insert("user", &user);
        // modify the session state
        // session.insert("user", count + 1)?;
    } else {
        // session.insert("", 1)?;
        let user = User::new(&0,"Anonimous","","","","");
        ctx.insert("user", &user);
        println!("No user in session");
    }
    

    let s = data.tera.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(Html(s))
}

pub async fn blog(data: web::Data<AppState>,) -> Result<impl Responder, Error> {

    Ok(Html(data.tera.render("blog-home.html", &tera::Context::new())
    .map_err(|_| error::ErrorInternalServerError("Template error"))?))
}


pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/").to(index))
        .service(resource("/blog").to(blog));
}
