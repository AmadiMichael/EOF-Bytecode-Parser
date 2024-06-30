pub mod constants;
pub mod parser;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::{parser::parse_eof_bytecode, types::eof_test_json::Input};
    use std::{fs::File, io::BufReader};

    #[test]
    fn test_ef_test_vectors() {
        // Open the file in read-only mode with buffer.
        let file = File::open("inputs.json").unwrap();
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        let test_input: Input = serde_json::from_reader(reader).expect("Could not deserialize");
        let test_vectors = test_input.valid_invalid.vectors;

        for (name, test) in test_vectors.into_iter() {
            let bytecode = get_bytecode_from_hex(test.code);
            let ret = parse_eof_bytecode(&bytecode);

            match test.results.prague.result {
                false => assert!(ret.is_err(), "Test {} did not fail as expected", name),
                true => {
                    assert!(
                        ret.is_ok(),
                        "Test {} did not pass as expected. Err: {}",
                        name,
                        ret.err().unwrap()
                    );
                }
            };

            println!("Test {} passed ✅", name);
        }
    }

    fn get_bytecode_from_hex(input: String) -> Vec<u8> {
        let input = input.replace(" ", "");
        let input = input.replace("0x", "");

        let bytecode = hex::decode(input).expect("invalid bytecode input");

        return bytecode;
    }
}
