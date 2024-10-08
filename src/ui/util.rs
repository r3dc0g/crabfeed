// Takes in RAW HTML and returns a representation as a list of ratatui widgets

use crate::error::Error;
use crate::AppResult;
use html_parser::{Dom, Node};
use ratatui::{style::*, text::*, widgets::*};

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

#[test]
fn parse_html_test() {
    let html = r#"
                <p>Analysing MQTT data, getting domains unblocked from Cloudflare DNS, making ASCII animations, and why Joe is drawn to Linux Mint. Plus why we don&#8217;t talk about Vivaldi even though it&#8217;s quite good, why Félim was wrong about right click in PuTTY, and Will doesn&#8217;t seem to understand Lemmy.</p>
<p>&nbsp;</p>
<p><strong>Discoveries</strong></p>
<p><a href=""https://github.com/mqtt-tools/mqttdecode"">MQTT decode</a></p>
<p><a href=""https://radar.cloudflare.com/domains/domain/apps.kde.org"">Cloudflare DNS was blocking apps.kde.org</a></p>
<p><a href=""https://github.com/cmang/durdraw"">Durdraw</a></p>
<p><a href=""https://blog.linuxmint.com/?p=4731"">Linux Mint 22</a></p>
<p>&nbsp;</p>
<p><strong>Feedback</strong></p>
<p><a href=""http://feditt.uk"">fedditt.uk</a></p>
<p><a href=""https://join-lemmy.org/"">Lemmy</a></p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p><strong>1Password</strong></p>
<p>Extended Access Management: Secure every sign-in for every app on every device. Support the show and check it out at <a href=""http://1password.com/latenightlinux"" target=""_blank"" rel=""noopener"" data-saferedirecturl=""https://www.google.com/url?q=http://1password.com/latenightlinux&amp;source=gmail&amp;ust=1719965161398000&amp;usg=AOvVaw1tfxrR7qwesy-7wXh-A0v8""><span class=""il"">1password</span>.<span class=""il"">com</span>/<span class=""il"">latenightlinux</span></a></p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<p>&nbsp;</p>
<div id=""post-727"" class=""post-727 post type-post status-publish format-standard hentry category-podcast"">
<div class=""post-content entry-content"">
<div id=""post-727"" class=""post-727 post type-post status-publish format-standard hentry category-podcast"">
<div class=""post-content entry-content"">
<p class=""post-contents entry-content"">See our <a href=""https://latenightlinux.com/contact/"">contact page</a> for ways to get in touch.</p>
<p><img decoding=""async"" class=""alignnone lazy loaded"" src=""https://latenightlinux.com/wp-content/uploads/latenightlinux-sm.jpg"" width=""207"" height=""207"" data-src=""https://latenightlinux.com/wp-content/uploads/latenightlinux-sm.jpg"" data-was-processed=""true"" /></p>
<p><strong>RSS</strong>: Subscribe to the <a href=""https://latenightlinux.com/feeds/"">RSS feeds here</a></p>
</div>
</div>
</div>
</div>

    "#.to_string();
}
