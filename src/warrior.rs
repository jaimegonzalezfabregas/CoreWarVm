use rand::Rng;

use crate::{instruction::instruction::Instruction, utils::ModUsize};
    use rand::prelude::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
pub struct Warrior {
    pub org: usize,
    pub name: String,
    pub body: Vec<Instruction>,
    instruction_counters: Vec<ModUsize>,
}

impl Warrior {
    pub fn new_thread(&mut self, ptr: ModUsize) {
        self.instruction_counters.push(ptr);
    }

    pub fn get_counters(&self) -> Vec<ModUsize> {
        self.instruction_counters.clone()
    }

    pub fn random_create(size: usize, core_size: usize) -> Self {
        let mut body = vec![];

        for _ in 0..size {
            // println!("creating random instruction");

            let inst = Instruction::get_random(size, core_size);

            // println!("{inst:?}");

            body.push(inst)
        }

        let org = rand::thread_rng().gen_range(0..size);

        // println!("\n\norg:{org}\n\n");

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

    pub fn set_last_instruction_counter(&mut self, val: ModUsize) {
        let last_i = self.instruction_counters.len() - 1;
        self.instruction_counters[last_i] = val;
    }

    pub fn parse(str: String, name: String, core_size: usize) -> Result<Self, String> {
        let str = str.to_uppercase();

        let mut body = vec![];
        let mut start = None;

        for (i, line) in str.split('\n').enumerate() {
            let line = line.trim();
            if line.starts_with("ORG") {
                if let None = start {
                    start = Some(i);
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

    #[cfg(not(tarpaulin_include))]
    pub(crate) fn print_state_at(&self, line: usize) {
        for (thread_i, ic) in self.instruction_counters.iter().enumerate() {
            if *ic == line {
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


    pub(crate) fn mutate(&self) -> Warrior {
        let mut offspring = self.clone();

        offspring.body.choose_mut(&mut rand::thread_rng()).unwrap().mutate();

        offspring
    }
}
