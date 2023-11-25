use chrono::NaiveDateTime;
use feed_rs::model::Text;
use chrono::{DateTime, Utc};
use crate::error::Error;
use crate::models::NewFeed;

pub type Result<T> = core::result::Result<T, Error>;

// Implements well...
// just need to add proper
// types for fields
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


    pub fn published(&mut self, published: NaiveDateTime) -> &mut Self {
        self.published = Some(published);
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
