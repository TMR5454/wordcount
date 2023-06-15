use wordcount::{count,CountOption};
use std::collections::HashMap;

#[test]
fn word_count_works() {
    use std::io::Cursor;

    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);

    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
}

#[test]
fn word_count_works2() {
    use std::io::Cursor;
    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("cc".to_string(), 1);
    exp.insert("dd".to_string(), 1);

    assert_eq!(count(Cursor::new("aa  cc dd"), CountOption::Word), exp);
}

#[test]
fn word_count_works3() {
    use std::io::Cursor;
    let freqs = count(Cursor::new("aa  cc dd"), CountOption::Word);

    assert_eq!(freqs.len(), 3);
    assert_eq!(freqs["aa"], 1);
    assert_eq!(freqs["cc"], 1);
    assert_eq!(freqs["dd"], 1);
}

