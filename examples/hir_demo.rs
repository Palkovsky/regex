// This example demonstrates obtaining HIR (High-level Intermediate Representation)
// from regex patterns, displaying them, and rebuilding regexes from HIRs.
// Run with: cargo run --example hir_demo

use regex::{RegexBuilder, RegexSetBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== RegexBuilder::hir() ===\n");

    // Single pattern example
    let hir = RegexBuilder::new(r"\d{2,4}").case_insensitive(true).hir()?;
    println!("Pattern: r\"\\d{{2,4}}\"");
    println!("HIR: {:?}\n", hir);

    // Build regex from HIR
    let re =
        RegexBuilder::new("").case_insensitive(true).build_from_hir(&hir)?;
    println!("Rebuilt regex from HIR");
    println!("Test match '123': {}", re.is_match("123"));
    println!("Test match 'AB': {}\n", re.is_match("AB"));

    // Complex pattern example
    let hir = RegexBuilder::new(
        r"^(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})$",
    )
    .hir()?;
    println!("Pattern: r\"^(?P<year>\\d{{4}})-(?P<month>\\d{{2}})-(?P<day>\\d{{2}})$\"");
    println!("HIR: {:?}\n", hir);

    // Build regex from HIR
    let re = RegexBuilder::new("").build_from_hir(&hir)?;
    println!("Rebuilt regex from HIR");
    let test_date = "2025-10-29";
    if let Some(caps) = re.captures(test_date) {
        println!("Test match '{}': matched", test_date);
        println!("  year: {}", &caps["year"]);
        println!("  month: {}", &caps["month"]);
        println!("  day: {}\n", &caps["day"]);
    }

    println!("=== RegexSetBuilder::hirs() ===\n");

    // Multiple patterns example
    let hirs = RegexSetBuilder::new([r"\d+", r"[a-z]+", r"\w+"]).hirs()?;
    println!("Patterns: [r\"\\d+\", r\"[a-z]+\", r\"\\w+\"]");
    println!("HIRs: {:?}\n", hirs);

    // Build regex set from HIRs
    let re_set = RegexSetBuilder::new([""]).build_from_hirs(&hirs)?;
    println!("Rebuilt regex set from HIRs");
    let test_str = "hello123";
    let matches: Vec<_> = re_set.matches(test_str).into_iter().collect();
    println!("Test '{}': matched patterns {:?}\n", test_str, matches);

    println!("=== bytes::RegexBuilder::hir() ===\n");

    // Bytes regex example
    let hir = regex::bytes::RegexBuilder::new(r"(?-u:.+)").hir()?;
    println!("Pattern: r\"(?-u:.+)\"");
    println!("HIR: {:?}\n", hir);

    // Build bytes regex from HIR
    let re = regex::bytes::RegexBuilder::new("").build_from_hir(&hir)?;
    println!("Rebuilt bytes regex from HIR");
    println!("Test match b\"hello\": {}\n", re.is_match(b"hello"));

    println!("=== bytes::RegexSetBuilder::hirs() ===\n");

    // Bytes regex set example
    let hirs =
        regex::bytes::RegexSetBuilder::new([r"foo", r"bar", r"baz"]).hirs()?;
    println!("Patterns: [r\"foo\", r\"bar\", r\"baz\"]");
    println!("HIRs: {:?}\n", hirs);

    // Build bytes regex set from HIRs
    let re_set =
        regex::bytes::RegexSetBuilder::new([""]).build_from_hirs(&hirs)?;
    println!("Rebuilt bytes regex set from HIRs");
    let test_bytes = b"foobar";
    let matches: Vec<_> = re_set.matches(test_bytes).into_iter().collect();
    println!("Test b\"foobar\": matched patterns {:?}", matches);

    Ok(())
}
