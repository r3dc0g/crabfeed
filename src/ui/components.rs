use ratatui::prelude::*;
use ratatui::widgets::*;
use ratatui::{
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, List, ListState, Paragraph},
};

use crate::config::get_configuration;

use super::util::parse_hex;

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct BlockLabel {
    label: String,
}

impl WidgetRef for BlockLabel {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
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

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct BlockText<'a> {
    title: Option<String>,
    paragraph: Paragraph<'a>,
    stlye: Style,
    inner_margin: Option<Margin>,
}

impl<'a> BlockText<'a> {
    pub fn title(mut self, title: Option<String>) -> Self {
        self.title = title;
        self
    }

    pub fn paragraph(mut self, paragraph: Paragraph<'a>) -> Self {
        self.paragraph = paragraph;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.stlye = style;
        self
    }

    pub fn margin(mut self, margin: Margin) -> Self {
        self.inner_margin = Some(margin);
        self
    }
}

impl<'a> WidgetRef for BlockText<'a> {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        // let width = area.width;
        // let height = area.height;

        // self.paragraph.clone()
        //     .render(area.inner(
        //         Margin::new(
        //             (0.05 * width as f32) as u16,
        //             (0.02 * height as f32) as u16
        //         )
        //     ), buf);
        if let Some(margin) = self.inner_margin {
            Block::default()
                .borders(Borders::ALL)
                .title(self.title.clone().unwrap_or("".to_string()))
                .style(self.stlye)
                .render(area, buf);

            Clear.render_ref(area.inner(margin), buf);
            self.paragraph.clone().render(area.inner(margin), buf);
        } else {
            self.paragraph
                .clone()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(self.title.clone().unwrap_or("".to_string())),
                )
                .render(area, buf);
        }
    }
}

impl<'a> Widget for BlockText<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ItemList<'a, T>
where
    T: IntoIterator + Clone,
    T::Item: Into<ListItem<'a>>,
{
    title: Option<String>,
    items: &'a T,
    style: Style,
}

impl<'a, T> StatefulWidgetRef for ItemList<'a, T>
where
    T: IntoIterator + Clone,
    T::Item: Into<ListItem<'a>>,
{
    type State = ListState;

    fn render_ref(&self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let hightlight = parse_hex(&get_configuration().unwrap_or_default().colors.highlight);
        let highlight_style = Style::default().bg(hightlight);

        StatefulWidget::render(
            List::new(self.items.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(self.title.clone().unwrap_or("".to_string()))
                        .border_style(self.style),
                )
                .highlight_style(highlight_style),
            area,
            buf,
            state,
        );
    }
}

impl<'a, T> StatefulWidget for ItemList<'a, T>
where
    T: IntoIterator + Clone,
    T::Item: Into<ListItem<'a>>,
{
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        self.render_ref(area, buf, state);
    }
}

impl<'a, T> ItemList<'a, T>
where
    T: IntoIterator + Clone,
    T::Item: Into<ListItem<'a>>,
{
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
}

pub struct Popup<W>
where
    W: WidgetRef,
{
    height: u16,
    width: u16,
    inner_widget: Option<W>,
}

impl<W> Popup<W>
where
    W: WidgetRef,
{
    pub fn new(inner_widget: Option<W>) -> Self {
        Self {
            height: 0,
            width: 0,
            inner_widget,
        }
    }

    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }
}

impl<W> Default for Popup<W>
where
    W: WidgetRef,
{
    fn default() -> Self {
        Self {
            height: 0,
            width: 0,
            inner_widget: None,
        }
    }
}

impl<W> WidgetRef for Popup<W>
where
    W: WidgetRef,
{
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let new_area = Rect::new(
            area.x + area.width / 2 - self.width / 2,
            area.y + area.height / 2 - self.height / 2,
            self.width,
            self.height,
        );

        Clear.render_ref(new_area, buf);
        self.inner_widget.render_ref(new_area, buf);
    }
}

impl<W> Widget for Popup<W>
where
    W: WidgetRef,
{
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_ref(area, buf);
    }
}
