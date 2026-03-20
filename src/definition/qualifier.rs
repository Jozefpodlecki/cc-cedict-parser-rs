use std::collections::HashMap;
use std::sync::LazyLock;

static QUALIFIERS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("lit.", "literary");
    map.insert("fig.", "figuratively");
    map.insert("coll.", "colloquially");
    map.insert("abbr.", "abbreviation");
    map
});

pub struct QualifierExtractor;

impl QualifierExtractor {
    pub fn extract<'a>(input: &'a str) -> (&'a str, Option<&'a str>) {
        let trimmed = input.trim();
        
        if let Some(start) = trimmed.find('(') {
            if let Some(end) = trimmed[start..].find(')') {
                let abs_end = start + end;
                let inside = trimmed[start + 1..abs_end].trim();
                let before = &trimmed[..start].trim();
                let after = &trimmed[abs_end + 1..].trim();
                
                if let Some(value) = QUALIFIERS.get(inside) {

                    let remainder = if before.is_empty() {
                        after.to_string()
                    } else if after.is_empty() {
                        before.to_string()
                    } else {
                        format!("{} {}", before, after)
                    };
                    return (remainder.leak(), Some(value));
                }
            }
        }
        
        (input, None)
    }

}

mod tests {
    use anyhow::Result;
    use super::*;

    #[test]
    fn should_extract_qualifier_from_start() {
        let input = "(fig.) to bring about; to produce";

        let result = QualifierExtractor::extract(input);

        assert_eq!(result, ("to bring about; to produce", Some("figuratively")));
    }

    #[test]
    fn should_extract_qualifier_from_end() {
        let input = "US and Canada (abbr.)";

        let result = QualifierExtractor::extract(input);

        assert_eq!(result, ("US and Canada", Some("abbreviation")));
    }
    
}