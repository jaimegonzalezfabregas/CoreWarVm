use crate::{op::Instruction, utils::modulo};

#[derive(Debug, Clone)]
pub struct Warrior {
    pub name: String,
    pub body: Vec<Instruction>,
    pub instruction_counters: Vec<isize>,
}

impl Warrior {
    pub fn random_create(size: isize, core_size: isize) -> Self {
        let mut body = vec![];

        for _ in 0..size {
            body.push(Instruction::get_random(size, core_size as usize))
        }

        Warrior {
            name: "random".into(),
            body,
            instruction_counters: vec![modulo(rand::random::<usize>(), size) as isize],
        }
    }

    pub fn get_next_instruction_counter(&mut self) -> isize {
        self.instruction_counters.rotate_left(1);
        self.instruction_counters[0]
    }

    pub fn instruction_counter_jump(&mut self, acc: isize, core_size: isize) {
        self.instruction_counters[0] += acc;
        self.instruction_counters[0] = modulo(self.instruction_counters[0], core_size) as isize;
    }

    pub fn parse(str: String, name: String) -> Result<Self, String> {
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
                match Instruction::parse(line.into()) {
                    Ok(op) => body.push(op),
                    Err(err) => return Err(format!("linea {i}: {err}")),
                }
            }
        }

        Ok(Self {
            name,
            instruction_counters: vec![start.unwrap_or_else(|| 0)],
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
