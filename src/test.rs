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
      obj.to_string()
   } else {
      String::new()
   }
}

#[test]
fn test_operators() {
   /* test single operators */
   test_eq!("1 + 2",  "3");
   test_eq!("1 - 2", "-1");
   test_eq!("2 * 3",  "6");
   test_eq!("9 / 3",  "3");
   test_eq!("9 / 2",  "4");
   /* compound statements */
   test_eq!("2 + 3 * 4", "14");
   test_eq!("2 * 3 + 4", "10");
   test_eq!("18 / 5", "3");
   test_eq!("2 - 3 * 18 / 5", "-8");
   /* introduce parenthesis */
   // test_eq!("(1-2)!.0", "-1");
   test_eq!("(1-2)", "-1");

}














