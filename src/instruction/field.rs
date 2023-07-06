use rand::{seq::SliceRandom, Rng};

use crate::{
    core::CoreRuntime,
    utils::{modulo, ModUsize},
};

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
            Direct(ModUsize::rand(core_size, 0..core_size)),
            Inmediate(ModUsize::rand(core_size, 0..core_size)),
            AIndirect(
                ModUsize::rand(core_size, 0..core_size),
                Decrement::get_random(),
            ),
            BIndirect(
                ModUsize::rand(core_size, 0..core_size),
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

    fn decrement(&mut self, core_size: usize) {
        *self.get_val() -= 1
    }

    fn increment(&mut self, core_size: usize) {
        *self.get_val() += 1
    }

    fn num_parse(line: &str, core_size: usize) -> Result<ModUsize, String> {
        match str::parse::<isize>(line.into()) {
            Ok(i) => Ok(ModUsize::new(i, core_size)),
            Err(_) => Err("parsing number failed".into()),
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
                ret = Self::Inmediate(Self::num_parse(&line[..1], core_size)?);
            } else if line.starts_with("$") {
                ret = Self::Direct(Self::num_parse(&line[..1], core_size)?);
            } else if line.starts_with("*") {
                ret = Self::AIndirect(Self::num_parse(&line[..1], core_size)?, None);
            } else if line.starts_with("@") {
                ret = Self::BIndirect(Self::num_parse(&line[..1], core_size)?, None);
            } else if line.starts_with("{") {
                ret = Self::AIndirect(Self::num_parse(&line[..1], core_size)?, Predecrement);
            } else if line.starts_with("<") {
                ret = Self::BIndirect(Self::num_parse(&line[..1], core_size)?, Predecrement);
            } else if line.starts_with(r"}") {
                ret = Self::AIndirect(Self::num_parse(&line[..1], core_size)?, Postincrement);
            } else if line.starts_with(">") {
                ret = Self::BIndirect(Self::num_parse(&line[..1], core_size)?, Postincrement);
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
        let ret = match *self {
            Field::Direct(p) => ic.inc(p as isize),
            Field::Inmediate(_) => ic,
            Field::AIndirect(x, m) => {
                if let Decrement::Predecrement = m {
                    core.get_instruction_at(&ic.inc(x as isize)).fields[0].decrement(core.core_size)
                }

                let ret = ModUsize::new(
                    ic.inc(x as isize)
                        + (core.get_instruction_at(ic.inc(x as isize)).fields[0].get_val()),
                    core.core_size,
                );

                if let Decrement::Postincrement = m {
                    core[(ic + x) as usize].fields[0].increment(core.core_size);
                }

                ret
            }
            Field::BIndirect(x, m) => {
                if let Decrement::Predecrement = m {
                    core[(ic + x) as usize].fields[1].decrement(core.core_size)
                }

                let ret = ModUsize(
                    ic + x + core[modulo(ic + x, core.len()) as usize].fields[1].get_val(),
                );

                if let Decrement::Postincrement = m {
                    core[(ic + x) as usize].fields[1].increment(core.core_size);
                }

                ret
            }
        };

        // println!("[debug]: {self:?} was solved to {ret:?}");

        ret
    }

    pub fn print(&self, core_size: usize) {
        match self {
            Field::Direct(x) => print!("{}", Self::prettyfy(x, core_size)),
            Field::Inmediate(x) => print!("#{}", Self::prettyfy(x, core_size)),
            Field::AIndirect(x, m) => match m {
                Decrement::None => print!("*{}", Self::prettyfy(x, core_size)),
                Decrement::Predecrement => print!("{{{}", Self::prettyfy(x, core_size)),
                Decrement::Postincrement => print!("}}{}", Self::prettyfy(x, core_size)),
            },
            Field::BIndirect(x, m) => match m {
                Decrement::None => print!("@{}", Self::prettyfy(x, core_size)),
                Decrement::Predecrement => print!("<{}", Self::prettyfy(x, core_size)),
                Decrement::Postincrement => print!(">{}", Self::prettyfy(x, core_size)),
            },
        }
    }

    fn prettyfy(x: &usize, core_size: usize) -> isize {
        if x > core_size / 2 {
            x - core_size
        } else {
            x
        }
    }

    pub(crate) fn default() -> Field {
        Self::Direct(0)
    }
}
