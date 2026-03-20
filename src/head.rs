pub struct Head<'a> {
    pub traditional: &'a str,
    pub simplified: &'a str,
    pub pinyin: Vec<&'a str>,
}

pub struct HeadParser;

impl HeadParser {
    pub fn parse<'a>(line: &'a str) -> Option<(Head, &'a str)> {
        let mut parts = line.splitn(3, ' ');

        let traditional = parts.next()?;
        let simplified = parts.next()?;
        let rest = parts.next()?;

        let start = rest.find('[')?;
        let end = rest[start..].find(']')? + start;

        let pinyin_raw = &rest[start + 1..end];
        let defs_raw = rest[end + 1..].trim();

        let pinyin = pinyin_raw.split_whitespace().collect();

        Some((
            Head {
                traditional,
                simplified,
                pinyin,
            },
            defs_raw,
        ))
    }
}