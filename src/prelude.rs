use chrono::NaiveDateTime;
use feed_rs::model::Text;
use chrono::{DateTime, Utc};
use crate::error::Error;
use diesel::prelude::*;
use crate::schema::feed;

pub type Result<T> = core::result::Result<T, Error>;

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

#[derive(Default)]
pub struct FeedBuilder {
    title: Option<String>,
    updated: Option<NaiveDateTime>,
    description: Option<String>,
    language: Option<String>,
    published: Option<NaiveDateTime>,
}

impl FeedBuilder {
    pub fn new() -> Self {
        FeedBuilder::default()
    }

    pub fn title(&mut self, title: Option<Text>) -> &mut Self {
        self.title = Some(title.unwrap_or_else(||
            Text {
                content_type: mime::TEXT_PLAIN_UTF_8,
                src: None,
                content: String::from("Untitled")
            }).content);
        self
    }

    pub fn updated(&mut self, updated: Option<DateTime<Utc>>) -> &mut Self {
        self.updated = Some(updated.unwrap_or_else(|| chrono::offset::Utc::now()).naive_utc());
        self
    }

    pub fn description(&mut self, description: Option<Text>) -> &mut Self {
        self.description = Some(description.unwrap_or_else(||
            Text {
                content_type: mime::TEXT_PLAIN_UTF_8,
                src: None,
                content: String::from("No description")
            }).content);
        self
    }

    pub fn language(&mut self, language: Option<String>) -> &mut Self {
        self.language = Some(language.unwrap_or_else(|| String::from("No language")));
        self
    }


    pub fn published(&mut self, published: Option<DateTime<Utc>>) -> &mut Self {
        self.published = Some(published.unwrap_or_else(|| chrono::offset::Utc::now()).naive_utc());
        self
    }

    pub fn build(&self) -> Result<NewFeed> {
        let Some(title) = self.title.as_ref() else {
            return Err(Error::Static("No Title"));
        };

        let Some(updated) = self.updated.as_ref() else {
            return Err(Error::Static("No Updated"));
        };

        let Some(description) = self.description.as_ref() else {
            return Err(Error::Static("No Description"));
        };

        let Some(language) = self.language.as_ref() else {
            return Err(Error::Static("No Language"));
        };

        let Some(published) = self.published.as_ref() else {
            return Err(Error::Static("No Published"));
        };

        Ok(NewFeed {
            title: title.as_str(),
            updated,
            description: description.as_str(),
            language: language.as_str(),
            published,
        })
    }
}
