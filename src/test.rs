use execute::run;

macro_rules! test_eq {
   ($text:expr, $expected:expr) => {
      let mut text = "#[include(*Plugins, *Operators, *Builtins)]\n".to_string();
      text.push_str($text);
      assert_eq!(run_test(&text).as_str(), $expected);
   }
}

#[allow(dead_code)]
fn run_test(text: &str) -> String {
   if let Some(obj) = run(text).stack.pop(){
      obj.to_repr()
   } else {
      String::new()
   }
}

#[test]
fn nothing() {
   /* nothing */
   test_eq!("", ""); //no text should do nothing
}

#[test]
fn whitespace() {
   /* single characters */
   test_eq!("\n", "");
   test_eq!("\t", "");
   test_eq!(" ", "");
   test_eq!("\r", "");
   /* random characters put together */

   test_eq!("\n\t \r", "");
}

#[test]
fn comments() {
   /* comments */
   test_eq!("// basic comment", "");
   test_eq!("//\r try to override the comment", "");
   test_eq!("// multiline\n // comment", "");
   test_eq!("//", ""); // comment with nothing after it
}
#[test]
fn constants(){
   test_eq!("null", "null");
   test_eq!("nil", "null");
   test_eq!("none", "null");
   test_eq!("true", "true");
   test_eq!("false", "false");
   test_eq!("NEG_1", "-1");
}
#[test]
fn types(){
   test_eq!("'a'", "'a'");
}

#[test]
fn operators() {
   /* test single operators */
   test_eq!("1 + 2",  "3");
   test_eq!("1 - 2", "-1");
   test_eq!("2 * 3",  "6");
   test_eq!("9 / 3",  "3");
   test_eq!("9 / 2",  "4");
   test_eq!("9 % 2",  "1");
   test_eq!("10 % 4", "2");
   test_eq!("NEG_1 * 9 % 4", "-1");

   /* compound statements */
   test_eq!("2 + 3 * 4", "14");
   test_eq!("2 * 3 + 4", "10");
   test_eq!("18 / 5", "3");
   test_eq!("2 - 3 * 18 / 5", "-8");

   /* introduce parenthesis */
   test_eq!("(1-2)", "-1");
   test_eq!("(2 * 5)", "10");
   test_eq!("3 + (4 * 5)", "23");
   test_eq!("(3 + 4) * 5", "35");
   test_eq!("5*(8/(3 - 4)) % 6", "-4");
}

#[test]
fn assignment(){
   /* simple */
   test_eq!("a = 3", "3");
   test_eq!("a = 3;", "");
   test_eq!("a = 3; a", "3");
   /* assign to non-symbols */
   test_eq!("'a' = 3", "3");
   test_eq!("'a' = 3;", "");
   /* use of `?` */
   test_eq!("'a' = 3; 'a'", "'a'");
   test_eq!("'a' = 3; 'a'?", "3");
   test_eq!("0 = 3; 0", "0");
   test_eq!("0 = 3; 0?", "3");
}















