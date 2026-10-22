/// Very naive implementation of FizzBuzz
pub fn fizz_buzz1(i: u32) -> String {
    if i % 3 == 0 {
        if i % 5 == 0 {
            return "FizzBuzz".to_owned()
        } else {
            return "Fizz".to_owned()
        }
    } else if i % 5 == 0 {
        return "Buzz".to_owned()
    } else {
        return format!("{i}")
    }
}

pub fn fizz_buzz2(i: u32) -> String {
    match (i % 3 == 0, i % 5 == 0) {
        (true, true) => return "FizzBuzz".to_owned(),
        (true, false) => return "Fizz".to_owned(),
        (false, true) => return "Buzz".to_owned(),
        (false, false) => return i.to_string(),
    }
}

// un fel de enum
pub fn fizz_buzz(i: u32) -> u8 {
    match (i % 3 == 0, i % 5 == 0) {
        (true, true) => return 1,          // cod for FizzBuzz
        (true, false) => return 2,         // cod for Fizz
        (false, true) => return 3,         // cod for Buzz
        (false, false) => return 4,        // cod for number
    }
}

// TODO Write a unit test, using the contents of `fizzbuzz.out` file
// to compare.
// You can use the `include_str!()` macro to include file
// contents as `&str` in your artifact.

/*
to do : open fizzbuzz.out and put its contents in a map using include_str!()
display on the terminal each item on a new line using &str
*/

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_fizz_buzz() {

        
        // deschide fisierul fizzbuz.out
        let expected_output = include_str!("../fizzbuzz.out");

        for (i, line) in expected_output.lines().enumerate() {

            // let nr:u32 = line.parse::<u32>().expect("FAIL");
            let result:u8 = fizz_buzz((i + 1) as u32);
            
            // if result != line {
            //     eprintln!("Test failed at line {}: Output: {} | Expected: {}",
            //         i + 1, result, line);


            //     eprintln!("");     // new line

            //     // if it enters this, it doesn't pass the test
            // }

            // println!("line {} is good", line);

            let mut ans:u8 = 0;
            if let Some(first_char) = line.chars().next() {
                match first_char {
                    '1' => ans = 1,
                    '2' => ans = 2,
                    '3' => ans = 3,
                    '4' => ans = 4,
                    _ => ans = 0,
                }
            } else {
                ans = 0;
                println!("naspa ba.");
            }

            assert_eq!(result , ans); 

        }

    
    }
}
