/// Live macro expansion documentation framework
///
/// Documents macro expansions as "live documentation" that stays in sync with code changes.
/// For each test:
/// 1. Input file (`tests/expand/your_name.rs`) - shows macro usage
/// 2. Expanded file (`tests/expand/your_name.expanded.rs`) - full macro expansion from cargo-expand
/// 3. Filtered output (`tests/expanded_simple/your_name.rs`) - only your macro's code with syntax highlighting
/// 4. Snapshot (`.snap`) - same as filtered output, used by insta for testing
///
/// # Usage
///
/// 1. Create `tests/expand/your_name.rs` with your macro
/// 2. Add: `snapshot_expansion!(your_name, &["pattern1", "pattern2"]);`
/// 3. Run tests - auto-generates files
///
/// # Pattern matching
///
/// Filters expanded code to show only your macro's output (excludes Debug, Clone, other derives).
/// - When a line contains any pattern, capturing starts
/// - Captures entire blocks by tracking brace depth `{ }`
///
/// ```rust
/// snapshot_expansion!(my_test, &["impl MyTrait"]);
/// ```
///
/// # CI
///
/// Requires nightly Rust and `cargo install cargo-expand` (nightly doesn't need to be default).

macro_rules! snapshot_expansion {
    ($test_name:ident, $patterns:expr) => {
        #[test]
        fn $test_name() {
            let expanded_file = concat!("tests/expand/", stringify!($test_name), ".expanded.rs");

            if !std::path::Path::new(expanded_file).exists() {
                macrotest::expand("tests/expand/*.rs");
            }

            let expanded = std::fs::read_to_string(expanded_file)
                .expect("Failed to read expanded file");

            let filtered = filter_impls(&expanded, $patterns);

            // Write filtered output as a .rs file for easy reading with syntax highlighting
            std::fs::create_dir_all("tests/expanded_simple").expect("Failed to create expanded_simple directory");
            let filtered_file = concat!("tests/expanded_simple/", stringify!($test_name), ".rs");
            std::fs::write(filtered_file, &filtered)
                .expect("Failed to write filtered file");

            insta::assert_snapshot!(filtered);
        }
    };
}

fn filter_impls(expanded: &str, patterns: &[&str]) -> String {
    let lines: Vec<&str> = expanded.lines().collect();
    let mut result = Vec::new();
    let mut capturing = false;
    let mut brace_depth = 0;

    for line in lines {
        let trimmed = line.trim();

        // Start capturing when line matches any pattern
        if !capturing && patterns.iter().any(|p| trimmed.contains(p)) {
            capturing = true;
            brace_depth = 0;
        }

        if capturing {
            result.push(line);

            // Track brace depth to know when the block ends
            for ch in line.chars() {
                match ch {
                    '{' => brace_depth += 1,
                    '}' => brace_depth -= 1,
                    _ => {}
                }
            }

            // Stop capturing when we close the outermost block
            if brace_depth == 0 && line.contains('}') {
                capturing = false;
            }
        }
    }

    result.join("\n")
}

// Generate test for each expansion file
snapshot_expansion!(transfer_response, &["impl AwaitedSet", "impl HasEventSet"]);
snapshot_expansion!(event_basic, &["impl ::es_core::Event", "impl ::es_core::Idempotent", "impl ::es_core::Correlated"]);
snapshot_expansion!(event_with_injectable, &["impl ::es_core::DynEvent", "impl ::es_core::Event", "impl ::es_core::Idempotent", "impl ::es_core::Correlated", "impl ::es_core::AwaitableFor"]);
snapshot_expansion!(event_with_dotted_paths, &["impl ::es_core::Event", "impl ::es_core::Idempotent", "impl ::es_core::Correlated"]);
snapshot_expansion!(injectable_event_test, &["impl ::es_core::DynEvent", "impl ::es_core::Event", "impl ::es_core::ExpectsAwaitedSet", "impl ::es_core::Idempotent", "impl ::es_core::Correlated"]);
