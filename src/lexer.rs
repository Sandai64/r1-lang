// SmokeScript lexer module

const STRING_BORDER: char = '"';

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
        let nonterm: NonTerminal = match c
        {
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
            '.' => Dot,

            STRING_BORDER => StringBorder,

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

    fn read_string(&mut self) -> String
    {
        let mut string: String = String::new();
        while let Some(c) = self.peek()
        {
            match NonTerminal::try_from_char(c)
            {
                Some(NonTerminal::StringBorder) => break,
                _ => (),
            };

            string.push(self.next().unwrap());
        }

        string
    }

    fn read_number(&mut self) -> NumberType
    {
        let mut number: String = String::new();
        let mut is_float: bool = false;

        while let Some(c) = self.peek()
        {
            match FlowChar::try_from_char(c)
            {
                Some(_) => break,
                None => ()
            };

            match NonTerminal::try_from_char(c)
            {
                Some(NonTerminal::Number) => (),
                Some(NonTerminal::Dot) => is_float = true,
                _ => break,
            };

            number.push(self.next().unwrap());
        }

        if is_float
        {
            return NumberType::Float(number)
        }

        NumberType::Integer(number)
    }

    pub fn lex(mut self) -> Vec<Token>
    {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = self.peek()
        {
            match FlowChar::try_from_char(c)
            {
                Some(FlowChar::Newline) =>
                {
                    tokens.push(Token::Symbol(NonTerminal::Newline));
                    self.eat();
                    continue;
                },

                Some(_) =>
                {
                    self.eat();
                    continue;
                },

                None => (),
            };

            let nonterm: NonTerminal = match NonTerminal::try_from_char(c)
            {
                Some(nt) => nt,
                None =>
                {
                    tokens.push(Token::Word(self.read_word()));
                    continue;
                }
            };

            match nonterm
            {
                NonTerminal::Number => tokens.push(Token::Number(self.read_number())),
                NonTerminal::StringBorder =>
                {
                    tokens.push(Token::Symbol(NonTerminal::StringBorder));
                    self.expect(STRING_BORDER);
                    tokens.push(Token::String(self.read_string()));
                    tokens.push(Token::Symbol(NonTerminal::StringBorder));
                    self.expect(STRING_BORDER);
                },

                _ =>
                {
                    tokens.push(Token::Symbol(nonterm));
                    self.eat();
                }
            }
        }

        tokens
    }
}

fn report_error(error: String, lexer: &Lexer)
{
    const ERR_LEFT: usize  = 15;
    const ERR_RIGHT: usize = 15;

    let mut txt: String = String::new();
    let pos: usize = lexer.orig.len() - lexer.pos - 1;

    // What the fuck ?
    let end: usize = if pos + ERR_RIGHT > lexer.orig.len()
    {
        lexer.orig.len()
    }
    else
    {
        pos + ERR_RIGHT
    };

    let start: usize = if pos < ERR_LEFT
    {
        0
    }
    else
    {
        pos - ERR_LEFT
    };

    println!("pos: {}, len: {}", lexer.pos, lexer.orig.len());
    let slice: &str = &lexer.orig[start..end];

    txt.push_str(&error);
    txt.push('\n');
    txt.push('\n');

    let mut marker: String = String::new();

    for (i,c) in slice.chars().enumerate()
    {
        if c == '\n'
        {
            break;
        }

        txt.push(c);

        if start + i < pos
        {
            marker.push('-');
        }
    }

    txt.push('\n');
    marker.pop();
    marker.push('^');
    txt.push_str(&marker);

    panic!("{}\n", txt);
}
