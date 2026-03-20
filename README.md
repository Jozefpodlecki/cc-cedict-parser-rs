# CC CEDICT Parser

![rustc](https://img.shields.io/badge/rustc-1.94.0-blue.svg)
![CI](https://github.com/Jozefpodlecki/cc-cedict-parser-rs/actions/workflows/ci.yml/badge.svg)

A parser for the CC-CEDICT Chinese-English dictionary.

Parses raw dictionary lines into structured entries with support for:

- Traditional & simplified forms
- Pinyin (tokenized)
- Multiple definitions
- Tags (e.g. `slang`, `idiom`, `TCM`)
- Qualifiers (e.g. `lit.`, `fig.`, `coll.`)
- Classifiers (`CL:` entries)
- Abbreviation / reference extraction

### Installation 🚀

```toml
cc-cedict-parser-rs = { git = "https://github.com/Jozefpodlecki/cc-cedict-parser-rs" }
```

### 📦 Getting Started

```rust
let reader = LineReader::from_file(&file_path)?;
    
for line in reader {
    let line = line?;
    let entry = Entry::new(&line).with_context(|| "Could not parse line")?;
}
```

### Example

`神通廣大 神通广大 [shen2 tong1 guang3 da4] /(idiom) to possess great magical power; to possess remarkable abilities/`

```
 Entry {
    traditional: "神通廣大",
    simplified: "神通广大",
    pinyin: [
        "shen2",
        "tong1",
        "guang3",
        "da4",
    ],
    definitions: [
        Definition {
            value: "to possess great magical power",
            tags: [
                "idiom",
            ],
            qualifier: None,
        },
        Definition {
            value: "to possess remarkable abilities",
            tags: [],
            qualifier: None,
        },
    ],
    classifiers: [],
    reference: None,
}
```

### Credits
- Powered by the CC-CEDICT project - https://cc-cedict.org/