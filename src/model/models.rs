// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]
use diesel::prelude::{Queryable,Insertable};
use mongodb::bson::oid::ObjectId;
use mongodb::options::UpdateModifications;
use serde_derive::{Serialize,Deserialize};
use crate::schema::users;
use rocket_okapi::okapi::schemars::{self, JsonSchema};


#[derive(Queryable, PartialEq,Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, PartialEq,Debug)]
#[diesel(table_name = users)]
#[warn(non_camel_case_types)]
pub struct new_users {
    pub name: String,
    pub email: String,
    pub password: String,
}



