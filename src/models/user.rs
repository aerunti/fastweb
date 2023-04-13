use serde::{Deserialize,Serialize};
use std::fmt::Debug;
// use djangohashers::make_password;
use rand::{thread_rng, Rng};

#[derive(Debug,Deserialize,Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub status: String,
    pub permissions: String,
}

impl User{
    pub fn new(id: &i64, name: &str, email: &str, password: &str, status: &str, permissions: &str) -> Self {
        User {
            id: *id,
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            status: status.to_string(),
            permissions: permissions.to_string(),
        }
    }
}
impl Default for User {
    fn default() -> Self {
        User { id: 0 ,
            name: String::from(""),
            email: String::from(""),
            password: String::from(""),
            status: String::from(""),
            permissions: String::from(""),
        }
    }
}


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct FormLogin {
    pub email: String,
    pub password: String,
}


impl FormLogin {
    pub fn new(email: &str, password: &str) -> Self {
        FormLogin {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

impl Default for FormLogin {
    fn default() -> Self {
        FormLogin {
            email: String::from(""),
            password: String::from(""),
        }
    }
}


#[derive(Debug,Serialize,Deserialize)]
pub struct FormRegister {
    pub email: String,
}

impl FormRegister {
    pub fn new() -> Self {
        FormRegister {
            email: String::from(""),
        }
    }
}

impl Default for FormRegister {
    fn default() -> Self {
        FormRegister {
            email: String::from(""),
        }
    }
}



const PASSWORD_LEN: usize = 15;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789)(*&^%$#@!~";

/// Generates a random password and returns it hashed.
pub fn generate_password() -> String {
    let mut rng = thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
    // make_password(&password)
}