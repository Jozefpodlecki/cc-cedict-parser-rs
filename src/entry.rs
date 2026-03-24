use crate::{definition::{Classifier, Definition, DefinitionParser, Reference}, head::HeadParser};

#[derive(Debug)]
pub struct Entry<'a> {
    pub traditional: &'a str,
    pub simplified: &'a str,
    pub pinyin: Vec<&'a str>,
    pub definitions: Vec<Definition<'a>>,
    pub classifiers: Vec<Classifier<'a>>,
    pub references: Vec<Reference<'a>>,
}

impl<'a> Entry<'a> {
    pub fn new(line: &'a str) -> Option<Self> {
        let (head, rest) = HeadParser::parse(line)?;
        let (definitions, classifiers, references) = DefinitionParser::parse(rest);

        Some(Self {
            traditional: head.traditional,
            simplified: head.simplified,
            pinyin: head.pinyin,
            definitions,
            classifiers,
            references
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

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.pinyin, None);
        assert_eq!(reference.simplified, Some("美国证券交易委员会"));
        assert_eq!(reference.traditional, "美國證券交易委員會");

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_multiple_definitions() -> Result<()> {
        let line = "神通廣大 神通广大 [shen2 tong1 guang3 da4] /(idiom) to possess great magical power; to possess remarkable abilities/";
        let entry = Entry::new(line).expect("Should parse line");

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
        let mut entry = Entry::new(line).expect("Should parse line");

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

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.pinyin, None);
        assert_eq!(reference.traditional, "空中小姐");

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_multiple_definitions_and_reference_1() -> Result<()> {
        let line = "箱型車 箱型车 [xiang1 xing2 che1] /van (Tw)/also written 廂型車|厢型车[xiang1 xing2 che1]/";
        let mut entry = Entry::new(line).expect("Should parse line");

        assert_eq!(entry.traditional, "箱型車");
        assert_eq!(entry.simplified, "箱型车");
        assert_eq!(entry.pinyin, vec!["xiang1", "xing2", "che1"]);

        assert_eq!(entry.definitions.len(), 1);

        let definition = &entry.definitions[0];
        assert_eq!(definition.value, "van");
        assert!(definition.tags.contains(&"taiwanese"));
        assert!(definition.qualifier.is_none());

        assert_eq!(entry.references.len(), 1);

        let reference = &entry.references[0];
        assert_eq!(reference.traditional, "廂型車");
        assert_eq!(reference.simplified, Some("厢型车"));
        assert_eq!(
            reference.pinyin,
            Some(vec!["xiang1", "xing2", "che1"])
        );


        Ok(())
    }

    #[test]
    fn should_parse_entry_with_multiple_definitions_and_references() -> Result<()> {
        let line = "代駕 代驾 [dai4 jia4] /to drive a vehicle for its owner (often as a paid service for sb who has consumed alcohol) (abbr. for 代理駕駛|代理驾驶[dai4 li3 jia4 shi3])/substitute driver (abbr. for 代駕司機|代驾司机[dai4 jia4 si1 ji1])/";
        let mut entry = Entry::new(line).expect("Should parse line");

        dbg!(&entry);

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.pinyin, Some(vec!["dai4", "jia4", "si1", "ji1"]));
        assert_eq!(reference.traditional, "代駕司機");
        assert_eq!(reference.simplified, Some("代驾司机"));

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.traditional, "代理駕駛");
        assert_eq!(reference.simplified, Some("代理驾驶"));
        assert_eq!(reference.pinyin, Some(vec!["dai4", "li3", "jia4", "shi3"]));

        Ok(())
    }

    #[test]
    fn should_parse_cedict_entry_with_parenthetical_annotations() -> Result<()> {
        let line = r#"NG NG [N G] /(loanword from Japanese "NG", an initialism for "no good") (film and TV) blooper; to do a blooper/"#;
        let entry = Entry::new(line).expect("Should parse line");

        assert_eq!(entry.traditional, "NG");
        assert_eq!(entry.simplified, "NG");

        assert_eq!(entry.pinyin, vec!["N", "G"]);

        assert_eq!(entry.definitions.len(), 2);

        let definition = &entry.definitions[0];
        assert!(definition.value.contains("loanword from Japanese \"NG\""));
        assert!(definition.value.contains("blooper"));
        assert!(definition.tags.contains(&"film and TV"));
        assert!(definition.qualifier.is_none());

        let definition = &entry.definitions[1];
        assert_eq!(definition.value, "to do a blooper");
        assert!(definition.tags.is_empty());
        assert!(definition.qualifier.is_none());

        assert!(entry.classifiers.is_empty());
        assert!(entry.references.is_empty());

        Ok(())
    }
    
    // 不吝珠玉 不吝珠玉 [bu4 lin4 zhu1 yu4] /(idiom) (courteous) please give me your frank opinion; your criticism will be most valuable/
}