#[test]
fn test_compile_failures() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/duplicate_keys.rs");
    t.compile_fail("tests/compile_fail/key_path_conflict.rs");
    t.compile_fail("tests/compile_fail/multiple_conflicts.rs");
    t.compile_fail("tests/compile_fail/deep_duplicate.rs");
    t.compile_fail("tests/compile_fail/invalid_field_name.rs");
}
