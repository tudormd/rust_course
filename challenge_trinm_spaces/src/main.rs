fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = " There's space in front.      ";
    assert_eq!(trim_spaces(test2), "There's space in front.");

    let test3 = "There's space in the rear. ";
    assert_eq!(trim_spaces(test3), "There's space in the rear.");

    let test4 = " We're surrounded by space!  ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "    ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 2 ";
    assert_eq!(trim_spaces(test7), "2");

}

 fn trim_spaces(s: &str) -> &str {
    let mut start = 0;

     for (index,  character) in s.chars().enumerate(){
         if character != ' ' {
             start  = index;
             break;
         }
     }

     let mut end = 0;

     for (index,  character) in s.chars().rev().enumerate(){
         print!("{}", character);
         if character != ' ' {
             end  = s.len() - index;
             break;
         }
     }

     &s[start..end]
 }
