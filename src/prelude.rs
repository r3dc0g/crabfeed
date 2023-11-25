use crate::error::Error;
use crate::models::NewFeed;

pub type Result<T> = core::result::Result<T, Error>;

// Implements well...
// just need to add proper
// types for fields
#[derive(Default)]
pub struct FeedBuilder {
    title: Option<String>,
    updated: Option<String>,
    description: Option<String>,
    language: Option<String>,
    published: Option<String>,
    rating: Option<String>,
    rights: Option<String>,
}

impl FeedBuilder {
    pub fn new() -> Self {
        FeedBuilder::default()
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    pub fn updated(&mut self, updated: impl Into<String>) -> &mut Self {
        self.updated = Some(updated.into());
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    pub fn language(&mut self, language: impl Into<String>) -> &mut Self {
        self.language = Some(language.into());
        self
    }

    pub fn published(&mut self, published: impl Into<String>) -> &mut Self {
        self.published = Some(published.into());
        self
    }

    pub fn rating(&mut self, rating: impl Into<String>) -> &mut Self {
        self.rating = Some(rating.into());
        self
    }

    pub fn rights(&mut self, rights: impl Into<String>) -> &mut Self {
        self.rights = Some(rights.into());
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

        let Some(rating) = self.rating.as_ref() else {
            return Err(Error::Static("No Rating"));
        };

        let Some(rights) = self.rights.as_ref() else {
            return Err(Error::Static("No Rights"));
        };

        Ok(NewFeed {
            title: title.as_str(),
            updated: updated.as_str(),
            description: description.as_str(),
            language: language.as_str(),
            published: published.as_str(),
            rating: rating.as_str(),
            rights: rights.as_str(),
        })
    }
}
