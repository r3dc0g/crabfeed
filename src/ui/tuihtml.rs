// Takes in RAW HTML and returns a representation as a list of ratatui widgets

use std::error::Error;
use ratatui::{style::*, text::*, widgets::*};
use html_parser::{Dom, Node};


fn handle_style(node: Node) -> Vec<Span<'static>> {

    if let Some(element) = node.element() {

        match element.name.as_str() {
            "b" | "strong" => {
                if element.children.len() > 1 {
                    let mut spans = Vec::new();
                    for child in element.children.iter() {
                        for span in handle_style(child.clone()) {
                            spans.push(span.add_modifier(Modifier::BOLD));
                        }
                    }
                    return spans;
                }
                else {
                    if element.children[0].element().is_some() {
                        let mut spans = vec![];
                        for span in handle_style(element.children[0].clone()) {
                            spans.push(span.add_modifier(Modifier::BOLD));
                        }
                        return spans;
                    }
                    return vec![
                        Span::styled(
                            element.children[0].text().unwrap().to_string(),
                            Style::default().add_modifier(Modifier::BOLD)
                        )
                    ];
                }
            }

            "i" | "em" => {
                if element.children.len() > 1 {
                    let mut spans = Vec::new();
                    for child in element.children.iter() {
                        for span in handle_style(child.clone()) {
                            spans.push(span.add_modifier(Modifier::ITALIC));
                        }
                    }
                    return spans;
                }
                else {
                    if element.children[0].element().is_some() {
                        let mut spans = vec![];
                        for span in handle_style(element.children[0].clone()) {
                            spans.push(span.add_modifier(Modifier::ITALIC));
                        }
                        return spans;
                    }
                    return vec![
                        Span::styled(
                            element.children[0].text().unwrap().to_string(),
                            Style::default().add_modifier(Modifier::ITALIC)
                        )
                    ];
                }
            }

            "s" | "strike" => {
                if element.children.len() > 1 {
                    let mut spans = Vec::new();
                    for child in element.children.iter() {
                        for span in handle_style(child.clone()) {
                            spans.push(span);
                        }
                    }
                    return spans;
                }
                else {
                    if element.children[0].element().is_some() {
                        let mut spans = vec![];
                        for span in handle_style(element.children[0].clone()) {
                            spans.push(span.add_modifier(Modifier::CROSSED_OUT));
                        }
                    }
                    return vec![
                        Span::styled(
                            element.children[0].text().unwrap().to_string(),
                            Style::default().add_modifier(Modifier::CROSSED_OUT)
                        )
                    ];
                }
            }

            "a" | "u" => {
                if element.children.len() > 1 {
                    let mut spans = Vec::new();
                    for child in element.children.iter() {
                        for span in handle_style(child.clone()) {
                            spans.push(span);
                        }
                    }
                    return spans;
                }
                else {
                    if element.children[0].element().is_some() {
                        let mut spans = vec![];
                        for span in handle_style(element.children[0].clone()) {
                            spans.push(span.add_modifier(Modifier::UNDERLINED));
                        }
                    }
                    return vec![
                        Span::styled(
                            element.children[0].text().unwrap().to_string(),
                            Style::default().add_modifier(Modifier::UNDERLINED)
                        )
                    ];
                }
            }

            _ => {
                return vec![];
            }
        }

    }
    else {
        if let Some(text) = node.text() {
            return vec![Span::raw(text.to_string())];
        }
        else {
            return vec![];
        }
    }
}

fn handle_children(children: Vec<Node>) -> Vec<Line<'static>> {

    let mut elements = Vec::new();

    if children.is_empty() {
        return vec![]
    }

    for child in children.iter() {
        if let Some(element) = child.element() {
            match element.name.as_str() {
                "p" => {
                    let mut spans = Vec::new();
                    for child in element.children.iter() {
                        for span in handle_style(child.clone()) {
                            spans.push(span);
                        }
                    }
                    elements.push(
                            Line::from(spans)
                    );
                }

                "h1" | "h2" | "h3" | "h4" | "h5" => {
                    let mut spans = Vec::new();
                    for child in element.children.iter() {
                        for span in handle_style(child.clone()) {
                            spans.push(span.add_modifier(Modifier::BOLD));
                        }
                    }
                    elements.push(
                        Line::from(spans)
                    );
                }

                "ul" => {
                    for child in element.children.iter() {
                        if let Some(element) = child.element() {
                            if element.name == "li" {
                                let mut spans = Vec::new();
                                for child in element.children.iter() {
                                    for span in handle_style(child.clone()) {
                                        spans.push(span);
                                    }
                                }
                                elements.push(
                                    Line::from(spans)                                );
                            }
                        }
                    }
                }

                "br" => {
                    elements.push(
                        Line::from(Span::raw("\n"))
                    );
                }

                "b" | "strong" | "i" | "em" | "s" | "strike" | "a" | "u" => {
                    let mut spans = Vec::new();
                    for child in element.children.iter() {
                        for span in handle_style(child.clone()) {
                            spans.push(span);
                        }
                    }
                    elements.push(
                        Line::from(spans)
                    );
                }

                _ => {}
            }
        }
        else {
            if let Some(text) = child.text() {
                elements.push(
                    Line::from(Span::raw(text.to_string()))
                );
            }
        }

        elements.push(Line::from(Span::raw("\n")));
    }

    elements
}

pub fn parse_html(html: &str) -> Result<Paragraph, Box<dyn Error>> {

    let dom = Dom::parse(html)?;
    let children = dom.children;

    let elements = Paragraph::new(handle_children(children)).wrap(Wrap::default());

    Ok(elements)
}
