use crate::{sense::{Classifier, Sense, SenseParser, Reference}, head::HeadParser};

/// A CC-CEDICT dictionary entry representing a single lexeme (headword).
///
/// Each entry corresponds to one Chinese word or phrase, written in both
/// traditional and simplified forms, along with its pronunciation in pinyin.
///
/// The entry may contain:
/// - one or more senses (meanings), each with one or more glosses,
/// - usage tags (e.g. literary, Taiwanese, colloquial),
/// - optional qualifiers,
/// - classifiers (measure words),
/// - and references to related lexical items such as:
///   - abbreviations (abbr. for),
///   - variants (variant of),
///   - alternative written forms (also written).
///
/// In CC-CEDICT, a single entry can express multiple meanings (senses),
/// typically separated by slashes in the raw data, with glosses optionally
/// grouped using semicolons.
///
/// Example:
/// 巨 [ju4] /very large; huge; tremendous; gigantic/(coll.) very; extremely/
///
/// This entry has:
/// - one lexeme ("巨")
/// - multiple senses
/// - some senses containing multiple glosses
#[derive(Debug)]
pub struct Entry<'a> {
    pub traditional: &'a str,
    pub simplified: &'a str,
    pub pinyin: Vec<&'a str>,
    pub senses: &'a str,
}

impl<'a> Entry<'a> {
    pub fn new(line: &'a str) -> Option<Self> {
        let (head, senses) = HeadParser::parse(line)?;

        Some(Self {
            traditional: head.traditional,
            simplified: head.simplified,
            pinyin: head.pinyin,
            senses
        })
    }
}

#[derive(Debug)]
pub struct RichEntry<'a> {
    pub traditional: &'a str,
    pub simplified: &'a str,
    pub pinyin: Vec<&'a str>,
    pub senses: Vec<Sense<'a>>,
    pub classifiers: Vec<Classifier<'a>>,
    pub references: Vec<Reference>,
}

impl<'a> RichEntry<'a> {
    pub fn new(line: &'a str) -> Option<Self> {
        let (head, rest) = HeadParser::parse(line)?;
        let (senses, classifiers, references) = SenseParser::parse(rest);

        Some(Self {
            traditional: head.traditional,
            simplified: head.simplified,
            pinyin: head.pinyin,
            senses,
            classifiers,
            references
        })
    }
}

mod tests {
    use anyhow::{Context, Result};
    use crate::sense::ReferenceKind;

    use super::*;

    #[test]
    fn should_parse_entry_with_tags_and_pinyin() -> Result<()> {
        let line = "綠茶婊 绿茶婊 [lu:4 cha2 biao3] /(slang) (derog.) a woman who appears innocent or gentle but is actually calculating and manipulative/";
        let entry = RichEntry::new(line).expect("Should parse line");

        assert_eq!(entry.simplified, "绿茶婊");
        assert_eq!(entry.traditional, "綠茶婊");
        assert_eq!(entry.pinyin, vec!["lu:4", "cha2", "biao3"]);
        assert_eq!(entry.senses[0].tags, vec!["slang", "derog."]);
        assert!(entry.senses[0].qualifier.is_none());
        // assert!(entry.definitions[0].reference.is_none());

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_tags_and_pinyin_and_abbr() -> Result<()> {
        let line = "美國交會 美国交会 [Mei3 guo2 Jiao1 hui4] /abbr. for 美國證券交易委員會|美国证券交易委员会, US Securities and Exchange Commission (SEC)/";
        let mut entry = RichEntry::new(line).expect("Should parse line");

        assert_eq!(entry.simplified, "美国交会");
        assert_eq!(entry.traditional, "美國交會");
        assert_eq!(entry.pinyin, vec!["Mei3", "guo2", "Jiao1", "hui4"]);
        assert_eq!(entry.senses[0].glosses, vec!["US Securities and Exchange Commission (SEC)".into()]);

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.kind, ReferenceKind::Abbreviation);
        assert_eq!(reference.pinyin, None);
        assert_eq!(reference.simplified, Some("美国证券交易委员会".into()));
        assert_eq!(reference.traditional, "美國證券交易委員會".into());

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_multiple_senses() -> Result<()> {
        let line = "神通廣大 神通广大 [shen2 tong1 guang3 da4] /(idiom) to possess great magical power; to possess remarkable abilities/";
        let entry = RichEntry::new(line).expect("Should parse line");

        assert_eq!(entry.simplified, "神通广大");
        assert_eq!(entry.traditional, "神通廣大");
        assert_eq!(entry.pinyin, vec!["shen2", "tong1", "guang3", "da4"]);
        assert_eq!(entry.senses[0].glosses, vec!["to possess great magical power".into(),  "to possess remarkable abilities".into()]);
        assert_eq!(entry.senses[0].tags, vec!["idiom"]);
        assert!(entry.senses[0].qualifier.is_none());

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_multiple_senses_and_starting_reference() -> Result<()> {
        let line = "空姐 空姐 [kong1 jie3] /abbr. for 空中小姐/stewardess/air hostess/female flight attendant/";
        let mut entry = RichEntry::new(line).expect("Should parse line");

        assert_eq!(entry.traditional, "空姐");
        assert_eq!(entry.simplified, "空姐");
        assert_eq!(entry.pinyin, vec!["kong1", "jie3"]);
        assert_eq!(entry.senses.len(), 3);
        // assert_eq!(entry.definitions[0].value, "stewardess");
        assert!(entry.senses[0].tags.is_empty());
        assert!(entry.senses[0].qualifier.is_none());
        // assert_eq!(entry.definitions[1].value, "air hostess");
        assert!(entry.senses[1].tags.is_empty());
        assert!(entry.senses[1].qualifier.is_none());

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.kind, ReferenceKind::Abbreviation);
        assert_eq!(reference.pinyin, None);
        assert_eq!(reference.traditional, "空中小姐".into());

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_multiple_senses_and_later_reference() -> Result<()> {
        let line = "箱型車 箱型车 [xiang1 xing2 che1] /van (Tw)/also written 廂型車|厢型车[xiang1 xing2 che1]/";
        let mut entry = RichEntry::new(line).expect("Should parse line");

        assert_eq!(entry.traditional, "箱型車");
        assert_eq!(entry.simplified, "箱型车");
        assert_eq!(entry.pinyin, vec!["xiang1", "xing2", "che1"]);

        assert_eq!(entry.senses.len(), 1);

        let sense = &entry.senses[0];
        assert_eq!(sense.glosses, vec!["van".into()]);
        assert!(sense.tags.contains(&"taiwanese"));
        assert!(sense.qualifier.is_none());

        assert_eq!(entry.references.len(), 1);

        let reference = &entry.references[0];
        assert_eq!(reference.kind, ReferenceKind::AlsoWritten);
        assert_eq!(reference.traditional, "廂型車".into());
        assert_eq!(reference.simplified, Some("厢型车".into()));
        assert_eq!(
            reference.pinyin,
            Some(vec!["xiang1".into(), "xing2".into(), "che1".into()])
        );


        Ok(())
    }

    #[test]
    fn should_parse_entry_with_alternative_written_form_and_reference() -> Result<()> {
        let line = "代駕 代驾 [dai4 jia4] /to drive a vehicle for its owner (often as a paid service for sb who has consumed alcohol) (abbr. for 代理駕駛|代理驾驶[dai4 li3 jia4 shi3])/substitute driver (abbr. for 代駕司機|代驾司机[dai4 jia4 si1 ji1])/";
        let mut entry = RichEntry::new(line).expect("Should parse line");

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.pinyin, Some(vec!["dai4".into(), "jia4".into(), "si1".into(), "ji1".into()]));
        assert_eq!(reference.traditional, "代駕司機".into());
        assert_eq!(reference.simplified, Some("代驾司机".into()));

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.kind, ReferenceKind::Abbreviation);
        assert_eq!(reference.traditional, "代理駕駛".into());
        assert_eq!(reference.simplified, Some("代理驾驶".into()));
        assert_eq!(reference.pinyin, Some(vec!["dai4".into(), "li3".into(), "jia4".into(), "shi3".into()]));

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_parenthetical_annotations() -> Result<()> {
        let line = r#"NG NG [N G] /(loanword from Japanese "NG", an initialism for "no good") (film and TV) blooper; to do a blooper/"#;
        let entry = RichEntry::new(line).expect("Should parse line");

        assert_eq!(entry.traditional, "NG");
        assert_eq!(entry.simplified, "NG");
        assert_eq!(entry.pinyin, vec!["N", "G"]);
        assert_eq!(entry.senses.len(), 1);

        let sense = &entry.senses[0];
        assert!(sense.glosses.iter().any(|g| g.contains("loanword from Japanese")));
        assert!(sense.glosses.iter().any(|g| g.contains("blooper")));
        assert!(sense.tags.contains(&"film and TV"));
        assert!(sense.qualifier.is_none());

        Ok(())
    }

    #[test]
    fn should_parse_entry_with_species_definition_and_qualifier_tag() -> Result<()> {
        let line = r#"玉帶海鵰 玉带海雕 [yu4 dai4 hai3 diao1] /(bird species of China) Pallas's fish eagle (Haliaeetus leucoryphus)/"#;
        let entry = RichEntry::new(line).expect("Should parse line");

        assert_eq!(entry.traditional, "玉帶海鵰");
        assert_eq!(entry.simplified, "玉带海雕");
        assert_eq!(entry.pinyin, vec!["yu4", "dai4", "hai3", "diao1"]);

        assert_eq!(entry.senses.len(), 1);

        let definition = &entry.senses[0];
        // assert_eq!(
        //     definition.value,
        //     "Pallas's fish eagle (Haliaeetus leucoryphus)"
        // );
        assert!(definition.tags.contains(&"bird species of China"));
        assert!(definition.qualifier.is_none());

        assert!(entry.references.is_empty());
        assert!(entry.classifiers.is_empty());

        Ok(())
    }

    #[test]
    fn should_parse_entry_only_reference_with_tag() -> Result<()> {
        let line = r#"劐 劐 [huo4] /(literary) variant of 穫|获[huo4]/"#;
        let mut entry = RichEntry::new(line).expect("Should parse line");

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.kind, ReferenceKind::Variant);
        assert_eq!(reference.traditional, "穫".into());
        assert_eq!(reference.simplified, Some("获".into()));
        assert_eq!(reference.pinyin, Some(vec!["huo4".into()]));

        Ok(())
    }

    #[test]
    fn should_parse_entry_only_reference() -> Result<()> {
        let line = r#"鉋 铇 [bao4] /variant of 刨[bao4]/"#;
        let mut entry = RichEntry::new(line).expect("Should parse line");

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.kind, ReferenceKind::Variant);
        assert_eq!(reference.traditional, "刨".into());
        assert_eq!(reference.simplified, None);
        assert_eq!(reference.pinyin, Some(vec!["bao4".into()]));

        Ok(())
    }

    #[test]
    fn should_parse_entry_two_references() -> Result<()> {
        let line = r#"超算 超算 [chao1 suan4] /supercomputing (abbr. for 超級計算|超级计算[chao1 ji2 ji4 suan4])/supercomputer (abbr. for 超級計算機|超级计算机[chao1 ji2 ji4 suan4 ji1])/"#;
        let mut entry = RichEntry::new(line).expect("Should parse line");

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.kind, ReferenceKind::Abbreviation);
        assert_eq!(reference.traditional, "超級計算機".into());
        assert_eq!(reference.simplified, Some("超级计算机".into()));
        assert_eq!(reference.pinyin, Some(vec!["chao1", "ji2", "ji4", "suan4", "ji1"].into_iter().map(Into::into).collect::<Vec<_>>()));

        let reference = entry.references.pop().with_context(|| "Should have reference")?;
        assert_eq!(reference.kind, ReferenceKind::Abbreviation);
        assert_eq!(reference.traditional, "超級計算".into());
        assert_eq!(reference.simplified, Some("超级计算".into()));
        assert_eq!(reference.pinyin, Some(vec!["chao1", "ji2", "ji4", "suan4"].into_iter().map(Into::into).collect::<Vec<_>>()));

        Ok(())
    }
    
    #[test]
    fn should_parse_entry_two_tags() -> Result<()> {
        let line = r#"不吝珠玉 不吝珠玉 [bu4 lin4 zhu1 yu4] /(idiom) (courteous) please give me your frank opinion; your criticism will be most valuable/"#;
        let mut entry = RichEntry::new(line).expect("Should parse line");

        assert_eq!(entry.traditional, "不吝珠玉");
        assert_eq!(entry.simplified, "不吝珠玉");
        assert_eq!(entry.pinyin, vec!["bu4", "lin4", "zhu1", "yu4"]);

        Ok(())
    }
}