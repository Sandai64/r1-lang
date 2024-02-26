// SmokeScript lexer module

use std::fmt::format;

#[derive(Debug)]
pub enum NonTerminal
{
    Plus,
    Minus,
    Mul,
    Div,
    Equals,

    Newline,
    Number,
    Dot,
    StringBorder,
    StatementEnd,
    BlockOpen,
    BlockClose,
    ParenOpen,
    ParenClose,
}

impl NonTerminal
{
    fn try_from_char(c: char) -> Option<NonTerminal>
    {
        use NonTerminal::*;
        let nonterm: NonTerminal = match c {
            '{' => BlockOpen,
            '}' => BlockClose,
            '(' => ParenOpen,
            ')' => ParenClose,

            '0'..='9' => Number,
            '+' => Plus,
            '-' => Minus,
            '/' => Div,
            '*' => Mul,
            '=' => Equals,

            ';' => StatementEnd,
            '"' => StringBorder,
            '.' => Dot,

            _ => return None,
        };

        Some(nonterm)
    }
}

#[derive(Debug)]
enum FlowChar
{
    Whitespace,
    Newline,
}

impl FlowChar
{
    fn try_from_char(c: char) -> Option<FlowChar>
    {
        use FlowChar::*;
        let flowchar: FlowChar = match c
        {
            '\n' => Newline,
            ' ' => Whitespace,

            _ => return None,
        };

        Some(flowchar)
    }
}

#[derive(Debug)]
pub enum NumberType
{
    Integer(String),
    Float(String),
}

#[derive(Debug)]
pub enum Token
{
    Symbol(NonTerminal),
    Word(String),
    Number(NumberType),
    String(String),
}

pub struct Lexer
{
    orig: String,
    txt: Vec<char>,
    pos: usize,
}

impl Lexer
{
    pub fn new(code: &str) -> Self
    {
        let txt: Vec<char> = code.chars().rev().collect();
        let pos: usize = txt.len() - 1;
        let orig: String = code.to_string();

        Self { txt, pos, orig }
    }

    pub fn next(&mut self) -> Option<char>
    {
        if self.pos == 0
        {
            return None;
        }

        self.pos -= 1;
        self.txt.pop()
    }

    fn peek(&self) -> Option<char>
    {
        if self.pos == 0
        {
            return None;
        }

        Some(self.txt[self.pos])
    }

    fn eat(&mut self)
    {
        let _ = self.next();
    }

    fn expect(&mut self, exp: char)
    {
        todo!();
        match self.next()
        {
            Some(got) if got == exp => (),
            Some(got) => report_error(
                format!("smklex: expected [{}] but got [{}]", exp, got),
                &self,
            ),
            None => report_error(
                format!("smklex: expected [{}] but found nothing.", exp),
                &self,
            ),
        }
    }

    fn read_word(&mut self) -> String
    {
        let mut word: String = String::new();
        while let Some(c) = self.peek()
        {
            match FlowChar::try_from_char(c)
            {
                Some(_) => break,
                None => (),
            };

            match NonTerminal::try_from_char(c)
            {
                Some(_) => break,
                None => (),
            };

            word.push(self.next().unwrap());
        }

        word
    }
}
