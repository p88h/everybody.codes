use std::collections::HashMap;

pub struct MultiStringMatcher {
    _patterns: Vec<Vec<u8>>,
    // code is a pair of length and 'hash' of the pattern
    codes: HashMap<u64, usize>,
    max_len: usize,
}

impl MultiStringMatcher {
    pub fn new(patterns: Vec<&str>) -> Self {
        let patterns_bytes: Vec<Vec<u8>> = patterns.iter().map(|&s| s.as_bytes().to_vec()).collect();
        let codes: HashMap<u64, usize> = patterns_bytes.iter().enumerate().map(|(i, p)| (Self::encode(p), i)).collect();
        let max_len = patterns_bytes.iter().map(|p| p.len()).max().unwrap_or(0);
        MultiStringMatcher { _patterns: patterns_bytes, codes, max_len }
    }

    fn encode(s: &Vec<u8>) -> u64 {
        let mut h: u64 = 0;
        for &b in s {
            h = h << 8 | (b as u64);
        }
        h
    }

    pub fn find_all_matches(&self, text: &[u8]) -> Vec<(usize, usize)> {
        let mut results = vec![];
        let mut h: u64 = !0;
        for (pos, c) in text.iter().enumerate() {
            h = (h << 8) | (*c as u64);
            let mut mask = 0xFFu64;
            for len in 0..self.max_len {
                let code = h & mask;
                if self.codes.contains_key(&code) {
                    results.push((pos - len, len + 1));
                }
                mask = (mask << 8) | 0xFF;
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multi_string_matcher() {
        let patterns = vec!["he", "she", "his", "hers"];
        let matcher = MultiStringMatcher::new(patterns);
        let text = "ushers";
        let count = matcher.find_all_matches(text.as_bytes()).len();
        assert_eq!(count, 3); // "she", "he", "hers"
    }
}