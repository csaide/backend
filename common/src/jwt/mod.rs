// Copyright (c) 2019 Christian Saide <supernomad>
// Licensed under the GPL-3.0, for details see https://github.com/csaide/backend/blob/master/LICENSE

use jwt::errors::ErrorKind;
use jwt::TokenData;
use jwt::{decode, encode, Header, Validation};

pub struct Key {
    pub id: String,
    pub secret: String,
}

impl From<String> for Key {
    fn from(s: String) -> Key {
        let parts: Vec<&str> = s.split("|").collect();
        Key {
            id: parts[0].to_owned(),
            secret: parts[1].to_owned(),
        }
    }
}

impl From<&String> for Key {
    fn from(s: &String) -> Key {
        let parts: Vec<&str> = s.split("|").collect();
        Key {
            id: parts[0].to_owned(),
            secret: parts[1].to_owned(),
        }
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Key {
        let parts: Vec<&str> = s.split("|").collect();
        Key {
            id: parts[0].to_owned(),
            secret: parts[1].to_owned(),
        }
    }
}

pub struct Manager {
    primary: Key,
    fallback: Vec<Key>,
}

impl Manager {
    pub fn new(primary: String, fallback: &[String]) -> Manager {
        let primary = Key::from(primary);
        let mut seconday: Vec<Key> = Vec::new();

        for secret in fallback {
            seconday.push(Key::from(secret));
        }

        Manager {
            primary: primary,
            fallback: seconday,
        }
    }

    pub fn sign(&self, sub: String) {
        let my_claims = Claims {
            sub: "b@b.com".to_owned(),
            company: "ACME".to_owned(),
            exp: 10000000000,
        };
        let key = "secret";
        let token = match encode(&Header::default(), &my_claims, key.as_ref()) {
            Ok(t) => t,
            Err(_) => panic!(), // in practice you would return the error
        };
    }
}
