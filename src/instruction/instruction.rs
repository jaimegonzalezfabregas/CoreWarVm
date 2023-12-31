use super::{field::Field, op_code::OpCode, op_modifier::OpModifier};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub ptr_range: usize,
    pub core_size: usize,
    pub code: OpCode,
    pub modifier: OpModifier,
    pub fields: [Field; 2],
}

impl Instruction {
    pub fn get_random(ptr_range: usize, core_size: usize) -> Self {
        Self {
            core_size,
            ptr_range,
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
                    SLT /*| LDP | STP*/ => {
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

    pub(crate) fn parse(line: String, core_size: usize) -> Result<Option<Self>, String> {
        let line = match line.find(";") {
            Some(x) => line[0..x].to_string(),
            None => line,
        };

        let line = line.trim_start().to_string();

        if line == "" {
            return Ok(None);
        }

        let (code, line) = OpCode::parse(line.clone().into())?;

        let line = line.trim_start().to_string();

        let (modifier, line) = OpModifier::parse(line.into())?;

        let line = line.trim_start().to_string();

        let (mut a, line) = Field::parse(line.into(), core_size)?;

        let line = line.trim_start().to_string();

        let (mut b, _) = Field::parse(line.into(), core_size)?;

        if let OpCode::DAT = code {
            if b.is_none() {
                (a, b) = (b, a)
            }
        }

        let a = if let Some(a) = a {
            a
        } else {
            Field::default(core_size)
        };

        let b = if let Some(b) = b {
            b
        } else {
            Field::default(core_size)
        };

        Ok(Some(Self {
            core_size,
            ptr_range: core_size,
            code,
            modifier,
            fields: [a, b],
        }))
    }

    #[cfg(not(tarpaulin_include))]
    pub(crate) fn print_state(&self) {
        self.code.print();
        self.modifier.print();
        print!(" ");
        self.fields[0].print();
        print!(", ");
        self.fields[1].print();
    }

    pub(crate) fn get_field(&self, i: usize) -> Field {
        self.fields[i]
    }

    pub(crate) fn mutate(&mut self) {
        *self = Self::get_random(self.ptr_range, self.core_size)
    }
}
