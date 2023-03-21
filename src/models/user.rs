use serde::{Deserialize,Serialize};
use std::fmt::Debug;
// use djangohashers::make_password;
use rand::{thread_rng, Rng};

#[derive(Debug,Deserialize)]
pub struct User {
    pub id: i32,
}

impl Default for User {
    fn default() -> Self {
        User { id: 0 }
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