#![allow(dead_code)]
use chrono::NaiveDateTime;
use feed_rs::model::Text;
use chrono::{DateTime, Utc};
use crate::error::Error;
use diesel::prelude::*;
use crate::schema::*;

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
    pub title: Option<&'a str>,
    pub updated: Option<&'a NaiveDateTime>,
    pub description: Option<&'a str>,
    pub language: Option<&'a str>,
    pub published: Option<&'a NaiveDateTime>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = entry)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    pub entry_id: i32,
    pub feed_id: i32,
    pub title: Option<String>,
    pub updated: Option<NaiveDateTime>,
    pub content_id: Option<i32>,
    pub summary: Option<String>,
    pub source: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = entry)]
pub struct NewEntry<'a> {
    pub feed_id: &'a i32,
    pub title: Option<&'a str>,
    pub updated: Option<&'a NaiveDateTime>,
    pub content_id: Option<&'a i32>,
    pub summary: Option<&'a str>,
    pub source: Option<&'a str>,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = author)]
pub struct Author {
    pub author_id: i32,
    pub name: String,
    pub uri: Option<String>,
    pub email: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = author)]
pub struct NewAuthor<'a> {
    pub name: &'a str,
    pub uri: Option<&'a str>,
    pub email: Option<&'a str>,
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
        let Some(feed_title) = title else {
            self.title = None;
            return self;
        };
        self.title = Some(feed_title.content);
        self
    }

    pub fn updated(&mut self, updated: Option<DateTime<Utc>>) -> &mut Self {
        let Some(feed_updated) = updated else {
            self.updated = None;
            return self;
        };

        self.updated = Some(feed_updated.naive_utc());
        self
    }

    pub fn description(&mut self, description: Option<Text>) -> &mut Self {
        let Some(feed_desc) = description else {
            self.description = None;
            return self;
        };
        self.description = Some(feed_desc.content);
        self
    }

    pub fn language(&mut self, language: Option<String>) -> &mut Self {
        let Some(feed_lang) = language else {
            self.language = None;
            return self;
        };
        self.language = Some(feed_lang);
        self
    }


    pub fn published(&mut self, published: Option<DateTime<Utc>>) -> &mut Self {
        let Some(feed_pub) = published else {
            self.published = None;
            return self;
        };
        self.published = Some(feed_pub.naive_utc());
        self
    }

    pub fn build(&self) -> Result<NewFeed> {
        Ok(NewFeed {
            title: self.title.as_deref(),
            updated: self.updated.as_ref(),
            description: self.description.as_deref(),
            language: self.language.as_deref(),
            published: self.published.as_ref(),
        })
    }
}

#[derive(Default)]
pub struct EntryBuilder {
    feed_id: i32,
    title: Option<String>,
    updated: Option<NaiveDateTime>,
    content_id: Option<i32>,
    summary: Option<String>,
    source: Option<String>,
}

impl EntryBuilder {
    pub fn new() -> Self {
        EntryBuilder::default()
    }

    pub fn feed_id(&mut self, feed_id: i32) -> &mut Self {
        self.feed_id = feed_id;
        self
    }

    pub fn title(&mut self, title: Option<Text>) -> &mut Self {
        let Some(entry_title) = title else {
            self.title = None;
            return self
        };

        self.title = Some(entry_title.content);
        self
    }

    pub fn updated(&mut self, updated: Option<DateTime<Utc>>) -> &mut Self {
        let Some(entry_updated) = updated else {
            self.updated = None;
            return self;
        };

        self.updated = Some(entry_updated.naive_utc());
        self
    }

    pub fn content_id(&mut self, content_id: Option<i32>) -> &mut Self {
        let Some(entry_content_id) = content_id else {
            self.content_id = None;
            return self;
        };

        self.content_id = Some(entry_content_id);
        self
    }

    pub fn summary(&mut self, summary: Option<Text>) -> &mut Self {
        let Some(entry_summary) = summary else {
            self.summary = None;
            return self;
        };

        self.summary = Some(entry_summary.content);
        self
    }

    pub fn source(&mut self, source: Option<String>) -> &mut Self {
        let Some(entry_source) = source else {
            self.source = None;
            return self;
        };

        self.source = Some(entry_source);
        self
    }

    pub fn build(&self) -> Result<NewEntry> {
        Ok(NewEntry {
            feed_id: &self.feed_id,
            title: self.title.as_deref(),
            updated: self.updated.as_ref(),
            content_id: self.content_id.as_ref(),
            summary: self.summary.as_deref(),
            source: self.source.as_deref(),
        })
    }
}

#[derive(Default)]
pub struct AuthorBuilder {
    pub name: String,
    pub uri: Option<String>,
    pub email: Option<String>,

}

impl AuthorBuilder {
    pub fn new() -> Self {
        AuthorBuilder::default()
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn uri(&mut self, uri: Option<String>) -> &mut Self {
        let Some(author_uri) = uri else {
            self.uri = None;
            return self;
        };

        self.uri = Some(author_uri);
        self
    }

    pub fn email(&mut self, email: Option<String>) -> &mut Self {
        let Some(author_email) = email else {
            self.email = None;
            return self;
        };

        self.email = Some(author_email);
        self
    }

    pub fn build(&self) -> Result<NewAuthor> {
        Ok(NewAuthor {
            name: self.name.as_str(),
            uri: self.uri.as_deref(),
            email: self.email.as_deref(),
        })
    }
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = feed_author)]
pub struct FeedAuthor {
    author_id: i32,
    feed_id: i32,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = entry_author)]
pub struct EntryAuthor {
    author_id: i32,
    entry_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = feed_author)]
pub struct NewFeedAuthor<'a> {
    author_id: &'a i32,
    feed_id: &'a i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = entry_author)]
pub struct NewEntryAuthor<'a> {
    author_id: &'a i32,
    entry_id: &'a i32,
}

#[derive(Default, Debug)]
pub struct FeedAuthorBuilder {
    author_id: i32,
    feed_id: i32,
}

impl FeedAuthorBuilder {
    pub fn new() -> Self {
        FeedAuthorBuilder::default()
    }

    pub fn author_id(&mut self, author_id: i32) -> &mut Self {
        self.author_id = author_id;
        self
    }

    pub fn feed_id(&mut self, feed_id: i32) -> &mut Self {
        self.feed_id = feed_id;
        self
    }

    pub fn build(&self) -> Result<NewFeedAuthor> {
        Ok(NewFeedAuthor {
            author_id: &self.author_id,
            feed_id: &self.feed_id,
        })
    }
}

#[derive(Default, Debug)]
pub struct EntryAuthorBuilder {
    author_id: i32,
    entry_id: i32,
}

impl EntryAuthorBuilder {
    pub fn new() -> Self {
        EntryAuthorBuilder::default()
    }

    pub fn author_id(&mut self, author_id: i32) -> &mut Self {
        self.author_id = author_id;
        self
    }

    pub fn entry_id(&mut self, entry_id: i32) -> &mut Self {
        self.entry_id = entry_id;
        self
    }

    pub fn build(&self) -> Result<NewEntryAuthor> {
        Ok(NewEntryAuthor {
            author_id: &self.author_id,
            entry_id: &self.entry_id,
        })
    }
}
