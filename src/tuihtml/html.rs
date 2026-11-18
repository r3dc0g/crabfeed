use std::collections::HashMap;

use ratatui::{style::Modifier};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HTMLTag {
    HTML,
    HEAD,
    BODY,
    TITLE,
    META,
    LINK,
    STYLE,
    DIV,
    SPAN,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    A,
    BOLD,
    U,
    S,
    DEL,
    BR,
    HR,
    IMG,
    LABEL,
    TABLE,
    TR,
    TH,
    TD,
    UL,
    OL,
    LI,
    TEXTAREA,
    IFRAME,
    SOURCE,
    NAV,
    HEADER,
    FOOTER,
    SECTION,
    ARTICLE,
    ASIDE,
    MAIN,
    FIGURE,
    FIGCAPTION,
    STRONG,
    EM,
    I,
    CODE,
    PRE,
    BLOCKQUOTE,
    CITE,
    ABBR,
    TIME,
    DATA,
    PROGRESS,
    METER,
    DETAILS,
    SUMMARY,
    DIALOG,
    CANVAS,
    SVG,
    MATH,
    TEMPLATE,
    SLOT,
    OUTPUT,
    FIELDSET,
    LEGEND,
    DATALIST,
    OPTGROUP,
    SMALL,
    AREA,
    COL,
    COLGROUP,
    CAPTION,
    THEAD,
    TBODY,
    TFOOT,
    UNKNOWN,
}

impl HTMLTag {
    pub fn from_string(string: String) -> HTMLTag {
        match string.to_lowercase().as_str() {
            "html" => HTMLTag::HTML,
            "head" => HTMLTag::HEAD,
            "body" => HTMLTag::BODY,
            "title" => HTMLTag::TITLE,
            "meta" => HTMLTag::META,
            "link" => HTMLTag::LINK,
            "style" => HTMLTag::STYLE,
            "div" => HTMLTag::DIV,
            "span" => HTMLTag::SPAN,
            "h1" => HTMLTag::H1,
            "h2" => HTMLTag::H2,
            "h3" => HTMLTag::H3,
            "h4" => HTMLTag::H4,
            "h5" => HTMLTag::H5,
            "h6" => HTMLTag::H6,
            "p" => HTMLTag::P,
            "a" => HTMLTag::A,
            "b" => HTMLTag::BOLD,
            "u" => HTMLTag::U,
            "s" => HTMLTag::S,
            "del" => HTMLTag::DEL,
            "br" => HTMLTag::BR,
            "hr" => HTMLTag::HR,
            "img" => HTMLTag::IMG,
            "label" => HTMLTag::LABEL,
            "table" => HTMLTag::TABLE,
            "tr" => HTMLTag::TR,
            "th" => HTMLTag::TH,
            "td" => HTMLTag::TD,
            "ul" => HTMLTag::UL,
            "ol" => HTMLTag::OL,
            "li" => HTMLTag::LI,
            "textarea" => HTMLTag::TEXTAREA,
            "iframe" => HTMLTag::IFRAME,
            "source" => HTMLTag::SOURCE,
            "nav" => HTMLTag::NAV,
            "header" => HTMLTag::HEADER,
            "footer" => HTMLTag::FOOTER,
            "section" => HTMLTag::SECTION,
            "article" => HTMLTag::ARTICLE,
            "aside" => HTMLTag::ASIDE,
            "main" => HTMLTag::MAIN,
            "figure" => HTMLTag::FIGURE,
            "figcaption" => HTMLTag::FIGCAPTION,
            "strong" => HTMLTag::STRONG,
            "em" => HTMLTag::EM,
            "i" => HTMLTag::I,
            "code" => HTMLTag::CODE,
            "pre" => HTMLTag::PRE,
            "blockquote" => HTMLTag::BLOCKQUOTE,
            "cite" => HTMLTag::CITE,
            "abbr" => HTMLTag::ABBR,
            "time" => HTMLTag::TIME,
            "data" => HTMLTag::DATA,
            "progress" => HTMLTag::PROGRESS,
            "meter" => HTMLTag::METER,
            "details" => HTMLTag::DETAILS,
            "summary" => HTMLTag::SUMMARY,
            "dialog" => HTMLTag::DIALOG,
            "canvas" => HTMLTag::CANVAS,
            "svg" => HTMLTag::SVG,
            "math" => HTMLTag::MATH,
            "template" => HTMLTag::TEMPLATE,
            "slot" => HTMLTag::SLOT,
            "output" => HTMLTag::OUTPUT,
            "fieldset" => HTMLTag::FIELDSET,
            "legend" => HTMLTag::LEGEND,
            "datalist" => HTMLTag::DATALIST,
            "optgroup" => HTMLTag::OPTGROUP,
            "small" => HTMLTag::SMALL,
            "area" => HTMLTag::AREA,
            "col" => HTMLTag::COL,
            "colgroup" => HTMLTag::COLGROUP,
            "caption" => HTMLTag::CAPTION,
            "thead" => HTMLTag::THEAD,
            "tbody" => HTMLTag::TBODY,
            "tfoot" => HTMLTag::TFOOT,
            _ => HTMLTag::UNKNOWN
        }
    }

    pub fn to_modifers(&self) -> Vec<Modifier> {
        match self {
            HTMLTag::BOLD |
            HTMLTag::STRONG |
            HTMLTag::H1 |
            HTMLTag::H2 |
            HTMLTag::H3 |
            HTMLTag::H4 |
            HTMLTag::H5 |
            HTMLTag::H6 => vec![Modifier::BOLD],

            HTMLTag::EM | HTMLTag::I => vec![Modifier::ITALIC],

            HTMLTag::U | HTMLTag::A => vec![Modifier::UNDERLINED],

            HTMLTag::S | HTMLTag::DEL => vec![Modifier::CROSSED_OUT],

            _ => vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HTMLElement {
    pub tag: HTMLTag,
    pub attributes: HashMap<String, String>,
    pub closing: bool
}

impl HTMLElement {
    pub fn new(tag: HTMLTag, attributes: HashMap<String, String>, closing: bool) -> Self {
        HTMLElement {
            tag,
            attributes,
            closing
        }
    }
}
