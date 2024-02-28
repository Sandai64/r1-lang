/* SmokeScript : memory-safe & fast interpreted programming language
* ------------------------------------------------------
* Please refer to the LICENSE file for more information
* about distribution and modification.
* [Master's thesis / Software Architecture & Language Theory]
* ------------------------------------------------------
*/

use std::time::{Duration, Instant};

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

    let start_time: Instant = Instant::now();

    let lexer: lexer::Lexer = lexer::Lexer::new(test_source);
    let tokens: Vec<lexer::Token> = lexer.lex();

    let time_diff: Duration = start_time.elapsed();
    println!("Successfully lex()-ed source in {} microseconds ({} ms)", time_diff.as_micros(), time_diff.as_micros() as f64 / 1000.0);

    println!("\nAnalyzed tokens -------------------\n{:?}", tokens);
}

fn main()
{
    lex_simple();
}
