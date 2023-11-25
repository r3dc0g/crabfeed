use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::feed;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = feed)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Feed {
    pub feed_id: i32,
    pub title: Option<String>,
    pub updated: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub published: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = feed)]
pub struct NewFeed<'a> {
    pub title: &'a str,
    pub updated: &'a NaiveDateTime,
    pub description: &'a str,
    pub language: &'a str,
    pub published: &'a NaiveDateTime,
}
