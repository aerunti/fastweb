// reserved for auth controllers
use actix_web_lab::respond::Html;
use crate::prelude::*;
use crate::models::user::{FormRegister,generate_password};
use validator::validate_email;

use crate::models::AppState;



pub async fn register(
    form: web::Form<FormRegister>,
    data: web::Data<AppState>,
    _session: Session
) -> Result<impl Responder, Error> {
    println!("form: {}", form.email);
    let mut  error = ""; 
    if validate_email(&form.email) {
        let password = generate_password();
        let user = sqlx::query!("select (1) as id, 'Herp Derpinson' as name")
                        .fetch_one(&data.db)
                        .await.unwrap();
        println!("password {}", password);
        println!("user {:?}", user);
        // enviar email
    } else {
        error = "Invalid email";
        println!("Invalid email");
    }
    // access the session state
    
    let mut ctx = tera::Context::new();
    ctx.insert("form", &form);
    ctx.insert("error", &error);

    let s = data.tera.render("auth/register.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(Html(s))
}


pub async fn form_register(
    data: web::Data<AppState>,
    session: Session
) -> Result<impl Responder, Error> {
    let form = FormRegister::new();
    println!("form: {}", form.email);
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
    let error = "";
    ctx.insert("form", &form);
    ctx.insert("error", &error);
    // if let Some(_user) = session.get::<i32>("user")? {
    //     // submitted form
        
    //     ctx.insert("name", "Alexandre");
    //     ctx.insert("text", "Welcome!");
    // } else {
    //     // let user = 0;
        
    //     ctx.insert("name", "Anonimous");
    //     ctx.insert("text", "Welcome!");

    // };
    let s = data.tera.render("auth/register.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(Html(s))
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(resource("/register").route(web::post().to(register))
        .route(web::get().to(form_register)));
}