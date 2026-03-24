use crate::sense::Reference;

#[derive(Debug)]
pub enum ExtractResult {
    None(Box<str>),
    Reference {
        parsed: Option<Box<str>>,
        reference: Reference
    }
}

pub struct ReferenceExtractor;

impl ReferenceExtractor {
    pub fn extract<'a>(text: Box<str>) -> ExtractResult {
        let trimmed = text.trim();

        // Case 1: prefix-based reference
        if let Some(payload) = Self::strip_prefix(trimmed) {
            let (core, _) = Self::split_on_comma(payload);
            if let Some(reference) = Self::parse_reference(core) {
                return ExtractResult::Reference {
                    parsed: None,
                    reference,
                };
            }
        }

        // Case 2: parenthetical reference
        if let Some((before, inside, _after)) = Self::extract_parenthetical_reference(trimmed) {
            if let Some(payload) = Self::strip_prefix(inside) {
                if let Some(reference) = Self::parse_reference(payload) {
                    return ExtractResult::Reference {
                        parsed: None,
                        reference,
                    };
                }
            }

            return ExtractResult::None(before.into());
        }

        ExtractResult::None(trimmed.into())
    }

    fn extract_parenthetical_reference<'a>(
        input: &'a str,
    ) -> Option<(&'a str, &'a str, Option<&'a str>)> {
        let start = input.rfind('(')?;
        let end = input[start..].find(')')? + start;

        let inside = input[start + 1..end].trim();
        let prefixes = ["abbr. for", "short for", "see", "variant of", "also written"];

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
            "also written",
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

    fn parse_reference<'a>(input: &'a str) -> Option<Reference> {
        let input = input.trim();

        let (chars_part, pinyin) = if let Some(start) = input.find('[') {
            let end = input[start..].find(']')? + start;
            let chars = input[..start].trim();
            let pinyin_vec: Vec<Box<str>> = input[start + 1..end]
                .split_whitespace()
                .map(Into::into)
                .collect();
            (chars, Some(pinyin_vec))
        } else {
            (input, None)
        };

        let (traditional, simplified) = Self::split_forms(chars_part);

        Some(Reference {
            traditional: traditional.into(),
            simplified: simplified.map(Into::into),
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
    fn should_extract_reference_starting_also_written() {
        let input = "also written 廂型車|厢型车[xiang1 xing2 che1]";

        let result = ReferenceExtractor::extract(input.into());
       
        match result {
            ExtractResult::None(_) => panic!("Should have reference"),
            ExtractResult::Reference { parsed, reference } => {
                assert!(parsed.is_none());
                assert_eq!(reference.traditional, "廂型車".into());
                assert_eq!(reference.simplified, Some("厢型车".into()));
                assert_eq!(reference.pinyin, Some(vec!["xiang1","xing2","che1"].into_iter().map(Into::into).collect::<Vec<_>>()));
            },
        }        
    }

    #[test]
    fn should_extract_reference_no_simplified() {
        let input = "abbr. for 第一作者[di4 yi1 zuo4 zhe3]";

        let result = ReferenceExtractor::extract(input.into());

        match result {
            ExtractResult::None(_) => panic!("Should have reference"),
            ExtractResult::Reference { parsed, reference } => {
                assert!(parsed.is_none());
                assert_eq!(reference.traditional, "第一作者".into());
                assert_eq!(reference.simplified, None);
                assert_eq!(reference.pinyin, Some(vec!["di4", "yi1", "zuo4", "zhe3"].into_iter().map(Into::into).collect::<Vec<_>>()));
            },
        }
    }

    #[test]
    fn should_extract_reference_variant_of() {
        let input = "variant of 帳篷|帐篷[zhang4 peng5]";

        let result = ReferenceExtractor::extract(input.into());
    
        match result {
            ExtractResult::None(_) => panic!("Should have reference"),
            ExtractResult::Reference { parsed, reference } => {
                assert!(parsed.is_none());
                assert_eq!(reference.traditional, "帳篷".into());
                assert_eq!(reference.simplified, Some("帐篷".into()));
                assert_eq!(reference.pinyin, Some(vec!["zhang4", "peng5"].into_iter().map(Into::into).collect::<Vec<_>>()));
            },
        }
    }
    
    #[test]
    fn should_extract_reference_also_written() {
        let input = "also written 廂型車|厢型车[xiang1 xing2 che1]";

        let result = ReferenceExtractor::extract(input.into());
        
        match result {
            ExtractResult::None(_) => panic!("Should have reference"),
            ExtractResult::Reference { parsed, reference } => {
                assert!(parsed.is_none());
                assert_eq!(reference.traditional, "廂型車".into());
                assert_eq!(reference.simplified, Some("厢型车".into()));
                assert_eq!(reference.pinyin, Some(vec!["xiang1", "xing2", "che1"].into_iter().map(Into::into).collect::<Vec<_>>()));
            },
        }
    }
}