use chrono::{DateTime, Utc, NaiveDateTime};
use diesel::prelude::*;
use crate::schema::feed;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = feed)]
pub struct Feed {
    pub feed_id: u32,
    pub title: String,
    pub updated: DateTime<Utc>,
    pub description: String,
    pub language: String,
    pub published: DateTime<Utc>,
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
