use rand::seq::SliceRandom;

use crate::{core::CoreRuntime, utils::ModUsize};

use super::decrement::Decrement;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Field {
    Direct(ModUsize),
    Inmediate(ModUsize),
    AIndirect(ModUsize, Decrement),
    BIndirect(ModUsize, Decrement),
}

impl Field {
    pub fn get_random(ptr_range: usize, core_size: usize) -> Field {
        use Field::*;
        [
            Direct(ModUsize::rand(core_size, 0..ptr_range)),
            Inmediate(ModUsize::rand(core_size, 0..core_size)),
            AIndirect(
                ModUsize::rand(core_size, 0..ptr_range),
                Decrement::get_random(),
            ),
            BIndirect(
                ModUsize::rand(core_size, 0..ptr_range),
                Decrement::get_random(),
            ),
        ]
        .choose(&mut rand::thread_rng())
        .unwrap()
        .clone()
    }

    pub fn get_val(&self) -> &ModUsize {
        match self {
            Field::Direct(x) => x,
            Field::Inmediate(x) => x,
            Field::AIndirect(x, _) => x,
            Field::BIndirect(x, _) => x,
        }
    }

    pub fn set_val(&mut self, data: ModUsize) {
        *match self {
            Field::Direct(x) => x,
            Field::Inmediate(x) => x,
            Field::AIndirect(x, _) => x,
            Field::BIndirect(x, _) => x,
        } = data;
    }

    fn decrement(&mut self) {
        self.set_val(*self.get_val() - 1 as usize)
    }

    fn increment(&mut self) {
        self.set_val(*self.get_val() + 1 as usize)
    }

    fn num_parse(line: &str, core_size: usize) -> Result<ModUsize, String> {
        match str::parse::<isize>(line.into()) {
            Ok(i) => Ok(ModUsize::new(i, core_size)),
            Err(_) => Err(format!("parsing number from \"{line}\" failed")),
        }
    }

    pub fn parse(line: String, core_size: usize) -> Result<(Option<Self>, String), String> {
        let line = line.trim();

        if line == "" {
            return Ok((None, "".into()));
        }

        let mut splited = line.split(",");

        let ret: Self;

        if let Some(line) = splited.next() {
            use Decrement::*;

            let line = line.trim();

            // println!("parsing field from {}", line);

            if line.starts_with("#") {
                ret = Self::Inmediate(Self::num_parse(&line[1..], core_size)?);
            } else if line.starts_with("$") {
                ret = Self::Direct(Self::num_parse(&line[1..], core_size)?);
            } else if line.starts_with("*") {
                ret = Self::AIndirect(Self::num_parse(&line[1..], core_size)?, None);
            } else if line.starts_with("@") {
                ret = Self::BIndirect(Self::num_parse(&line[1..], core_size)?, None);
            } else if line.starts_with("{") {
                ret = Self::AIndirect(Self::num_parse(&line[1..], core_size)?, Predecrement);
            } else if line.starts_with("<") {
                ret = Self::BIndirect(Self::num_parse(&line[1..], core_size)?, Predecrement);
            } else if line.starts_with(r"}") {
                ret = Self::AIndirect(Self::num_parse(&line[1..], core_size)?, Postincrement);
            } else if line.starts_with(">") {
                ret = Self::BIndirect(Self::num_parse(&line[1..], core_size)?, Postincrement);
            } else {
                match str::parse::<isize>(line) {
                    Ok(i) => {
                        ret = Self::Direct(ModUsize::new(i, core_size));
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            }
        } else {
            return Ok((None, "".into()));
        }

        Ok((Some(ret), splited.collect::<Vec<&str>>().join(" ")))
    }

    pub fn solve(&self, core: &mut CoreRuntime, ic: ModUsize) -> ModUsize {
        let ret = match (*self, 0, 1) {
            (Field::Direct(p), _, _) => ic + p,
            (Field::Inmediate(_), _, _) => ic,
            (Field::AIndirect(x, m), i, _) | (Field::BIndirect(x, m), _, i) => {
                let inst = core.get_mut_instruction_at(&(ic + x));
                if let Decrement::Predecrement = m {
                    inst.fields[i].decrement()
                }

                let ret = ic + x + *inst.fields[i].get_val();

                if let Decrement::Postincrement = m {
                    inst.fields[i].increment()
                }

                ret
            }
        };

        // println!("[debug]: {self:?} was solved to {ret:?}");

        ret
    }

    pub fn print(&self) {
        match self {
            Field::Direct(x) => print!("{x}"),
            Field::Inmediate(x) => print!("#{x}"),
            Field::AIndirect(x, m) => match m {
                Decrement::None => print!("*{x}"),
                Decrement::Predecrement => print!("{{{x}"),
                Decrement::Postincrement => print!("}}{x}"),
            },
            Field::BIndirect(x, m) => match m {
                Decrement::None => print!("@{x}"),
                Decrement::Predecrement => print!("<{x}"),
                Decrement::Postincrement => print!(">{x}"),
            },
        }
    }

    pub(crate) fn default(core_size: usize) -> Field {
        Self::Direct(ModUsize::new(0, core_size))
    }
}
