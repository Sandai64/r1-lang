#[cfg(test)]

use crate::lexer;

#[test]
fn initialize_lexer()
{
    let test_source: &str = r#"
    fn main()
    {
        let mut rt = Runtime::new();

        async fut
        {
            let io = Reactor::new_io();
            let data: u32 = io.get().await?;
            let res = data + 100;
            println!("{}", data);
        }

        rt.block_on(fut).unwrap();
    }"#;

    let lexer: lexer::Lexer = lexer::Lexer::new(test_source);
    let tokens: Vec<lexer::Token> = lexer.lex();

    println!("{:?}", tokens);
}
