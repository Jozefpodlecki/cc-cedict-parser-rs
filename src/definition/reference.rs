use crate::definition::Reference;

pub struct ReferenceExtractor;

impl ReferenceExtractor {
    pub fn extract<'a>(text: &'a str) -> (&'a str, Option<&'a str>, Option<Reference<'a>>) {
        let text = text.trim();

        if let Some(payload) = Self::strip_prefix(text) {
            let (core, english) = Self::split_on_comma(payload);
            let reference = Self::parse_reference(core);
            return (payload, english, reference);
        }

        if let Some((before, inside, _after)) = Self::extract_parenthetical_reference(text) {
            let payload = match Self::strip_prefix(inside) {
                Some(p) => p,
                None => return (text, None, None),
            };

            let reference = Self::parse_reference(payload);

            return (text, Some(before), reference);
        }

        (text, None, None)
    }

    fn extract_parenthetical_reference<'a>(input: &'a str) -> Option<(&'a str, &'a str, Option<&'a str>)> {
        let start = input.rfind('(')?;
        let end = input[start..].find(')')? + start;

        let inside = input[start + 1..end].trim();

        let prefixes = ["abbr. for", "short for", "see", "variant of"];

        if !prefixes.iter().any(|p| inside.starts_with(p)) {
            return None;
        }

        let before = input[..start].trim();
        let after = input[end + 1..].trim();

        let after = if after.is_empty() { None } else { Some(after) };

        Some((before, inside, after))
    }

    fn strip_prefix<'a>(input: &'a str) -> Option<&'a str> {
        let prefixes = [
            "abbr. for",
            "short for",
            "see",
            "variant of",
        ];

        for prefix in prefixes {
            if let Some(rest) = input.strip_prefix(prefix) {
                return Some(rest.trim());
            }
        }

        None
    }

    fn split_on_comma(input: &str) -> (&str, Option<&str>) {
        let mut depth = 0;

        for (i, c) in input.char_indices() {
            match c {
                '[' | '(' => depth += 1,
                ']' | ')' => depth -= 1,
                ',' if depth == 0 => {
                    let (left, right) = input.split_at(i);
                    return (left.trim(), Some(right[1..].trim()));
                }
                _ => {}
            }
        }

        (input, None)
    }

    fn parse_reference<'a>(input: &'a str) -> Option<Reference<'a>> {
        let input = input.trim();

        let (chars_part, pinyin) = if let Some(start) = input.find('[') {
            let end = input[start..].find(']')? + start;

            let chars = input[..start].trim();
            let pinyin_str = input[start + 1..end].trim();

            let pinyin_vec: Vec<&'a str> = pinyin_str
                .split_whitespace()
                .collect();

            (chars, Some(pinyin_vec))
        } else {
            (input, None)
        };

        let (traditional, simplified) = Self::split_forms(chars_part);

        Some(Reference {
            traditional,
            simplified,
            pinyin,
        })
    }

    fn split_forms<'a>(input: &'a str) -> (&'a str, Option<&'a str>) {
        if let Some((a, b)) = input.split_once('|') {
            (a.trim(), Some(b.trim()))
        } else {
            (input.trim(), None)
        }
    }
}

mod tests {
    use anyhow::Result;
    use super::*;

    #[test]
    fn should_extract_reference_with_simplified() {
        let input = "abbr. for 剃頭挑子一頭熱|剃头挑子一头热[ti4 tou2 tiao1 zi5 yi1 tou2 re4]";

        let (_, _, reference) = ReferenceExtractor::extract(input);
        let reference = reference.expect("Should have reference");

        assert_eq!(reference.traditional, "剃頭挑子一頭熱");
        assert_eq!(reference.simplified, Some("剃头挑子一头热"));
        assert_eq!(reference.pinyin, Some(vec!["ti4", "tou2", "tiao1", "zi5", "yi1", "tou2", "re4"]));
    }

    #[test]
    fn should_extract_reference_no_simplified() {
        let input = "abbr. for 第一作者[di4 yi1 zuo4 zhe3]";

        let (_, _, reference) = ReferenceExtractor::extract(input);
        let reference = reference.expect("Should have reference");

        assert_eq!(reference.traditional, "第一作者");
        assert_eq!(reference.simplified, None);
        assert_eq!(reference.pinyin, Some(vec!["di4", "yi1", "zuo4", "zhe3"]));
    }

    #[test]
    fn should_extract_reference_with_text_after_comma() {
        let input = "abbr. for 美國證券交易委員會|美国证券交易委员会, US Securities and Exchange Commission (SEC)";

        let (_, text, reference) = ReferenceExtractor::extract(input);
        let reference = reference.expect("Should have reference");

        assert_eq!(text, Some("US Securities and Exchange Commission (SEC)"));
        assert_eq!(reference.traditional, "美國證券交易委員會");
        assert_eq!(reference.simplified, Some("美国证券交易委员会"));
        assert_eq!(reference.pinyin, None);
    }

    #[test]
    fn should_extract_reference_with_text() {
        let input = "top-tier academic conference (abbr. for 頂級會議|顶级会议[ding3 ji2 hui4 yi4])";

        let (_, text, reference) = ReferenceExtractor::extract(input);
        let reference = reference.expect("Should have reference");

        assert_eq!(text, Some("top-tier academic conference"));
        assert_eq!(reference.traditional, "頂級會議");
        assert_eq!(reference.simplified, Some("顶级会议"));
        assert_eq!(reference.pinyin, Some(vec!["ding3", "ji2", "hui4", "yi4"]));
    }

    #[test]
    fn should_extract_reference_variant_of() {
        let input = "variant of 帳篷|帐篷[zhang4 peng5]";

        let (_, _, reference) = ReferenceExtractor::extract(input);
        let reference = reference.expect("Should have reference");

        assert_eq!(reference.traditional, "帳篷");
        assert_eq!(reference.simplified, Some("帐篷"));
        assert_eq!(reference.pinyin, Some(vec!["zhang4", "peng5"]));
    }
    
}