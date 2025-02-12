/// LeetCode 1768: Merge Strings Alternately
pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut res = String::with_capacity(word1.len() + word2.len());

    let mut chars1 = word1.chars();
    let mut chars2 = word2.chars();

    for _ in 0..(word1.len().min(word2.len())) {
        res.push(chars1.next().unwrap());
        res.push(chars2.next().unwrap());
    }

    res.extend(chars1);
    res.extend(chars2);

    res
}

#[cfg(test)]
mod test {
    use super::merge_alternately;

    #[test]
    fn cases() {
        assert_eq!(merge_alternately("abc".into(), "pqr".into()), "apbqcr");
        assert_eq!(merge_alternately("ab".into(), "pqrs".into()), "apbqrs");
        assert_eq!(merge_alternately("abcd".into(), "pq".into()), "apbqcd");
    }
}
