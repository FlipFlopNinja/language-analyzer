
use std::io::BufRead;
use std::io;

fn main() {
    let stdin = io::stdin();
    // For each line from stdin
    for line in stdin.lock().lines() {
        // Check if the line was correctly read
        if let Ok(line) = line {
            // Split the line at every whitespace
            // Then Print the terms
            // Iterators in rust are lazy. So the count at the end is only to consume the iterator!
            line.split_whitespace().inspect(|term| println!("{}", term)).count();
        }
    }

}

// tokenization

// function for returning a vector of whitespace-splitted words for a given string
fn split_string(buf: &str) -> (Vec<&str>) {
    return buf.split_whitespace().collect();
}

// struct for iterator
struct Tokenizer {
    strings: Vec<String>,
}

// returns a vector for each string of the given vector
impl Iterator for Tokenizer {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Vec<String>> {
        if self.strings.len() <= 0 {
            None
        } else {
            // 1. get the last element of the vec "strings" and remove it
            // 2. unwrap returned value to get String
            // 3. split_whitespace to get rid of whitespaces, returns iterator
            // 4. iterator values getting owned (&str -> String)
            // 5. collects all values and returns them in an Vec (Vec<String>)
            Some(self.strings.pop().unwrap().split_whitespace().map(ToOwned::to_owned).collect())
        }
    }
}


// normalization to lowercase (vec<String> to x strings)
struct ToLowerCase {
    strings: Vec<String>,
}

impl Iterator for ToLowerCase {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.strings.len() <= 0 {
            None
        } else {
            Some(self.strings.pop().unwrap().to_lowercase())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::split_string;
    use super::Tokenizer;


    #[test]
    fn basic_test() {
        // use asserts to check your results
        // Use assert to check for a true value
        assert!(true);
        // Or with an expression
        assert!(1 == 1);
        // Or use assert_eq! for a check of equality
        assert_eq!(1, 1);
    }

    #[test]
    fn split_test() {
        let test: Vec<&str> = split_string("Das ist ein Haus");
        assert_eq!(test, ["Das", "ist", "ein", "Haus"]);
    }

    #[test]
    fn tokenizer_test() {
        let mut test = vec!["Das ist ein Haus".to_owned(),
                            "Das ist ein Boot".to_owned(),
                            "Das ist ein Auto".to_owned()];
        let result = (Tokenizer { strings: test }).collect::<Vec<Vec<String>>>();
        for stringvec in result {
            for string in stringvec {
                for char in string.chars() {
                    assert_eq!(char.is_whitespace(), false);
                }
            }
        }
    }

    #[test]
    fn failing_test() {
        let mut test = vec!["Das ist ein! Haus.".to_owned(),
                            "Das ist ein! Boot.".to_owned(),
                            "Das ist ein! Auto.".to_owned()];
        let mut iter = Tokenizer { strings: test };
        // let result = iter.collect::<Vec<Vec<String>>>();
        assert_eq!(iter.next().unwrap(),
                   vec!["Das".to_owned(), "ist".to_owned(), "ein".to_owned(), "Auto".to_owned(), ]);
        assert_eq!(iter.next().unwrap(),
                   vec!["Das".to_owned(), "ist".to_owned(), "ein".to_owned(), "Boot".to_owned(), ]);
        assert_eq!(iter.next().unwrap(),
                   vec!["Das".to_owned(), "ist".to_owned(), "ein".to_owned(), "Haus".to_owned(), ]);
        assert_eq!(iter.next(), None);
    }
}
