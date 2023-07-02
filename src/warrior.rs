use crate::{op::Instruction, utils::modulo};

#[derive(Debug, Clone)]
pub struct Warrior {
    pub org: isize,
    pub name: String,
    pub body: Vec<Instruction>,
    pub instruction_counters: Vec<isize>,
}

impl Warrior {
    pub fn new_thread(&mut self, ptr: isize) {
        self.instruction_counters.push(ptr);
    }

    pub fn random_create(size: isize, core_size: isize) -> Self {
        let mut body = vec![];

        for _ in 0..size {
            body.push(Instruction::get_random(size, core_size as usize))
        }

        Warrior {
            org: modulo(rand::random::<usize>(), size) as isize,
            name: "random".into(),
            body,
            instruction_counters: vec![],
        }
    }

    pub fn get_next_instruction_counter(&mut self) -> isize {
        let ret = self.instruction_counters[0];
        self.instruction_counters.rotate_left(1);
        ret
    }

    pub fn set_instruction_counter(&mut self, val: isize, core_size: isize) {
        self.instruction_counters[0] = modulo(val, core_size) as isize;
    }

    pub fn parse(str: String, name: String, core_size: isize) -> Result<Self, String> {
        let str = str.to_uppercase();

        let mut body = vec![];
        let mut start = None;

        for (i, line) in str.split('\n').enumerate() {
            if line == "ORG" {
                if let None = start {
                    start = Some(i as isize);
                } else {
                    return Err(format!("linea {i}: multiple ORG pseudoinstructions found"));
                }
            } else {
                match Instruction::parse(line.into(), core_size) {
                    Ok(None) => (),
                    Ok(Some(op)) => body.push(op),
                    Err(err) => return Err(format!("linea {i}: {err}")),
                }
            }
        }

        Ok(Self {
            org: start.unwrap_or_else(|| 0),
            name,
            instruction_counters: vec![],
            body,
        })
    }

    pub(crate) fn print_state_at(&self, i: usize) {
        for ic in self.instruction_counters.iter() {
            if i as isize == *ic {
                print!(" < {}", self.name);
            }
        }
    }
}
