use crate::{definition::{Classifier, Definition, DefinitionParser, Reference}, head::HeadParser};

#[derive(Debug)]
pub struct Entry<'a> {
    pub traditional: &'a str,
    pub simplified: &'a str,
    pub pinyin: Vec<&'a str>,
    pub definitions: Vec<Definition<'a>>,
    pub classifiers: Vec<Classifier<'a>>,
    pub reference: Option<Reference<'a>>,
}

impl<'a> Entry<'a> {
    pub fn new(line: &'a str) -> Option<Self> {
        let (head, rest) = HeadParser::parse(line)?;
        let (definitions, classifiers, reference) = DefinitionParser::parse(rest);

        Some(Self {
            traditional: head.traditional,
            simplified: head.simplified,
            pinyin: head.pinyin,
            definitions,
            classifiers,
            reference
        })
    }
}

mod tests {
    use anyhow::{Context, Result};
    use super::*;

    #[test]
    fn should_parse_entry_with_tags_and_pinyin() -> Result<()> {
        let line = "綠茶婊 绿茶婊 [lu:4 cha2 biao3] /(slang) (derog.) a woman who appears innocent or gentle but is actually calculating and manipulative/";
        let entry = Entry::new(line).expect("Should parse line");

        assert_eq!(entry.simplified, "绿茶婊");
        assert_eq!(entry.traditional, "綠茶婊");
        assert_eq!(entry.pinyin, vec!["lu:4", "cha2", "biao3"]);
        assert_eq!(entry.definitions[0].tags, vec!["slang", "derog."]);
        assert!(entry.definitions[0].qualifier.is_none());
        // assert!(entry.definitions[0].reference.is_none());

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_tags_and_pinyin_and_abbr() -> Result<()> {
        let line = "美國交會 美国交会 [Mei3 guo2 Jiao1 hui4] /abbr. for 美國證券交易委員會|美国证券交易委员会, US Securities and Exchange Commission (SEC)/";
        let mut entry = Entry::new(line).expect("Should parse line");

        assert_eq!(entry.simplified, "美国交会");
        assert_eq!(entry.traditional, "美國交會");
        assert_eq!(entry.pinyin, vec!["Mei3", "guo2", "Jiao1", "hui4"]);
        assert_eq!(entry.definitions[0].value, "US Securities and Exchange Commission (SEC)");
        assert!(entry.definitions[0].tags.is_empty());
        assert!(entry.definitions[0].qualifier.is_none());

        let reference = entry.reference.with_context(|| "Should have reference")?;
        assert_eq!(reference.pinyin, None);
        assert_eq!(reference.simplified, Some("美国证券交易委员会"));
        assert_eq!(reference.traditional, "美國證券交易委員會");

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_multiple_definitions() -> Result<()> {
        let line = "神通廣大 神通广大 [shen2 tong1 guang3 da4] /(idiom) to possess great magical power; to possess remarkable abilities/";
        let entry = Entry::new(line).expect("Should parse line");
        dbg!(&entry);
        assert_eq!(entry.simplified, "神通广大");
        assert_eq!(entry.traditional, "神通廣大");
        assert_eq!(entry.pinyin, vec!["shen2", "tong1", "guang3", "da4"]);
        assert_eq!(entry.definitions[0].value, "to possess great magical power");
        assert_eq!(entry.definitions[0].tags, vec!["idiom"]);
        assert!(entry.definitions[0].qualifier.is_none());
        assert_eq!(entry.definitions[1].value, "to possess remarkable abilities");
        assert!(entry.definitions[1].tags.is_empty());
        assert!(entry.definitions[1].qualifier.is_none());

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_multiple_definitions_and_reference() -> Result<()> {
        let line = "空姐 空姐 [kong1 jie3] /abbr. for 空中小姐/stewardess/air hostess/female flight attendant/";
        let entry = Entry::new(line).expect("Should parse line");

        assert_eq!(entry.traditional, "空姐");
        assert_eq!(entry.simplified, "空姐");
        assert_eq!(entry.pinyin, vec!["kong1", "jie3"]);
        assert_eq!(entry.definitions.len(), 3);
        assert_eq!(entry.definitions[0].value, "stewardess");
        assert!(entry.definitions[0].tags.is_empty());
        assert!(entry.definitions[0].qualifier.is_none());
        assert_eq!(entry.definitions[1].value, "air hostess");
        assert!(entry.definitions[1].tags.is_empty());
        assert!(entry.definitions[1].qualifier.is_none());

        let reference = entry.reference.with_context(|| "Should have reference")?;
        assert_eq!(reference.pinyin, None);
        assert_eq!(reference.simplified, Some("空中小姐"));

        Ok(())
    }
}