use crate::schema::*;
use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use feed_rs::model::Text;
use mime::Mime;
use crate::AppResult;

#[derive(Clone, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = feed)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Feed {
    pub id: i32,
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

    pub fn build(&self) -> AppResult<NewFeed> {
        Ok(NewFeed {
            title: self.title.as_deref(),
            updated: self.updated.as_ref(),
            description: self.description.as_deref(),
            language: self.language.as_deref(),
            published: self.published.as_ref(),
        })
    }
}

#[derive(Clone, Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Feed, foreign_key = feed_id))]
#[diesel(table_name = entry)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    pub id: i32,
    pub feed_id: i32,
    pub title: Option<String>,
    pub updated: Option<NaiveDateTime>,
    pub content_id: Option<i32>,
    pub media_id: Option<i32>,
    pub summary: Option<String>,
    pub source: Option<String>,
    pub read: Option<bool>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = entry)]
pub struct NewEntry<'a> {
    pub feed_id: &'a i32,
    pub title: Option<&'a str>,
    pub updated: Option<&'a NaiveDateTime>,
    pub content_id: Option<&'a i32>,
    pub media_id: Option<&'a i32>,
    pub summary: Option<&'a str>,
    pub source: Option<&'a str>,
}

#[derive(Default)]
pub struct EntryBuilder {
    feed_id: i32,
    title: Option<String>,
    updated: Option<NaiveDateTime>,
    content_id: Option<i32>,
    media_id: Option<i32>,
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
            return self;
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

    pub fn media_id(&mut self, media_id: Option<i32>) -> &mut Self {
        let Some(entry_media_id) = media_id else {
            self.media_id = None;
            return self;
        };

        self.media_id = Some(entry_media_id);
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

    pub fn build(&self) -> AppResult<NewEntry> {
        Ok(NewEntry {
            feed_id: &self.feed_id,
            title: self.title.as_deref(),
            updated: self.updated.as_ref(),
            content_id: self.content_id.as_ref(),
            media_id: self.media_id.as_ref(),
            summary: self.summary.as_deref(),
            source: self.source.as_deref(),
        })
    }
}

#[derive(Clone, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = media)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Media {
    pub id: i32,
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = media)]
pub struct NewMedia<'a> {
    pub title: Option<&'a str>,
    pub thumbnail: Option<&'a str>,
    pub description: Option<&'a str>,
}

#[derive(Default)]
pub struct MediaBuilder {
    title: Option<String>,
    thumbnail: Option<String>,
    description: Option<String>,
}

impl MediaBuilder {
    pub fn new() -> Self {
        MediaBuilder::default()
    }

    pub fn title(&mut self, title: Option<Text>) -> &mut Self {
        let Some(media_title) = title else {
            self.title = None;
            return self;
        };

        self.title = Some(media_title.content);
        self
    }

    pub fn thumbnail(&mut self, thumbnail: Option<String>) -> &mut Self {
        self.thumbnail = thumbnail;
        self
    }

    pub fn description(&mut self, description: Option<Text>) -> &mut Self {
        let Some(media_description) = description else {
            self.title = None;
            return self;
        };
        self.description = Some(media_description.content);
        self
    }

    pub fn build(&self) -> AppResult<NewMedia> {
        Ok(NewMedia {
            title: self.title.as_deref(),
            thumbnail: self.thumbnail.as_deref(),
            description: self.description.as_deref(),
        })
    }
}

#[derive(Clone, Queryable, Selectable, Identifiable, Debug, PartialEq)]
#[diesel(table_name = author)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Author {
    pub id: i32,
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
pub struct AuthorBuilder {
    name: String,
    uri: Option<String>,
    email: Option<String>,
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

    pub fn build(&self) -> AppResult<NewAuthor> {
        Ok(NewAuthor {
            name: self.name.as_str(),
            uri: self.uri.as_deref(),
            email: self.email.as_deref(),
        })
    }
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Feed))]
#[diesel(belongs_to(Author))]
#[diesel(table_name = feed_author)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeedAuthor {
    pub id: i32,
    pub author_id: i32,
    pub feed_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = feed_author)]
pub struct NewFeedAuthor<'a> {
    pub author_id: &'a i32,
    pub feed_id: &'a i32,
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

    pub fn build(&self) -> AppResult<NewFeedAuthor> {
        Ok(NewFeedAuthor {
            author_id: &self.author_id,
            feed_id: &self.feed_id,
        })
    }
}

#[derive(Queryable, Selectable, Debug, Identifiable, Associations)]
#[diesel(belongs_to(Entry))]
#[diesel(belongs_to(Author))]
#[diesel(table_name = entry_author)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EntryAuthor {
    pub id: i32,
    pub author_id: i32,
    pub entry_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = entry_author)]
pub struct NewEntryAuthor<'a> {
    pub author_id: &'a i32,
    pub entry_id: &'a i32,
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

    pub fn build(&self) -> AppResult<NewEntryAuthor> {
        Ok(NewEntryAuthor {
            author_id: &self.author_id,
            entry_id: &self.entry_id,
        })
    }
}

#[derive(Clone, Queryable, Selectable, Debug, Identifiable)]
#[diesel(table_name = link)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Link {
    pub id: i32,
    pub href: String,
    pub rel: Option<String>,
    pub media_type: Option<String>,
    pub href_lang: Option<String>,
    pub title: Option<String>,
    pub length: Option<i64>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = link)]
pub struct NewLink<'a> {
    pub href: &'a str,
    pub rel: Option<&'a str>,
    pub media_type: Option<&'a str>,
    pub href_lang: Option<&'a str>,
    pub title: Option<&'a str>,
    pub length: Option<&'a i64>,
}

#[derive(Default, Debug)]
pub struct LinkBuilder {
    href: String,
    rel: Option<String>,
    media_type: Option<String>,
    href_lang: Option<String>,
    title: Option<String>,
    length: Option<i64>,
}

impl LinkBuilder {
    pub fn new() -> Self {
        LinkBuilder::default()
    }

    pub fn href(&mut self, href: String) -> &mut Self {
        self.href = href;
        self
    }

    pub fn rel(&mut self, rel: Option<String>) -> &mut Self {
        let Some(link_rel) = rel else {
            self.rel = None;
            return self;
        };

        self.rel = Some(link_rel);
        self
    }

    pub fn media_type(&mut self, media_type: Option<String>) -> &mut Self {
        let Some(link_media) = media_type else {
            self.media_type = None;
            return self;
        };

        self.media_type = Some(link_media);
        self
    }

    pub fn href_lang(&mut self, href_lang: Option<String>) -> &mut Self {
        let Some(link_href_lang) = href_lang else {
            self.href_lang = None;
            return self;
        };

        self.href_lang = Some(link_href_lang);
        self
    }

    pub fn title(&mut self, title: Option<String>) -> &mut Self {
        let Some(link_title) = title else {
            self.title = None;
            return self;
        };

        self.title = Some(link_title);
        self
    }

    pub fn length(&mut self, length: Option<u64>) -> &mut Self {
        let Some(link_length) = length else {
            self.length = None;
            return self;
        };

        self.length = Some(link_length as i64);
        self
    }

    pub fn build(&self) -> AppResult<NewLink> {
        Ok(NewLink {
            href: self.href.as_str(),
            rel: self.rel.as_deref(),
            media_type: self.media_type.as_deref(),
            href_lang: self.href_lang.as_deref(),
            title: self.title.as_deref(),
            length: self.length.as_ref(),
        })
    }
}

#[derive(Queryable, Selectable, Debug, Associations, Identifiable)]
#[diesel(belongs_to(Feed))]
#[diesel(belongs_to(Link))]
#[diesel(table_name = feed_link)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeedLink {
    pub id: i32,
    pub link_id: i32,
    pub feed_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = feed_link)]
pub struct NewFeedLink<'a> {
    pub link_id: &'a i32,
    pub feed_id: &'a i32,
}

#[derive(Default, Debug)]
pub struct FeedLinkBuilder {
    link_id: i32,
    feed_id: i32,
}

impl FeedLinkBuilder {
    pub fn new() -> Self {
        FeedLinkBuilder::default()
    }

    pub fn link_id(&mut self, link_id: i32) -> &mut Self {
        self.link_id = link_id;
        self
    }

    pub fn feed_id(&mut self, feed_id: i32) -> &mut Self {
        self.feed_id = feed_id;
        self
    }

    pub fn build(&self) -> AppResult<NewFeedLink> {
        Ok(NewFeedLink {
            link_id: &self.link_id,
            feed_id: &self.feed_id,
        })
    }
}

#[derive(Queryable, Selectable, Debug, Identifiable, Associations)]
#[diesel(belongs_to(Entry))]
#[diesel(belongs_to(Link))]
#[diesel(table_name = entry_link)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EntryLink {
    pub id: i32,
    pub link_id: i32,
    pub entry_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = entry_link)]
pub struct NewEntryLink<'a> {
    pub link_id: &'a i32,
    pub entry_id: &'a i32,
}

#[derive(Default)]
pub struct EntryLinkBuilder {
    link_id: i32,
    entry_id: i32,
}

impl EntryLinkBuilder {
    pub fn new() -> Self {
        EntryLinkBuilder::default()
    }

    pub fn link_id(&mut self, link_id: i32) -> &mut Self {
        self.link_id = link_id;
        self
    }

    pub fn entry_id(&mut self, entry_id: i32) -> &mut Self {
        self.entry_id = entry_id;
        self
    }

    pub fn build(&self) -> AppResult<NewEntryLink> {
        Ok(NewEntryLink {
            link_id: &self.link_id,
            entry_id: &self.entry_id,
        })
    }
}

#[derive(Clone, Queryable, Selectable, Debug, Identifiable)]
#[diesel(table_name = category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Category {
    pub id: i32,
    pub term: String,
    pub scheme: Option<String>,
    pub label: Option<String>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = category)]
pub struct NewCategory<'a> {
    pub term: &'a str,
    pub scheme: Option<&'a str>,
    pub label: Option<&'a str>,
}

#[derive(Default, Debug)]
pub struct CategoryBuilder {
    pub term: String,
    pub scheme: Option<String>,
    pub label: Option<String>,
}

impl CategoryBuilder {
    pub fn new() -> Self {
        CategoryBuilder::default()
    }

    pub fn term(&mut self, term: String) -> &mut Self {
        self.term = term;
        self
    }

    pub fn scheme(&mut self, scheme: Option<String>) -> &mut Self {
        let Some(category_scheme) = scheme else {
            self.scheme = None;
            return self;
        };

        self.scheme = Some(category_scheme);
        self
    }

    pub fn label(&mut self, label: Option<String>) -> &mut Self {
        let Some(category_label) = label else {
            self.label = None;
            return self;
        };

        self.label = Some(category_label);
        self
    }

    pub fn build(&self) -> AppResult<NewCategory> {
        Ok(NewCategory {
            term: self.term.as_str(),
            scheme: self.scheme.as_deref(),
            label: self.label.as_deref(),
        })
    }
}

#[derive(Queryable, Selectable, Debug, Identifiable, Associations)]
#[diesel(belongs_to(Feed))]
#[diesel(belongs_to(Category))]
#[diesel(table_name = feed_category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeedCategory {
    pub id: i32,
    pub category_id: i32,
    pub feed_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = feed_category)]
pub struct NewFeedCategory<'a> {
    pub category_id: &'a i32,
    pub feed_id: &'a i32,
}

#[derive(Default, Debug)]
pub struct FeedCategoryBuilder {
    category_id: i32,
    feed_id: i32,
}

impl FeedCategoryBuilder {
    pub fn new() -> Self {
        FeedCategoryBuilder::default()
    }

    pub fn category_id(&mut self, link_id: i32) -> &mut Self {
        self.category_id = link_id;
        self
    }

    pub fn feed_id(&mut self, feed_id: i32) -> &mut Self {
        self.feed_id = feed_id;
        self
    }

    pub fn build(&self) -> AppResult<NewFeedCategory> {
        Ok(NewFeedCategory {
            category_id: &self.category_id,
            feed_id: &self.feed_id,
        })
    }
}

#[derive(Queryable, Selectable, Debug, Identifiable, Associations)]
#[diesel(belongs_to(Entry))]
#[diesel(belongs_to(Category))]
#[diesel(table_name = entry_category)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EntryCategory {
    pub id: i32,
    pub category_id: i32,
    pub entry_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = entry_category)]
pub struct NewEntryCategory<'a> {
    pub category_id: &'a i32,
    pub entry_id: &'a i32,
}

#[derive(Default)]
pub struct EntryCategoryBuilder {
    category_id: i32,
    entry_id: i32,
}

impl EntryCategoryBuilder {
    pub fn new() -> Self {
        EntryCategoryBuilder::default()
    }

    pub fn category_id(&mut self, link_id: i32) -> &mut Self {
        self.category_id = link_id;
        self
    }

    pub fn entry_id(&mut self, entry_id: i32) -> &mut Self {
        self.entry_id = entry_id;
        self
    }

    pub fn build(&self) -> AppResult<NewEntryCategory> {
        Ok(NewEntryCategory {
            category_id: &self.category_id,
            entry_id: &self.entry_id,
        })
    }
}

#[derive(Clone, Queryable, Selectable, Debug, Identifiable)]
#[diesel(table_name = content)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Content {
    pub id: i32,
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub length: Option<i64>,
    pub src: Option<i32>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = content)]
pub struct NewContent<'a> {
    pub body: Option<&'a str>,
    pub content_type: Option<&'a str>,
    pub length: Option<&'a i64>,
    pub src: Option<&'a i32>,
}

#[derive(Default, Debug)]
pub struct ContentBuilder {
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub length: Option<i64>,
    pub src: Option<i32>,
}

impl ContentBuilder {
    pub fn new() -> Self {
        ContentBuilder::default()
    }

    pub fn body(&mut self, body: Option<String>) -> &mut Self {
        let Some(content_body) = body else {
            self.body = None;
            return self;
        };

        self.body = Some(content_body);
        self
    }

    pub fn content_type(&mut self, content_type: Mime) -> &mut Self {
        self.content_type = Some(content_type.type_().to_string());
        self
    }

    pub fn length(&mut self, length: Option<u64>) -> &mut Self {
        let Some(content_length) = length else {
            self.length = None;
            return self;
        };

        self.length = Some(content_length as i64);
        self
    }

    pub fn src(&mut self, src: Option<i32>) -> &mut Self {
        let Some(content_src) = src else {
            self.src = None;
            return self;
        };

        self.src = Some(content_src);
        self
    }

    pub fn build(&self) -> AppResult<NewContent> {
        Ok(NewContent {
            body: self.body.as_deref(),
            content_type: self.content_type.as_deref(),
            length: self.length.as_ref(),
            src: self.src.as_ref(),
        })
    }
}

#[derive(Queryable, Selectable, Debug, Identifiable, Associations)]
#[diesel(belongs_to(Media))]
#[diesel(belongs_to(Link))]
#[diesel(table_name = media_link)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MediaLink {
    pub id: i32,
    pub link_id: i32,
    pub media_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = media_link)]
pub struct NewMediaLink<'a> {
    pub link_id: &'a i32,
    pub media_id: &'a i32,
}

#[derive(Default)]
pub struct MediaLinkBuilder {
    link_id: i32,
    media_id: i32,
}

impl MediaLinkBuilder {
    pub fn new() -> Self {
        MediaLinkBuilder::default()
    }

    pub fn link_id(&mut self, link_id: i32) -> &mut Self {
        self.link_id = link_id;
        self
    }

    pub fn media_id(&mut self, media_id: i32) -> &mut Self {
        self.media_id = media_id;
        self
    }

    pub fn build(&self) -> AppResult<NewMediaLink> {
        Ok(NewMediaLink {
            link_id: &self.link_id,
            media_id: &self.media_id,
        })
    }
}
