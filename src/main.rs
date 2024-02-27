/* SmokeScript : memory-safe & fast interpreted programming language
* ------------------------------------------------------
* Please refer to the LICENSE file for more information
* about distribution and modification.
* [Master's thesis / Software Architecture & Language Theory]
* ------------------------------------------------------
*/

mod lexer;
mod tests;

fn lex_simple() -> ()
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

fn main()
{
    println!("{} interpreter v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("Erwan EGASSE - Master's thesis on Software Architecture and Language Theory");
    println!("\n-----------------\n");

    lex_simple();
}
