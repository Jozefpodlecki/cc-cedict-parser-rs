use std::{collections::HashMap, sync::LazyLock};

pub struct TagExtractor;

pub static TAG_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert("biology", "biology");
    map.insert("math", "math");
    map.insert("math.", "mathematics");  // Normalize math. to mathematics
    map.insert("chemistry", "chemistry");
    map.insert("physics", "physics");
    map.insert("informal", "informal");
    map.insert("slang", "slang");
    map.insert("ACG", "ACG");
    map.insert("idiom", "idiom");
    map.insert("derog.", "derog.");
    map.insert("literary", "literary");
    map.insert("TCM", "TCM");
    
    map.insert("Internet slang", "Internet slang");
    map.insert("mathematics", "mathematics");
    map.insert("loanword", "loanword");
    map.insert("film and TV", "film and TV");
    map.insert("Tw", "taiwanese");
    map.insert("tw", "taiwanese");
    map.insert("taiwanese", "taiwanese");
    map.insert("euphemism", "euphemism");
    map.insert("courteous", "courteous");
    map.insert("dialect", "dialect");
    map.insert("computing", "computing");
    map.insert("old", "old");
    map.insert("name", "name");
    map.insert("constellation", "constellation");
    map.insert("Cantonese", "Cantonese");
    map.insert("zoology", "zoology");

    map.insert("bound form", "bound form");
    map.insert("bird species of China", "bird species of China");
    
    map
});

impl TagExtractor {
     pub fn extract<'a>(input: &'a str) -> (Box<str>, Vec<&'a str>) {
        let mut tags = Vec::new();
        let mut result = String::with_capacity(input.len());
        let trimmed_input = input.trim();

        let mut last_pos = 0;
        let mut current_pos = 0;

        while let Some(start) = trimmed_input[current_pos..].find('(') {
            let abs_start = current_pos + start;

            if let Some(end) = trimmed_input[abs_start..].find(')') {
                let abs_end = abs_start + end;
                let tag_content = trimmed_input[abs_start + 1..abs_end].trim();

                result.push_str(&trimmed_input[last_pos..abs_start]);

                if let Some(&normalized) = TAG_MAP.get(tag_content) {
                    tags.push(normalized);
                } else {
                    result.push('(');
                    result.push_str(tag_content);
                    result.push(')');
                }

                last_pos = abs_end + 1;
                current_pos = abs_end + 1;
            } else {
                result.push_str(&trimmed_input[last_pos..]);
                break;
            }
        }

        result.push_str(&trimmed_input[last_pos..]);

        (result.trim().into(), tags)
    }
    // pub fn extract<'a>(input: &'a str) -> (String, Vec<&'a str>) {
    //     let mut tags = Vec::new();
    //     let mut result = String::with_capacity(input.len());
    //     let mut last_pos = 0;
    //     let mut current_pos = 0;
    //     let trimmed_input = input.trim();
        
    //     while let Some(start) = trimmed_input[current_pos..].find('(') {
    //         let abs_start = current_pos + start;
            
    //         if let Some(end) = trimmed_input[abs_start..].find(')') {
    //             let abs_end = abs_start + end;
    //             let tag_content = trimmed_input[abs_start + 1..abs_end].trim();
                
    //             // Add content before this tag to result
    //             result.push_str(&trimmed_input[last_pos..abs_start]);
                
    //             // Try to validate and normalize the tag
    //             if let Some(&normalized) = TAG_MAP.get(tag_content) {
    //                 tags.push(normalized);
    //                 last_pos = abs_end + 1;
    //                 current_pos = abs_end + 1;
    //             } else {
    //                 // Not a valid tag, treat as regular content
    //                 // Keep the parentheses as part of the content
    //                 result.push('(');
    //                 result.push_str(tag_content);
    //                 result.push(')');
    //                 last_pos = abs_end + 1;
    //                 current_pos = abs_end + 1;
    //             }
    //         } else {
    //             // Unclosed parenthesis, treat rest as content
    //             result.push_str(&trimmed_input[last_pos..]);
    //             break;
    //         }
    //     }
        
    //     result.push_str(&trimmed_input[last_pos..]);
        
    //     (result.trim().to_owned(), tags)
    // }
    
    pub fn is_valid_tag(tag: &str) -> bool {
        TAG_MAP.contains_key(tag)
    }
    
    pub fn available_tags() -> Vec<&'static str> {
        TAG_MAP.keys().copied().collect()
    }
    
    pub fn normalize_tag(tag: &str) -> Option<&'static str> {
        TAG_MAP.get(tag).copied()
    }
}

mod tests {
    use anyhow::Result;
    use super::*;

    #[test]
    fn should_extract_single_tag() {
        let input = "(biology) alternation of generations";
        let (remainder, tags) = TagExtractor::extract(input);
        
        assert_eq!(tags, vec!["biology"]);
        assert_eq!(remainder.as_ref(), "alternation of generations");
    }

    #[test]
    fn should_extract_multiple_tags() {
        let input = "Boolean (math.) (Tw)";
        let (remainder, tags) = TagExtractor::extract(input);
        
        assert_eq!(tags, vec!["mathematics", "taiwanese"]);
        assert_eq!(remainder.as_ref(), "Boolean");
    }
}