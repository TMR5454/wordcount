//! wordcount is simple counter of characters, words and lines.
//! In detail, refer the [`count`](fn.count.html) function document.
#![warn(missing_docs)]

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// Options for [`count`](fn.count.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// by Unicode 1 word
    Char,
    /// by word
    Word,
    /// by line
    Line,
}

// CountOption default value is [`Word`](enum.CountOption.html"variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// read 1-line as UTF-8 string and count frequency.
/// 
/// Target of counting frequency is depended by option.
/// * [`CountOptino::Char](enum.CountOption.html#variant.Char): Unicode 1 word
/// * [`CountOptino::Word](enum.CountOption.html#variant.Word): matching "\w+" as regex
/// * [`CountOptino::Line](enum.CountOption.html#variant.Line): 1 line (separated by \n or \r\n)
/// 
/// # Panics
/// 
/// If input strings is not encoded as UTF8
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }

    freqs
}
