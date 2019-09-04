use crate::TestCases;

#[test]
fn oxide_test() {
    let t = TestCases::new();
    t.exe_compile_fail("tests/oxide/*.o2", "/Users/Jonathan/Oxide/target/debug/o2c", &[]);
    t.exe_stderr("tests/oxide/passing/*.o2", "/Users/Jonathan/Oxide/target/debug/o2c", &["--pretty-lex"]);
    t.exe_stdout("tests/oxide/passing/*.o2", "/Users/Jonathan/Oxide/target/debug/o2c", &[]);
}