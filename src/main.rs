/* SmokeScript : memory-safe & fast interpreted programming language
 * ------------------------------------------------------
 * Please refer to the LICENSE file for more information
 * about distribution and modification.
 * [Master's thesis / Software Architecture & Language Theory]
 * ------------------------------------------------------
*/

// Disable this at one point
#![allow(unused)]

mod lexer;

fn run(source: String) {}

fn report(line: u32, error: &str, message: &str)
{
  println!("[line {}] Error {}: {}", line, error, message);
}

fn error(had_error_ref: &mut bool, line: u32, message: &str)
{
  report(line, "", message);
  *had_error_ref = true;
}

fn main()
{
  let mut had_error: bool = false;

}
