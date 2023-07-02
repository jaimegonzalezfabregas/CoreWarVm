use std::fs;

use crate::op::Instruction;

mod test_dwarf;
mod test_imp;

pub fn parse_ares_dump(file_path: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(file_path).unwrap();

    let mut ret = vec![];

    let str = contents.to_uppercase();

    for (i, line) in str.split('\n').enumerate() {
        match Instruction::parse(line.into(), 8000) {
            Ok(None) => println!("no instruction found at (file {}: line: {})", file_path, i),
            Ok(Some(op)) => ret.push(op),
            Err(err) => panic!(
                "while parsing ares dump {} (file {}: line: {})",
                err, file_path, i
            ),
        }
    }

    ret
}
