use std::collections::HashSet;
/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let filtered: HashSet<u8> = sentence
            .to_lowercase()
            .bytes()
            .filter(|&c| c.is_ascii_alphabetic())
            .collect();
    let alphabet_set: HashSet<u8> = HashSet::from_iter(b'a'..=b'z');
    filtered == alphabet_set
}

/// Determine whether a sentence is a pangram.
pub fn is_pangram_set_operation(sentence: &str) -> bool {
    let filtered: HashSet<u8> = sentence
            .to_lowercase()
            .bytes()
            .filter(|&c| c.is_ascii_alphabetic())
            .collect();
    let alphabet_set: HashSet<u8> = HashSet::from_iter(b'a'..=b'z');
    filtered == alphabet_set
}

/// Determine whether a sentence is a pangram.
pub fn is_pangram_array_based(sentence: &str) -> bool {
    let mut seen = [false; 26]; // array to track which letters have been seen
    let mut count = 0; // count of unique letters seen

    for byte in sentence.bytes() {
        if byte.is_ascii_alphabetic() {
            let index = (byte.to_ascii_lowercase() - b'a') as usize;
            if !seen[index] {
                seen[index] = true;
                count += 1;
                if count == 26 {
                    return true; // all letters have been seen
                }
            }
        }
    }

    false // not all letters have been seen
}
