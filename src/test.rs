use std::fs;

use crate::op::{ReadOnlyInstruction};


mod test_dwarf;
mod test_imp;
mod test_warrior_colision;
mod test_normal_run;

pub fn parse_ares_dump(file_path: &str) -> Vec<ReadOnlyInstruction> {
    let contents = fs::read_to_string(file_path).unwrap();

    let mut ret = vec![];

    let str = contents.to_uppercase();

    for (i, line) in str.split('\n').enumerate() {
        match ReadOnlyInstruction::parse(line.into(), 8000) {
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
