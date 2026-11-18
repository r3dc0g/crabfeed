use ratatui::widgets::Paragraph;

use crate::tuihtml::parser::construct_widget;

pub struct HTMLWidget {
    paragraph: Paragraph<'static>,
    links: Vec<String>,
}

impl HTMLWidget {
    pub fn new(html: String) -> Self {
        let (paragraph, links) = construct_widget(html);
        Self {
            paragraph,
            links
        }
    }

    pub fn get_paragraph(&self) -> Paragraph<'static> {
        self.paragraph.clone()
    }

    pub fn get_links(&self) -> Vec<String> {
        self.links.clone()
    }
}
