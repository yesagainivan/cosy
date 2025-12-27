/// Calculate the Levenshtein distance between two strings.
///
/// Returns the number of edits (insertions, deletions, substitutions)
/// required to transform `a` into `b`.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a;
    }

    let mut dp = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        dp[i][0] = i;
    }
    for j in 0..=len_b {
        dp[0][j] = j;
    }

    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            dp[i + 1][j + 1] = std::cmp::min(
                dp[i][j + 1] + 1, // deletion
                std::cmp::min(
                    dp[i + 1][j] + 1, // insertion
                    dp[i][j] + cost,  // substitution
                ),
            );
        }
    }

    dp[len_a][len_b]
}

/// Find the best match for `target` in `candidates` within `max_dist`.
pub fn find_best_match(target: &str, candidates: &[String], max_dist: usize) -> Option<String> {
    let mut best_dist = max_dist + 1;
    let mut best_match = None;

    for candidate in candidates {
        let dist = levenshtein(target, candidate);
        if dist <= max_dist && dist < best_dist {
            best_dist = dist;
            best_match = Some(candidate.clone());
        }
    }

    best_match
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein() {
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("book", "back"), 2);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("abc", ""), 3);
        assert_eq!(levenshtein("same", "same"), 0);
    }

    #[test]
    fn test_best_match() {
        let candidates = vec!["port".to_string(), "host".to_string(), "debug".to_string()];

        assert_eq!(
            find_best_match("prt", &candidates, 2),
            Some("port".to_string())
        );
        assert_eq!(
            find_best_match("pot", &candidates, 2),
            Some("port".to_string())
        );
        assert_eq!(
            find_best_match("hst", &candidates, 2),
            Some("host".to_string())
        );

        // Too far
        assert_eq!(find_best_match("xyz", &candidates, 2), None);
    }
}
