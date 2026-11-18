use std::{collections::HashMap, str::Chars};

use crate::tuihtml::html::{HTMLElement, HTMLTag};

// Enum for the different HTML tags
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Element(HTMLElement),
    Text(String),
    NewLine,
    EOF,
}

impl Token {
    pub fn get_html_element(token: &Token) -> Option<HTMLElement> {
        match token {
            Token::Element(e) => Some(e.clone()),
            Token::Text(_) => None,
            Token::NewLine => None,
            Token::EOF => None,
        }
    }

    pub fn get_text(token: &Token) -> Option<String> {
        match token {
            Token::Element(_) => None,
            Token::Text(s) => Some(s.clone()),
            Token::NewLine => None,
            Token::EOF => None,
        }
    }

    pub fn is_whitespace(token: &Token) -> bool {
        match token {
            Token::Text(s) => {
                for char in s.chars() {
                    if !char.is_whitespace() {
                        return false;
                    }
                }
                return true
            }
            _ => false
        }
    }
}

#[derive(Debug, Clone)]
pub struct HTMLTokenizer<'a> {
    html_pos: Chars<'a>,
    next_char: char,
}

impl<'a> HTMLTokenizer<'a> {

    pub fn new(html: Chars<'a>) -> HTMLTokenizer<'a> {
        HTMLTokenizer {
            html_pos: html,
            next_char: '\0',
        }
    }

    pub fn init(&mut self) {
        self.next_char = self.html_pos.next().unwrap();
        self.consume_whitespace();
    }

    pub fn next(&mut self) -> Token  {
        let mut lexeme = String::new();

        while self.html_pos.clone().peekable().peek().is_some() {
            match self.next_char {
                '<' => {
                    if lexeme.is_empty() {
                        self.next_char();
                        return Token::Element(self.capture_element())
                    }
                    // if is_only_whitespace(&lexeme) {
                    //     lexeme.clear();
                    // }
                    return Token::Text(lexeme);
                },
                '\n' | '\r' => {
                    if lexeme.is_empty() {
                        self.consume_whitespace();
                        return Token::NewLine;
                    }
                    return Token::Text(lexeme);
                },
                _ => {}
            }

            lexeme.push(self.next_char);
            self.next_char();
        }

        if lexeme.is_empty() {
            return Token::EOF;
        }

        lexeme.push(self.next_char);
        return Token::Text(lexeme);
    }

    fn capture_element(&mut self) -> HTMLElement {
        let mut closing_tag = false;
        let mut tag = String::new();
        let mut attributes = HashMap::new();

        while self.html_pos.clone().peekable().peek().is_some() {

            if self.next_char.is_whitespace() && !closing_tag {
                self.consume_whitespace();
                attributes = self.capture_tag_attributes();
            }

            match self.next_char {
                '/' => {
                    closing_tag = true;
                    self.next_char();
                },
                '>' => {
                    self.next_char();
                    if self.next_char == '\n' {
                        self.consume_whitespace();
                    }
                    return HTMLElement::new(HTMLTag::from_string(tag), attributes, closing_tag);
                },
                _ => {}
            }

            tag.push(self.next_char);
            self.next_char();
        }

        return HTMLElement::new(HTMLTag::from_string(tag), attributes, closing_tag);

    }

    fn capture_tag_attributes(&mut self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        let mut key = String::new();
        let mut value = String::new();

        enum AttributeState {
            KEY,
            VALUE
        }

        let mut state = AttributeState::KEY;

        while self.html_pos.clone().peekable().peek().is_some() {

            if self.next_char == '>' {
                return attributes;
            }

            match state {
                AttributeState::KEY => {
                    if self.next_char == '=' {
                        state = AttributeState::VALUE;
                        self.next_char();
                        self.consume_whitespace();
                        self.next_char();
                    }
                    else {
                        key.push(self.next_char);
                        self.next_char();
                    }
                },
                AttributeState::VALUE => {
                    if self.next_char == '\"' {
                        state = AttributeState::KEY;
                        attributes.insert(key.clone(), value.clone());
                        self.next_char();
                        self.consume_whitespace();
                        key = String::new();
                        value = String::new();
                    }
                    else {
                        value.push(self.next_char);
                        self.next_char();
                    }
                }
            }
        }

        return attributes;
    }

    fn next_char(&mut self) {
        if self.html_pos.clone().peekable().peek().is_some() {
            self.next_char = self.html_pos.next().unwrap();
        }
    }

    fn consume_whitespace(&mut self) {
        while self.next_char.is_whitespace() && self.html_pos.clone().peekable().peek().is_some() {
            self.next_char();
        }
    }
}

fn is_only_whitespace(string: &String) -> bool {
    let mut is_whitespace = true;

    for char in string.chars() {
        if !char.is_whitespace() {
            is_whitespace = false;
        }
    }

    is_whitespace
}

