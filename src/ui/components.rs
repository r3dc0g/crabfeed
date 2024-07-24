use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, List, ListState, Paragraph},
    style::{Style, Color}
};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct BlockLabel {
    label: String,
}

impl WidgetRef for BlockLabel {
    fn render_ref(&self,area:Rect,buf: &mut Buffer) {
        Paragraph::new(self.label.clone())
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL))
            .render(area, buf);
    }
}

impl Widget for BlockLabel {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

impl BlockLabel {
    pub fn new() -> Self {
        Self {
            label: "".to_string(),
        }
    }

    pub fn label(mut self, label: String) -> Self {
        self.label = label;
        self
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ItemList<'a, T>
where
    T: IntoIterator + Clone,
    T::Item: Into<ListItem<'a>>, {

    title: Option<String>,
    items: &'a T,
    style: Style,
}

impl<'a, T> StatefulWidgetRef for ItemList<'a, T>
where
    T: IntoIterator + Clone,
    T::Item: Into<ListItem<'a>>, {
    type State = ListState;

    fn render_ref(&self,area:Rect,buf: &mut Buffer,state: &mut Self::State) {
        StatefulWidget::render(List::new(self.items.clone())
            .block(Block::default().borders(Borders::ALL).title(self.title.clone().unwrap_or("".to_string())).border_style(self.style))
            .highlight_style(Style::default().bg(Color::Red)),
            area, buf, state);
    }
}

impl<'a, T> StatefulWidget for ItemList<'a, T>
where
    T: IntoIterator + Clone,
    T::Item: Into<ListItem<'a>>, {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_ref(area, buf, state);
    }
}

impl<'a, T> ItemList<'a, T>
where
    T: IntoIterator + Clone,
    T::Item: Into<ListItem<'a>>, {
    pub fn new(items: &'a T) -> Self {
        Self {
            title: None,
            items: &items,
            style: Style::default(),
        }
    }

    pub fn title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn items(mut self, items: &'a T) -> Self {
        self.items = items;
        self
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct BlockText<'a> {
    title: Option<String>,
    paragraph: Paragraph<'a>,
}

impl<'a> BlockText<'a> {
    pub fn new(paragraph: Paragraph<'a>) -> Self {
        Self {
            title: None,
            paragraph,
        }
    }

    pub fn title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }

    pub fn paragraph(mut self, paragraph: Paragraph<'a>) -> Self {
        self.paragraph = paragraph;
        self
    }
}

impl<'a> WidgetRef for BlockText<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        self.paragraph.clone().block(Block::default().borders(Borders::ALL).title(self.title.clone().unwrap_or("".to_string()))).render(area, buf);
    }
}

impl<'a> Widget for BlockText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

