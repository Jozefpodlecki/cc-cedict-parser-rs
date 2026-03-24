# Architecture

## Overview

`cc-cedict-parser-rs` is a stream-oriented, zero-copy parsing library
for transforming CC-CEDICT dictionary lines into structured Rust
representations.

The system is designed around a multi-stage parsing pipeline:

`Input Source → LineReader → HeadParser → SenseParser → Entry / RichEntry`

Key architectural goals:

-   Streaming processing (no full dataset load)
-   Zero-copy parsing where possible
-   Clear separation of parsing stages
-   Composable, testable components

------------------------------------------------------------------------

## High-Level Architecture

### Pipeline

Raw Line (String) ↓ HeadParser ↓ (Head, defs_raw) ↓ SenseParser ↓
(senses, classifiers, references) ↓ RichEntry

### Layered Responsibilities

I/O Layer └── LineReader

Lexical Parsing Layer └── HeadParser

Semantic Parsing Layer └── SenseParser

Domain Assembly Layer ├── Entry (lightweight) └── RichEntry (fully
parsed)

------------------------------------------------------------------------

## Module Structure

crate ├── reader ├── head ├── sense ├── entry

### Public API

Only `reader` and `entry` are exposed, making them the primary
user-facing components.

------------------------------------------------------------------------

## Input Layer: LineReader

### Responsibilities

-   Read from any BufRead
-   Skip comments (#)
-   Normalize line endings
-   Yield Result`<String>`{=html}

### Characteristics

-   Generic over input source
-   Lazy error propagation
-   No domain logic

------------------------------------------------------------------------

## Lexical Parsing: HeadParser

### Responsibilities

-   Extract traditional and simplified forms
-   Extract and tokenize pinyin
-   Forward remaining definition string

### Design

-   Zero-copy string slicing
-   Single-pass parsing
-   Fail-fast using Option

------------------------------------------------------------------------

## Semantic Parsing: SenseParser

### Responsibilities

-   Parse senses (/.../)
-   Extract glosses, tags, qualifiers
-   Extract classifiers (CL:)
-   Extract references (abbr., variant, also written)

------------------------------------------------------------------------

## Domain Models

### Entry (Lightweight)

-   Minimal parsing
-   Holds raw senses string
-   Zero-copy

### RichEntry (Fully Parsed)

-   Structured senses
-   Classifiers
-   References

------------------------------------------------------------------------

## Data Flow Example

``` rust
let reader = LineReader::from_file(path)?;

for line in reader {
    let line = line?;
    let entry = RichEntry::new(&line)?;
}
```

------------------------------------------------------------------------

## Error Handling

-   I/O uses Result
-   Parsing uses Option
-   Fail-fast design with minimal diagnostics

------------------------------------------------------------------------

## Memory Model

-   Predominantly zero-copy (&str)
-   Allocations only for vectors
-   Lifetime-bound to input line

------------------------------------------------------------------------

## Architectural Patterns

-   Streaming pipeline
-   Staged parsing
-   Manual parser combinators
-   Data-oriented design

------------------------------------------------------------------------

## Performance Characteristics

### Strengths

-   Low memory usage
-   High throughput
-   No regex overhead

### Trade-offs

-   Limited error reporting
-   Strict format assumptions

------------------------------------------------------------------------

## Summary

A high-performance, streaming parser optimized for large-scale
dictionary processing with clear modular separation and efficient memory
usage.
