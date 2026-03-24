use super::Classifier;

pub struct ClassifierParser;

impl ClassifierParser {
    pub fn parse<'a>(input: &'a str) -> Option<Vec<Classifier<'a>>> {
        let input = input.strip_prefix("CL:")?;

        let mut classifiers = Vec::new();

        for part in input.split(',') {
            let part = part.trim();

            let (chars, pinyin) = part.split_once('[')?;
            let pinyin = pinyin.strip_suffix(']')?.trim();

            let (traditional, simplified) = if let Some((t, s)) = chars.split_once('|') {
                (t.trim(), Some(s.trim()))
            } else {
                (chars.trim(), None)
            };

            classifiers.push(Classifier {
                traditional,
                simplified,
                pinyin,
            });
        }

        Some(classifiers)
    }
}

mod tests {
    use anyhow::{Context, Result};
    use super::*;

    #[test]
    fn should_parse_single_classifier_without_simplified() -> Result<()> {
        let input = "CL:頂|顶[ding3]";

        let mut classifiers = ClassifierParser::parse(input).with_context(|| "Should parse classifier")?;

        assert_eq!(classifiers.len(), 1);

        let classifier = classifiers.remove(0);

        assert_eq!(classifier.traditional, "頂");
        assert_eq!(classifier.simplified, Some("顶"));
        assert_eq!(classifier.pinyin, "ding3");

        Ok(())
    }

    #[test]
    fn should_parse_multiple_classifiers() -> Result<()> {
        let input = "CL:雙|双[shuang1],隻|只[zhi1]";

        let classifiers = ClassifierParser::parse(input).with_context(|| "Should parse classifier")?;

        assert_eq!(classifiers.len(), 2);
        
        assert_eq!(classifiers[0].traditional, "雙");
        assert_eq!(classifiers[0].simplified, Some("双"));
        assert_eq!(classifiers[0].pinyin, "shuang1");
        
        assert_eq!(classifiers[1].traditional, "隻");
        assert_eq!(classifiers[1].simplified, Some("只"));
        assert_eq!(classifiers[1].pinyin, "zhi1");

        Ok(())
    }
}