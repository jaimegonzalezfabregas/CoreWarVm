pub mod test_chang_vs_mice;
pub mod test_death;
mod test_dwarf;
pub mod test_imp;
mod test_imp_wall;
pub mod test_predecrement;
mod test_warrior_colision;
mod test_arithmetic;
pub mod test_div_cero;
mod test_mod_cero;

#[cfg(test)]
mod tests {

    use std::fs;

    use crate::core::CoreRuntime;
    use crate::instruction::instruction::Instruction;
    use crate::instruction::{op_code::OpCode, op_modifier::OpModifier};

    pub fn parse_ares_dump(file_path: &str, core_size: usize) -> Vec<ReadOnlyInstruction> {
        let contents = fs::read_to_string(file_path).unwrap();

        let mut ret = vec![];

        let str = contents.to_uppercase();

        for (i, line) in str.split('\n').enumerate() {
            match ReadOnlyInstruction::parse(line.into(), core_size) {
                Ok(None) => (),
                Ok(Some(op)) => ret.push(op),
                Err(err) => panic!(
                    "while parsing ares dump {} (file {}: line: {})",
                    err, file_path, i
                ),
            }
        }

        ret
    }

    pub fn compare_runtime_with_file(file_path: &str, runtime: &CoreRuntime, note: &str) {
        let res = parse_ares_dump(file_path, runtime.core_size);

        for cell_i in 0..runtime.core_size {
            let a = <Instruction as Into<ReadOnlyInstruction>>::into(runtime.core[cell_i].clone());
            let b = res[cell_i];
            if a != b {
                runtime.print_state(Some(cell_i.max(10) - 10..cell_i + 10));

                panic!("[{}]: checking pos {cell_i}: \n{a:?} \n!= \n{b:?}\n", note);
            }
        }
    }

    use crate::instruction::field::Field;

    #[derive(Clone, Copy, Debug, Eq)]
    pub struct ReadOnlyInstruction {
        pub code: OpCode,
        pub modifier: OpModifier,
        pub fields: [Option<Field>; 2],
    }

    impl PartialEq for ReadOnlyInstruction {
        fn eq(&self, other: &Self) -> bool {
            self.code == other.code
                && self.modifier == other.modifier
                && {
                    self.fields[0].is_none()
                        || other.fields[0].is_none()
                        || self.fields[0] == other.fields[0]
                }
                && {
                    self.fields[1].is_none()
                        || other.fields[1].is_none()
                        || self.fields[1] == other.fields[1]
                }
        }
    }

    impl ReadOnlyInstruction {
        pub fn parse(line: String, core_size: usize) -> Result<Option<Self>, String> {
            let line = match line.find(";") {
                Some(x) => line[0..x].to_string(),
                None => line,
            };

            let line = line.trim_start().to_string();

            if line == "" {
                return Ok(None);
            }

            let (code, line) = OpCode::parse(line.into())?;

            let line = line.trim_start().to_string();

            let (modifier, line) = OpModifier::parse(line.into())?;

            let line = line.trim_start().to_string();

            let (mut a, line) = Field::parse(line.into(), core_size)?;

            let line = line.trim_start().to_string();

            let (mut b, _) = Field::parse(line.into(), core_size)?;

            if let OpCode::DAT = code {
                if let None = b {
                    (b, a) = (a, b);
                }
            }

            Ok(Some(Self {
                code,
                modifier,
                fields: [a, b],
            }))
        }
    }
    impl From<Instruction> for ReadOnlyInstruction {
        fn from(value: Instruction) -> Self {
            return Self {
                code: value.code,
                modifier: value.modifier,
                fields: [Some(value.fields[0]), Some(value.fields[1])],
            };
        }
    }
}
