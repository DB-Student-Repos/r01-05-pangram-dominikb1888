use std::collections::HashSet;
/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let hashed = HashSet::from(sentence);
    let alphabet = HashSet::from((b'a'..=b'z').collect()).unwrap();
    hashed == alphabet

}
