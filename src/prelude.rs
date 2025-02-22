use crate::AppResult;
use chrono::NaiveDateTime;
use chrono::{DateTime, Utc};
use feed_rs::model::Text;
use mime::Mime;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Feed {
    pub id: i64,
    pub title: Option<String>,
    pub updated: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub published: Option<NaiveDateTime>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct FeedData {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub description: String,
}

impl From<Feed> for FeedData {
    fn from(feed: Feed) -> Self {
        Self {
            id: feed.id,
            title: feed.title.unwrap_or_default(),
            url: String::new(),
            description: feed.description.unwrap_or_default(),
        }
    }
}

impl FeedData {
    pub fn update_url(&mut self, url: String) {
        self.url = url;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewFeed {
    pub title: Option<String>,
    pub updated: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub language: Option<String>,
    pub published: Option<NaiveDateTime>,
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
            title: self.title.clone(),
            updated: self.updated,
            description: self.description.clone(),
            language: self.language.clone(),
            published: self.published,
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Entry {
    pub id: i64,
    pub feed_id: i64,
    pub title: Option<String>,
    pub updated: Option<NaiveDateTime>,
    pub content_id: Option<i64>,
    pub media_id: Option<i64>,
    pub summary: Option<String>,
    pub source: Option<String>,
    pub read: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct EntryData {
    pub id: i64,
    pub feed_id: i64,
    pub title: String,
    pub description: String,
    pub links: Vec<Link>,
    pub media: Option<Media>,
    pub read: bool,
}

impl From<Entry> for EntryData {
    fn from(value: Entry) -> Self {
        Self {
            id: value.id,
            feed_id: value.feed_id,
            title: value.title.unwrap_or_default(),
            description: String::new(),
            links: vec![],
            media: None,
            read: value.read.unwrap_or_default(),
        }
    }
}

impl EntryData {
    pub fn update_links(&mut self, links: Vec<Link>) {
        self.links = links;
    }

    pub fn update_media(&mut self, media: Media) {
        self.media = Some(media);
    }

    pub fn update_description(&mut self, description: String) {
        self.description = description;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewEntry {
    pub feed_id: i64,
    pub title: Option<String>,
    pub updated: Option<NaiveDateTime>,
    pub content_id: Option<i64>,
    pub media_id: Option<i64>,
    pub summary: Option<String>,
    pub source: Option<String>,
}

#[derive(Default)]
pub struct EntryBuilder {
    feed_id: i64,
    title: Option<String>,
    updated: Option<NaiveDateTime>,
    content_id: Option<i64>,
    media_id: Option<i64>,
    summary: Option<String>,
    source: Option<String>,
}

impl EntryBuilder {
    pub fn new() -> Self {
        EntryBuilder::default()
    }

    pub fn feed_id(&mut self, feed_id: i64) -> &mut Self {
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

    pub fn content_id(&mut self, content_id: Option<i64>) -> &mut Self {
        let Some(entry_content_id) = content_id else {
            self.content_id = None;
            return self;
        };

        self.content_id = Some(entry_content_id);
        self
    }

    pub fn media_id(&mut self, media_id: Option<i64>) -> &mut Self {
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
            feed_id: self.feed_id,
            title: self.title.clone(),
            updated: self.updated,
            content_id: self.content_id,
            media_id: self.media_id,
            summary: self.summary.clone(),
            source: self.source.clone(),
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Media {
    pub id: i64,
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NewMedia {
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub description: Option<String>,
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
            title: self.title.clone(),
            thumbnail: self.thumbnail.clone(),
            description: self.description.clone(),
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub uri: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NewAuthor {
    pub name: String,
    pub uri: Option<String>,
    pub email: Option<String>,
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
            name: self.name.clone(),
            uri: self.uri.clone(),
            email: self.email.clone(),
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Link {
    pub id: i64,
    pub href: String,
    pub rel: Option<String>,
    pub media_type: Option<String>,
    pub href_lang: Option<String>,
    pub title: Option<String>,
    pub length: Option<i64>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NewLink {
    pub href: String,
    pub rel: Option<String>,
    pub media_type: Option<String>,
    pub href_lang: Option<String>,
    pub title: Option<String>,
    pub length: Option<i64>,
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
            href: self.href.clone(),
            rel: self.rel.clone(),
            media_type: self.media_type.clone(),
            href_lang: self.href_lang.clone(),
            title: self.title.clone(),
            length: self.length,
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Category {
    pub id: i64,
    pub term: String,
    pub scheme: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NewCategory {
    pub term: String,
    pub scheme: Option<String>,
    pub label: Option<String>,
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
            term: self.term.clone(),
            scheme: self.scheme.clone(),
            label: self.label.clone(),
        })
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Content {
    pub id: i64,
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub length: Option<i64>,
    pub src: Option<i64>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NewContent {
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub length: Option<i64>,
    pub src: Option<i64>,
}

#[derive(Default, Debug)]
pub struct ContentBuilder {
    pub body: Option<String>,
    pub content_type: Option<String>,
    pub length: Option<i64>,
    pub src: Option<i64>,
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

    pub fn src(&mut self, src: Option<i64>) -> &mut Self {
        let Some(content_src) = src else {
            self.src = None;
            return self;
        };

        self.src = Some(content_src);
        self
    }

    pub fn build(&self) -> AppResult<NewContent> {
        Ok(NewContent {
            body: self.body.clone(),
            content_type: self.content_type.clone(),
            length: self.length,
            src: self.src,
        })
    }
}
