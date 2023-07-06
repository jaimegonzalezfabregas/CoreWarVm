use super::{op_modifier::OpModifier, op_code::OpCode, field::Field};



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunnableInstruction {
    pub code: OpCode,
    pub modifier: OpModifier,
    pub fields: [Field; 2],
}

impl RunnableInstruction {
    pub fn get_random(ptr_range: isize, core_size: isize) -> Self {
        Self {
            code: OpCode::get_random(),
            modifier: OpModifier::get_random(),
            fields: [
                Field::get_random(ptr_range, core_size),
                Field::get_random(ptr_range, core_size),
            ],
        }
    }

    pub fn get_field_transmisions(&self) -> (Vec<(usize, usize)>, bool) {
        match self.modifier {
            OpModifier::A => (vec![(0, 0)], false),
            OpModifier::B => (vec![(1, 1)], false),
            OpModifier::AB => (vec![(0, 1)], false),
            OpModifier::BA => (vec![(1, 0)], false),
            OpModifier::F => (vec![(0, 0), (1, 1)], false),
            OpModifier::X => (vec![(0, 1), (1, 0)], false),
            OpModifier::I => (vec![(0, 0), (1, 1)], true),
            OpModifier::Default => {
                use OpCode::*;
                match self.code {
                    DAT | NOP => (vec![], false),
                    MOV | SEQ | SNE | CMP => {
                        if let Field::Inmediate(_) = self.fields[0] {
                            (vec![(0, 1)], false)
                        } else if let Field::Inmediate(_) = self.fields[1] {
                            (vec![(1, 1)], false)
                        } else {
                            (vec![(0, 0), (1, 1)], true)
                        }
                    }
                    ADD | SUB | MUL | DIV | MOD => {
                        if let Field::Inmediate(_) = self.fields[0] {
                            (vec![(0, 1)], false)
                        } else if let Field::Inmediate(_) = self.fields[1] {
                            (vec![(1, 1)], false)
                        } else {
                            (vec![(0, 0), (1, 1)], false)
                        }
                    }
                    SLT | LDP | STP => {
                        if let Field::Inmediate(_) = self.fields[0] {
                            (vec![(0, 1)], false)
                        } else {
                            (vec![(1, 1)], false)
                        }
                    }
                    JMP | JMZ | JMN | DJN | SPL => (vec![(1, 1)], false),
                }
            }
        }
    }

    pub(crate) fn parse(line: String, core_size: isize) -> Result<Option<Self>, String> {
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

        let (a, line) = Field::parse(line.into(), core_size)?;

        let line = line.trim_start().to_string();

        let (b, _) = Field::parse(line.into(), core_size)?;

        let a = if let Some(a) = a {
            a
        } else {
            return Err("missing field a".into());
        };

        let b = if let Some(b) = b {
            b
        } else {
            return Err("missing field b".into());
        };

        Ok(Some(Self {
            code,
            modifier,
            fields: [a, b],
        }))
    }

    pub(crate) fn print_state(&self) {
        self.code.print();
        self.modifier.print();
        print!(" ");
        self.fields[0].print();
        print!(" ");
        self.fields[1].print();
    }
}
