use std::io;
use std::io::prelude::*;

fn main() {
    for result in wsTokenizer(io::stdin()).collect::<Vec<Vec<String>>>() {
        for string in result {
            println!("{}", string);
        }
    }
}

// tokenization

// whiteSpace tokenizer

fn wsTokenizer(stream: std::io::Stdin) -> wsTokenizer {
    let mut lines: Vec<String> = Vec::new();
    for line in stream.lock().lines() {
        if let Ok(line) = line {
        lines.push(line);
        }
    }
    lines.reverse();
    wsTokenizer {strings: lines}
}

struct wsTokenizer {
    strings: Vec<String>,
}

// returns a vector for each string of the given vector
impl Iterator for wsTokenizer {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Vec<String>> {
        if self.strings.is_empty() {
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
//impl Iterator for brTokenizer {}

// function for returning a vector of whitespace-splitted words for a given string
fn split_string(buf: &str) -> (Vec<&str>) {
    return buf.split_whitespace().collect();
}

// normalization to lowercase (vec<String> to x strings)
struct ToLowerCase {
    strings: Vec<String>,
}

impl Iterator for ToLowerCase {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.strings.is_empty() {
            None
        } else {
            Some(self.strings.pop().unwrap().to_lowercase())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::split_string;


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

    /*
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
        assert_eq!(iter.next().unwrap(),
                   vec!["Das".to_owned(), "ist".to_owned(), "ein".to_owned(), "Auto".to_owned(), ]);
        assert_eq!(iter.next().unwrap(),
                   vec!["Das".to_owned(), "ist".to_owned(), "ein".to_owned(), "Boot".to_owned(), ]);
        assert_eq!(iter.next().unwrap(),
                   vec!["Das".to_owned(), "ist".to_owned(), "ein".to_owned(), "Haus".to_owned(), ]);
        assert_eq!(iter.next(), None);
    }*/
}
