// This example demonstrates the new build_hir_json methods
// Run with: cargo run --example hir_json_demo

use regex::{RegexBuilder, RegexSetBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RegexBuilder::build_hir_json() ===\n");
    
    // Single pattern example
    let json = RegexBuilder::new(r"\d{2,4}")
        .case_insensitive(true)
        .build_hir_json()?;
    println!("Pattern: r\"\\d{{2,4}}\"");
    println!("HIR JSON: {}\n", json);
    
    // Complex pattern example
    let json = RegexBuilder::new(r"^(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})$")
        .build_hir_json()?;
    println!("Pattern: r\"^(?P<year>\\d{{4}})-(?P<month>\\d{{2}})-(?P<day>\\d{{2}})$\"");
    println!("HIR JSON: {}\n", json);
    
    println!("=== RegexSetBuilder::build_hir_json() ===\n");
    
    // Multiple patterns example
    let json = RegexSetBuilder::new([
        r"\d+",
        r"[a-z]+",
        r"\w+",
    ])
    .build_hir_json()?;
    println!("Patterns: [r\"\\d+\", r\"[a-z]+\", r\"\\w+\"]");
    println!("HIR JSON: {}\n", json);
    
    println!("=== bytes::RegexBuilder::build_hir_json() ===\n");
    
    // Bytes regex example
    let json = regex::bytes::RegexBuilder::new(r"(?-u:.+)")
        .build_hir_json()?;
    println!("Pattern: r\"(?-u:.+)\"");
    println!("HIR JSON: {}\n", json);
    
    println!("=== bytes::RegexSetBuilder::build_hir_json() ===\n");
    
    // Bytes regex set example
    let json = regex::bytes::RegexSetBuilder::new([
        r"foo",
        r"bar",
        r"baz",
    ])
    .build_hir_json()?;
    println!("Patterns: [r\"foo\", r\"bar\", r\"baz\"]");
    println!("HIR JSON: {}", json);
    
    Ok(())
}
