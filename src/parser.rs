use std::collections::BTreeMap;

/// Parse the contents of a `.env` file into ordered-by-key pairs.
///
/// Handles blank lines, `#` comments, an optional `export ` prefix, single and
/// double quoted values, and trailing ` # comments` on unquoted values.
pub fn parse_env(input: &str) -> BTreeMap<String, String> {
    let mut vars = BTreeMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let line = line
            .strip_prefix("export ")
            .map(str::trim_start)
            .unwrap_or(line);

        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        if key.is_empty() {
            continue;
        }

        vars.insert(key.to_string(), parse_value(value.trim()));
    }

    vars
}

fn parse_value(raw: &str) -> String {
    for quote in ['"', '\''] {
        if let Some(rest) = raw.strip_prefix(quote) {
            if let Some(end) = rest.find(quote) {
                return rest[..end].to_string();
            }
        }
    }

    match raw.find(" #") {
        Some(idx) => raw[..idx].trim_end().to_string(),
        None => raw.to_string(),
    }
}

/// One row of the comparison table: a key and the value from each source,
/// in source order. `None` means the source does not define the key.
#[derive(Debug, PartialEq, Eq)]
pub struct Row {
    pub key: String,
    pub values: Vec<Option<String>>,
}

impl Row {
    /// True when the sources do not all agree (including a key missing from some).
    pub fn differs(&self) -> bool {
        self.values.windows(2).any(|pair| pair[0] != pair[1])
    }
}

impl Row {
    /// Split the value at `source` into runs of `(text, differs)`, where `differs`
    /// marks characters that don't match the same position in some other source
    /// that defines this key (so `O` vs `0` stands out).
    pub fn diff_runs(&self, source: usize) -> Vec<(String, bool)> {
        let Some(value) = &self.values[source] else {
            return Vec::new();
        };
        let others: Vec<Vec<char>> = self
            .values
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != source)
            .filter_map(|(_, v)| v.as_ref().map(|v| v.chars().collect()))
            .collect();

        let mut runs: Vec<(String, bool)> = Vec::new();
        for (i, ch) in value.chars().enumerate() {
            let differs = others.iter().any(|other| other.get(i) != Some(&ch));
            match runs.last_mut() {
                Some((text, flag)) if *flag == differs => text.push(ch),
                _ => runs.push((ch.to_string(), differs)),
            }
        }
        runs
    }
}

/// Merge several parsed env files into rows keyed by the union of all keys.
pub fn compare(sources: &[BTreeMap<String, String>]) -> Vec<Row> {
    let mut keys: Vec<&String> = sources.iter().flat_map(|s| s.keys()).collect();
    keys.sort();
    keys.dedup();

    keys.into_iter()
        .map(|key| Row {
            key: key.clone(),
            values: sources.iter().map(|s| s.get(key).cloned()).collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_pairs_and_skips_noise() {
        let vars =
            parse_env("# comment\n\nAPI_KEY=abc\nexport SECRET = def \nnot a pair\n=nokey\n");
        assert_eq!(vars.len(), 2);
        assert_eq!(vars["API_KEY"], "abc");
        assert_eq!(vars["SECRET"], "def");
    }

    #[test]
    fn handles_quotes_and_inline_comments() {
        let vars = parse_env("A=\"hello world\" # note\nB='x=y'\nC=plain # trailing\nD=a#b\n");
        assert_eq!(vars["A"], "hello world");
        assert_eq!(vars["B"], "x=y");
        assert_eq!(vars["C"], "plain");
        assert_eq!(vars["D"], "a#b");
    }

    #[test]
    fn value_may_contain_equals() {
        assert_eq!(
            parse_env("URL=https://x.com/?a=1")["URL"],
            "https://x.com/?a=1"
        );
    }

    #[test]
    fn diff_runs_flag_mismatched_characters() {
        let rows = compare(&[parse_env("K=ab0d\n"), parse_env("K=abOd\n"), parse_env("")]);
        assert_eq!(
            rows[0].diff_runs(0),
            vec![
                ("ab".into(), false),
                ("0".into(), true),
                ("d".into(), false)
            ]
        );
        assert_eq!(rows[0].diff_runs(1)[1], ("O".to_string(), true));
        assert!(rows[0].diff_runs(2).is_empty());
    }

    #[test]
    fn diff_runs_flag_length_differences() {
        let rows = compare(&[parse_env("K=abc\n"), parse_env("K=ab\n")]);
        assert_eq!(
            rows[0].diff_runs(0),
            vec![("ab".into(), false), ("c".into(), true)]
        );
        assert_eq!(rows[0].diff_runs(1), vec![("ab".into(), false)]);
    }

    #[test]
    fn compare_unions_keys_and_flags_differences() {
        let a = parse_env("API_KEY=1\nURL=same\n");
        let b = parse_env("API_KEY=2\nURL=same\nONLY_B=x\n");
        let rows = compare(&[a, b]);

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].key, "API_KEY");
        assert!(rows[0].differs());
        assert_eq!(rows[1].key, "ONLY_B");
        assert_eq!(rows[1].values, vec![None, Some("x".to_string())]);
        assert!(rows[1].differs());
        assert_eq!(rows[2].key, "URL");
        assert!(!rows[2].differs());
    }
}
