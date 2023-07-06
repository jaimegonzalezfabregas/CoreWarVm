use rand::Rng;

use crate::{core::ModUsize, instruction::instruction::Instruction, utils::modulo};

#[derive(Debug, Clone)]
pub struct Warrior {
    pub org: isize,
    pub name: String,
    pub body: Vec<Instruction>,
    pub instruction_counters: Vec<ModUsize>,
}

impl Warrior {
    pub fn new_thread(&mut self, ptr: ModUsize) {
        self.instruction_counters.push(ptr);
    }

    pub fn random_create(size: isize, core_size: isize) -> Self {
        let mut body = vec![];

        for _ in 0..size {
            println!("creating random instruction");

            let inst = Instruction::get_random(size, core_size);

            println!("{inst:?}");

            body.push(inst)
        }

        let org = rand::thread_rng().gen_range(0..size);

        println!("\n\norg:{org}\n\n");

        Warrior {
            org,
            name: "random".into(),
            body,
            instruction_counters: vec![],
        }
    }

    pub fn get_next_instruction_counter(&mut self) -> ModUsize {
        let ret = self.instruction_counters[0];
        self.instruction_counters.rotate_left(1);
        ret
    }

    pub fn set_instruction_counter(&mut self, val: ModUsize) {
        self.instruction_counters[0] = val;
    }

    pub fn parse(str: String, name: String, core_size: isize) -> Result<Self, String> {
        let str = str.to_uppercase();

        let mut body = vec![];
        let mut start = None;

        for (i, line) in str.split('\n').enumerate() {
            let line = line.trim();
            if line.starts_with("ORG") {
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
        for (thread_i, ic) in self.instruction_counters.iter().enumerate() {
            if i as isize == *ic {
                print!(" < {}({thread_i})", self.name);
            }
        }
    }

    pub(crate) fn kill_thread(&mut self) {
        let last_i = self.instruction_counters.len() - 1;
        self.instruction_counters.remove(last_i);
    }

    pub(crate) fn dead(&self) -> bool {
        self.instruction_counters.len() == 0
    }
}
