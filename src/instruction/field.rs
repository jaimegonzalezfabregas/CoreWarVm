use rand::Rng;

use crate::{core::CorePtr, utils::modulo};

use super::{decrement::Decrement, instruction::Instruction};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Field {
    Direct(isize),
    Inmediate(isize),
    AIndirect(isize, Decrement),
    BIndirect(isize, Decrement),
}

impl Field {
    pub fn get_random(ptr_range: isize, core_size: isize) -> Field {
        use Field::*;
        [
            Direct(rand::thread_rng().gen_range(0..ptr_range)),
            Inmediate(rand::thread_rng().gen_range(0..core_size)),
            AIndirect(
                rand::thread_rng().gen_range(0..ptr_range),
                Decrement::get_random(),
            ),
            BIndirect(
                rand::thread_rng().gen_range(0..ptr_range),
                Decrement::get_random(),
            ),
        ][rand::random::<usize>() % 4]
    }

    pub fn get_val(&self) -> isize {
        *match self {
            Field::Direct(x) => x,
            Field::Inmediate(x) => x,
            Field::AIndirect(x, _) => x,
            Field::BIndirect(x, _) => x,
        }
    }

    pub fn set_val(&mut self, data: isize, core_size: isize) {
        *match self {
            Field::Direct(x) => x,
            Field::Inmediate(x) => x,
            Field::AIndirect(x, _) => x,
            Field::BIndirect(x, _) => x,
        } = modulo(data, core_size) as isize;
    }

    fn decrement(&mut self, core_size: isize) {
        self.set_val(self.get_val() - 1, core_size)
    }

    fn increment(&mut self, core_size: isize) {
        self.set_val(self.get_val() + 1, core_size)
    }

    pub fn parse(line: String, core_size: isize) -> Result<(Option<Self>, String), String> {
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
                match str::parse::<isize>(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::Inmediate(modulo(i, core_size) as isize);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("$") {
                match str::parse::<isize>(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::Direct(modulo(i, core_size) as isize);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("*") {
                match str::parse::<isize>(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::AIndirect(modulo(i, core_size) as isize, None);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("@") {
                match str::parse::<isize>(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::BIndirect(modulo(i, core_size) as isize, None);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("{") {
                match str::parse::<isize>(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::AIndirect(modulo(i, core_size) as isize, Predecrement);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("<") {
                match str::parse::<isize>(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::BIndirect(modulo(i, core_size) as isize, Predecrement);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with(r"}") {
                match str::parse::<isize>(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::AIndirect(modulo(i, core_size) as isize, Postincrement);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with(">") {
                match str::parse::<isize>(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::BIndirect(modulo(i, core_size) as isize, Postincrement);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else {
                match str::parse::<isize>(line) {
                    Ok(i) => {
                        ret = Self::Direct(modulo(i, core_size) as isize);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            }
        } else {
            return Ok((None, "".into()));
        }

        Ok((Some(ret), splited.collect::<Vec<&str>>().join(" ")))
    }

    pub fn solve(&self, core: &mut Vec<Instruction>, ic: isize) -> CorePtr {
        let core_size = core.len() as isize;
        let ret = match self {
            Field::Direct(p) => CorePtr(ic + p),
            Field::Inmediate(_) => CorePtr(ic),
            Field::AIndirect(x, m) => {
                if let Decrement::Predecrement = m {
                    core[(ic + x) as usize].fields[0].decrement(core_size)
                }

                let ret = CorePtr(ic + x + core[(ic + x) as usize].fields[0].get_val());

                if let Decrement::Postincrement = m {
                    core[(ic + x) as usize].fields[0].increment(core_size);
                }

                ret
            }
            Field::BIndirect(x, m) => {
                if let Decrement::Predecrement = m {
                    core[(ic + x) as usize].fields[1].decrement(core_size)
                }

                let ret =
                    CorePtr(ic + x + core[modulo(ic + x, core.len()) as usize].fields[1].get_val());

                if let Decrement::Postincrement = m {
                    core[(ic + x) as usize].fields[1].increment(core_size);
                }

                ret
            }
        };

        // println!("[debug]: {self:?} was solved to {ret:?}");

        ret
    }

    pub fn print(&self, core_size: isize) {
        match self {
            Field::Direct(x) => print!("{}", Self::t(x, core_size)),
            Field::Inmediate(x) => print!("#{}", Self::t(x, core_size)),
            Field::AIndirect(x, m) => match m {
                Decrement::None => print!("*{}", Self::t(x, core_size)),
                Decrement::Predecrement => print!("{{{}", Self::t(x, core_size)),
                Decrement::Postincrement => print!("}}{}", Self::t(x, core_size)),
            },
            Field::BIndirect(x, m) => match m {
                Decrement::None => print!("@{}", Self::t(x, core_size)),
                Decrement::Predecrement => print!("<{}", Self::t(x, core_size)),
                Decrement::Postincrement => print!(">{}", Self::t(x, core_size)),
            },
        }
    }

    fn t(&x: &isize, core_size: isize) -> isize {
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
