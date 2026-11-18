// Takes in RAW HTML and returns a representation as a list of ratatui widgets

use crate::AppResult;
use html_parser::{Dom, Node};
use ratatui::{prelude::*, widgets::*};

fn remove_bad_chars(text: &str) -> String {
    text.replace("&nbsp;", "").replace("&#8217;", "'")
}

fn handle_style(node: Node) -> Vec<Span<'static>> {
    if let Some(element) = node.element() {
        if element.children.is_empty() {
            return vec![];
        }

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
                } else {
                    if element.children[0].element().is_some() {
                        let mut spans = vec![];
                        for span in handle_style(element.children[0].clone()) {
                            spans.push(span.add_modifier(Modifier::BOLD));
                        }
                        return spans;
                    }
                    return vec![Span::styled(
                        element.children[0].text().unwrap().to_string(),
                        Style::default().add_modifier(Modifier::BOLD),
                    )];
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
                } else {
                    if element.children[0].element().is_some() {
                        let mut spans = vec![];
                        for span in handle_style(element.children[0].clone()) {
                            spans.push(span.add_modifier(Modifier::ITALIC));
                        }
                        return spans;
                    }
                    return vec![Span::styled(
                        element.children[0].text().unwrap().to_string(),
                        Style::default().add_modifier(Modifier::ITALIC),
                    )];
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
                } else {
                    if element.children[0].element().is_some() {
                        let mut spans = vec![];
                        for span in handle_style(element.children[0].clone()) {
                            spans.push(span.add_modifier(Modifier::CROSSED_OUT));
                        }
                    }
                    return vec![Span::styled(
                        element.children[0].text().unwrap().to_string(),
                        Style::default().add_modifier(Modifier::CROSSED_OUT),
                    )];
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
                } else if element.children.len() == 1 {
                    if element.children[0].element().is_some() {
                        let mut spans = vec![];
                        for span in handle_style(element.children[0].clone()) {
                            spans.push(span.add_modifier(Modifier::UNDERLINED));
                        }

                        return spans;
                    }

                    return vec![Span::styled(
                        element.children[0].text().unwrap().to_string(),
                        Style::default().add_modifier(Modifier::UNDERLINED),
                    )];
                } else {
                    return vec![];
                }
            }

            _ => {
                return vec![];
            }
        }
    } else {
        if let Some(text) = node.text() {
            return vec![Span::raw(remove_bad_chars(text))];
        } else {
            return vec![];
        }
    }
}

fn handle_children(children: Vec<Node>) -> Vec<Line<'static>> {
    let mut elements = Vec::new();

    if children.is_empty() {
        return vec![];
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
                    elements.push(Line::from(spans));
                }

                "h1" | "h2" | "h3" | "h4" | "h5" => {
                    let mut spans = Vec::new();
                    for child in element.children.iter() {
                        for span in handle_style(child.clone()) {
                            spans.push(span.add_modifier(Modifier::BOLD));
                        }
                    }
                    elements.push(Line::from(spans));
                }

                "ul" => {
                    for child in element.children.iter() {
                        if let Some(element) = child.element() {
                            if element.name == "li" {
                                let mut spans = Vec::new();
                                spans.push(Span::raw("• ").add_modifier(Modifier::BOLD));
                                for child in element.children.iter() {
                                    for span in handle_style(child.clone()) {
                                        spans.push(span);
                                    }
                                }
                                elements.push(Line::from(spans));
                            }
                        }
                    }
                }

                "br" => {
                    elements.push(Line::from(Span::raw("\n")));
                }

                "b" | "strong" | "i" | "em" | "s" | "strike" | "a" | "u" => {
                    let mut spans = Vec::new();
                    for child in element.children.iter() {
                        for span in handle_style(child.clone()) {
                            spans.push(span);
                        }
                    }
                    elements.push(Line::from(spans));
                }

                _ => {}
            }
        } else {
            if let Some(text) = child.text() {
                elements.push(Line::from(Span::raw(text.to_string())));
            }
        }

        elements.push(Line::from(Span::raw("\n")));
    }

    elements
}

pub fn parse_html<'a>(html: String) -> AppResult<Paragraph<'a>> {
    let dom = Dom::parse(&html)?;
    let children = dom.children;

    if children.is_empty() {
        return Ok(Paragraph::new(Span::raw("")).wrap(Wrap::default()));
    }

    let elements = Paragraph::new(handle_children(children)).wrap(Wrap::default());

    Ok(elements)
}

pub fn parse_hex(hex: &String) -> Color {
    Color::Rgb(
        u8::from_str_radix(&hex[1..3], 16).unwrap(),
        u8::from_str_radix(&hex[3..5], 16).unwrap(),
        u8::from_str_radix(&hex[5..7], 16).unwrap(),
    )
}
