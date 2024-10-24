#[test]
fn run_tests() {
    let t = trybuild::TestCases::new();

    // Copy/Clone references
    t.pass("tests/copy_clone/*.rs");
    //t.compile_fail("tests/ownership/refs/*FAIL.rs");

    // Ownership references
    t.pass("tests/ownership/refs/*COMPILE.rs");
    t.compile_fail("tests/ownership/refs/*FAIL.rs");

    // Ownership value
    t.pass("tests/ownership/value/*COMPILE.rs");
    t.compile_fail("tests/ownership/value/*FAIL.rs");
}

