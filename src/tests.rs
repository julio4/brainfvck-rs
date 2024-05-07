macro_rules! assert_output {
    ($file:expr, $expected:expr, $size:expr) => {{
        use crate::instructions::ParserFromFileTrait;
        use crate::interpreter::InterpreterTrait;
        use crate::state::{State, StateTrait};
        use std::fs::File;

        // Open file and parse as program
        let program = File::open($file)
            .expect("File not found")
            .parse_as_program();

        // Create state
        let mut state = State::new($size);

        // Create output vector
        let mut output = Vec::new();

        // Run the program
        program.run(&mut state, &mut output).unwrap();

        // Read expected output into a vector
        let expected_output = $expected.as_bytes().to_vec();

        // Assert output
        assert_eq!(output, expected_output);
    }};
}

#[test]
fn test1() {
    assert_output!("./tests/test.bf", "H", 2);
}

#[test]
fn test2() {
    assert_output!("./tests/test2.bf", "Hello World!\n", 2);
}

#[test]
fn test5() {
    assert_output!("./tests/test5.bf", "8", 4);
}

// #[test]
// fn test8() {
//     assert_output!("./tests/test8.bf", "H");
// }
