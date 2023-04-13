// reserved for auth controllers
use actix_web_lab::respond::Html;
use crate::prelude::*;
use crate::models::user::{FormRegister,FormLogin, generate_password};
use validator::validate_email;
use djangohashers::{check_password,make_password};
use crate::models::{AppState,user::User};
use crate::email::smtp::send_email;
// use tera::Tera;
use actix_web::{HttpRequest,HttpMessage, http::header};
use actix_identity::Identity;


pub async fn form_recover_password(
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
    let s = data.tera.render("auth/recover-password.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(Html(s))
}


pub async fn recover_password(
    form: web::Form<FormRegister>,
    data: web::Data<AppState>,
    _session: Session
    ) -> Result<impl Responder, Error> {

    println!("email: {}", form.email);
    let mut ctx = Context::new();
    let mut  error = ""; 
    println!("{}", error);
    if validate_email(&form.email) {
        let password = generate_password();
        let user = sqlx::query!("select email from users where email = $1", &form.email)
                        .fetch_optional(&data.db)
                        .await.unwrap();
        println!("password {}", password);
        println!("user {:?}", user);
        
        if user.is_some(){
            // save the new password
            let _row = sqlx::query!("update users set passw=$2 where email= $1", &form.email, make_password(&password))
            .execute(&data.db)
            .await.unwrap();
            // error = "Email already registered";
            println!("Check your email box for the new password");
            
            //context.insert("name", name);
            
            let _email = send_email(form.email.clone(),password);
            error = "Check your email box for the new password"
            
            // println!("send email to {} result {}", form.email, email)
        } else{
            error =  "User not Found.Check the typed email or Register.";
        }
        // enviar email

    } else {
        error = "Invalid email";
        println!("Invalid email");
    }
    // access the session state
    

    ctx.insert("form", &form);
    ctx.insert("error", &error);

    let s = data.tera.render("auth/recover-password.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(Html(s))
}



pub async fn register(
    form: web::Form<FormRegister>,
    data: web::Data<AppState>,
    _session: Session
    ) -> Result<impl Responder, Error> {

    println!("email: {}", form.email);
    let mut ctx = tera::Context::new();
    let mut  error = ""; 
    println!("{}", error);
    if validate_email(&form.email) {
        let password = generate_password();
        let user = sqlx::query!("select email from users where email = $1", &form.email)
                        .fetch_optional(&data.db)
                        .await.unwrap();
        println!("password {}", password);
        println!("user {:?}", user);

        if user.is_some(){
            // let _row = sqlx::query!("update users set passw=$2 where email= $1", &form.email, make_password(&password))
            // .execute(&data.db)
            // .await.unwrap();
            // error = "Email already registered";
            // println!("Email already registered. Sending email with new password");
            let mut context = Context::new();
            //context.insert("name", name);
            context.insert("password",&password);
            error = "User already registered. Do Login or use recover password"
            // let email = send_email(form.email.clone(),password);
            // println!("send email to {} result {}", form.email, email)
        } else{
            println!("lets insert");
            let user = sqlx::query!("insert into users (email, passw) values ($1, $2) returning id", &form.email, make_password(&password))
                        .fetch_one(&data.db)
                        .await.unwrap();
            println!("user {:?}", user);
            let email = send_email(form.email.clone(),password);
            println!("send email to {} result {}", form.email, email);
            error =  "Check your email for the password";
        }
        // enviar email

    } else {
        error = "Invalid email";
        println!("Invalid email");
    }
    // access the session state
    

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

pub async fn login(session: Session,
    // form: web::Form<FormLogin>,
) -> HttpResponse{
    
    match  session.get::<User>("user") {
        Ok(None) => {
            // session.insert("form_login",&form).expect("Erro no formulário");
            HttpResponse::Found().append_header((header::LOCATION, "/do_login")).finish()
        },
        Ok(Some(user)) => {
            println!("{:?}", user);
            println!("Welcome! {}", user.id);
            HttpResponse::Found().append_header((header::LOCATION, "/")).finish()
        },
        Err(e) => {
            println!("No form_login in session {:?}", e);
            // session.insert("form_login",&form).expect("Erro no formulário");
            HttpResponse::Found().append_header((header::LOCATION, "/do_login")).finish()
        }
    } 
}
pub async fn do_login(
    request: HttpRequest,
    // form: web::Form<FormLogin>,
    data: web::Data<AppState>,
    session: Session,
    form: web::Form<FormLogin>,
    ) -> Result<impl Responder, Error> {


    println!("email: {} password: {}", form.email,form.password);
    // let passw = make_password(&form.password);
    // println!("password {}",&passw);
    let row = sqlx::query!("select * from users where email = $1 ", &form.email)
            .fetch_optional(&data.db)
            .await.unwrap();
    let error = "";
    println!("{}",&error);
    if row.is_some(){
        let user = row.unwrap();
        let ok = check_password(&form.password, &user.passw); 
        println!("form password {} user password {:?}  check password {:?}", &form.password, &user.passw, &ok);

        
        
        if ok == Ok(true){
            println!("password ok");

            let authenticated_user = User::new(&user.id, &user.name, &user.email,&user.passw, &user.status, &user.permissions);
            Identity::login(&request.extensions(), authenticated_user.id.to_string()).unwrap();
            session.insert("user", authenticated_user)?;
            Ok(HttpResponse::Found().append_header((header::LOCATION, "/")).finish())
        } else{
            println!("invalid password");

            session.insert("error","Invalid password").unwrap();
            Ok(HttpResponse::Found().append_header((header::LOCATION, "/do_login")).finish())
            
        }

    } else{
        session.insert("error","User Not Found").unwrap();
        Ok(HttpResponse::Found().append_header((header::LOCATION, "/do_login")).finish())
        
    }
    

}

pub async fn form_login(
    data: web::Data<AppState>,
    session: Session
) -> Result<impl Responder, Error> {
    let  mut ctx = tera::Context::new();
    match  session.get::<String>("error") {
        Ok(None) => {
            // session.insert("form_login",&form).expect("Erro no formulário");
            ctx.insert("error", "");
        },
        Ok(Some(msg)) => {
            println!("{:?}", msg);
            ctx.insert("error", &msg);
        },
        Err(e) => {
            println!("Erro ao pegar erro na sessão{:?}", e);
            // session.insert("form_login",&form).expect("Erro no formulário");
            ctx.insert("error", "");
        }
    } 
    //define a empty form instance so we can reuse same layout
    let form = FormLogin::new("","");
    println!("email: {} password: {}", form.email,form.password);
    
    ctx.insert("form",&form);

    let s = data.tera.render("auth/login.html", &ctx)
        .map_err(|e| error::ErrorInternalServerError(format!("Template error {}",e)))?;
    Ok(Html(s))
}


pub async  fn logout(session:Session) -> HttpResponse{
    session.purge();
    HttpResponse::build(StatusCode::FOUND)
        .append_header((header::LOCATION, "/"))
        .finish()
}

pub fn configure(config: &mut ServiceConfig) {
    config.service(
        resource("/register")
            .route(web::post().to(register))
            .route(web::get().to(form_register)));
    config.service(
        resource("/login")
            // .route(web::post().to(login))
            .route(web::get().to(login)),
        );
    config.service(
        resource("/do_login")
            .route(web::post().to(do_login))
            .route(web::get().to(form_login)),
        );
    config.service(
        resource("/recover-password")
            .route(web::post().to(recover_password))
            .route(web::get().to(form_recover_password)),
        );
    config.service(
        resource("/logout")
            .route(web::get().to(logout)),
        );
}