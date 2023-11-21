use diesel::prelude::*;
use crate::schema::feed;

#[derive(Queryable, Selectable)]
#[diesel(table_name = feed)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Feed {
    pub id: u32,
    pub title: String,
    pub updated: String,
    pub description: String,
    pub language: String,
    pub published: String,
    pub rating: String,
    pub rights: String,
    pub ttl: u32,
}

#[derive(Insertable)]
#[diesel(table_name = feed)]
pub struct NewFeed<'a> {
    pub title: &'a str,
    pub updated: &'a str,
    pub description: &'a str,
    pub language: &'a str,
    pub published: &'a str,
    pub rating: &'a str,
    pub rights: &'a str,
    pub ttl: u32,
}
