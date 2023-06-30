use crate::{
    op::{Field, Instruction, OpCode, OpModifier},
    utils::modulo,
    warrior::Warrior,
};

#[derive(Debug)]
pub struct CoreRuntime {
    pub core_size: isize,
    pub core: Vec<Instruction>,
    pub warriors: Vec<Warrior>,
}
#[derive(Debug, Clone)]

pub struct CoreConfig {
    core_size: isize,
    warrior_data: Vec<(isize, Warrior)>,
}

pub enum CorePtr {
    Cell(isize),
    ToVirtualDAT(isize),
}

impl CoreRuntime {
    pub fn tick(&mut self) {
        if self.warriors.len() == 0 {
            return;
        }
        let instruction_counter = self.warriors[0].get_next_instruction_counter();

        let instruction = self.core[instruction_counter as usize];

        let field_a_solution = instruction.fields[0].solve(&mut self.core, instruction_counter);
        let field_b_solution = instruction.fields[1].solve(&mut self.core, instruction_counter);

        let mut die = false;

        match instruction.code {
            OpCode::DAT => {
                die = true;
            }
            OpCode::MOV => {
                let src = field_a_solution;
                let dst = field_b_solution;

                if let OpModifier::I = instruction.modifier {
                    let instruction = self.get_instruction_at(&src);
                    self.set_instruction_at(&dst, instruction);
                } else {
                    for (i_src, i_dst) in instruction.get_field_transmisions() {
                        let data = self.read_field(&src, i_src);
                        self.write_field(&dst, i_dst, data);
                    }
                }
            }
            OpCode::ADD => {
                let src = field_a_solution;
                let dst = field_b_solution;

                if let OpModifier::I = instruction.modifier {
                    let instruction = self.get_instruction_at(&src);
                    self.set_instruction_at(&dst, instruction);
                } else {
                    for (i_src, i_dst) in instruction.get_field_transmisions() {
                        let data = self.read_field(&src, i_src);
                        self.write_field(&dst, i_dst, data);
                    }
                }
            }
            OpCode::SUB => todo!(),
            OpCode::MUL => todo!(),
            OpCode::DIV => todo!(),
            OpCode::MOD => todo!(),
            OpCode::JMP => todo!(),
            OpCode::JMZ => todo!(),
            OpCode::JMN => todo!(),
            OpCode::DJN => todo!(),
            OpCode::SPL => todo!(),
            OpCode::CMP => todo!(),
            OpCode::SEQ => todo!(),
            OpCode::SNE => todo!(),
            OpCode::SLT => todo!(),
            OpCode::LDP => todo!(),
            OpCode::STP => todo!(),
            OpCode::NOP => todo!(),
        }

        if let OpCode::JMP = instruction.code {
            self.warriors[0].instruction_counter_jump(1, self.core.len() as isize);
        } else {
            self.warriors[0].instruction_counter_jump(1, self.core.len() as isize);
        }

        if die {
            self.warriors.remove(0);
        } else {
            self.warriors.rotate_left(1);
        }
    }

    fn get_instruction_at(&self, ptr: &CorePtr) -> Instruction {
        match *ptr {
            CorePtr::Cell(i) => self.core[modulo(i, self.core_size)],
            CorePtr::ToVirtualDAT(d) => Instruction {
                code: OpCode::DAT,
                modifier: OpModifier::Default,
                fields: [Field::Inmediate(d), Field::Inmediate(d)],
            },
        }
    }

    fn set_instruction_at(&mut self, ptr: &CorePtr, instruction: Instruction) {
        match *ptr {
            CorePtr::Cell(i) => self.core[modulo(i, self.core_size)] = instruction,
            CorePtr::ToVirtualDAT(_) => (),
        };
    }

    fn read_field(&self, ptr: &CorePtr, i_field: usize) -> isize {
        match *ptr {
            CorePtr::Cell(i) => self.core[modulo(i, self.core_size)].fields[i_field].get_val(),
            CorePtr::ToVirtualDAT(d) => d,
        }
    }

    fn write_field(&mut self, ptr: &CorePtr, i_field: usize, data: isize) {
        match *ptr {
            CorePtr::Cell(i) => self.core[modulo(i, self.core_size)].fields[i_field].set_val(data),
            CorePtr::ToVirtualDAT(_) => (),
        }
    }

    pub(crate) fn print_state(&self) {
        for (i, cell) in self.core.iter().enumerate() {
            cell.print_state();
            for warr in self.warriors.iter() {
                warr.print_state_at(i);
            }

            print!("\n");
        }
    }
}

impl CoreConfig {
    pub fn new(core_size: isize) -> Self {
        Self {
            core_size,
            warrior_data: vec![],
        }
    }

    pub fn brawl(&self) -> CoreRuntime {
        println!("brawl called to {self:#?}!");

        let mut core = vec![
            Instruction {
                code: OpCode::DAT,
                fields: [Field::Inmediate(0), Field::Inmediate(0)],
                modifier: OpModifier::Default,
            };
            self.core_size as usize
        ];

        for (deploy_position, warrior) in self.warrior_data.iter() {
            for (i, op) in warrior.body.iter().enumerate() {
                core[modulo(*deploy_position + i as isize, self.core_size)] = *op;
            }
        }

        println!("BRAAAAWL!");

        CoreRuntime {
            core_size: self.core_size,
            core,
            warriors: self.warrior_data.iter().map(|(_, e)| e).cloned().collect(),
        }
    }
    pub fn deploy(
        &mut self,
        warrior: Warrior,
        input_position: Option<isize>,
    ) -> Result<(), String> {
        let w_len = warrior.body.len() as isize;
        let core_size = self.core_size;

        if let Some(deploy_position) = input_position {
            for (position, warrior) in self.warrior_data.iter() {
                if check_segment_colision(
                    deploy_position,
                    w_len,
                    *position,
                    warrior.body.len() as isize,
                    core_size,
                ) {
                    return Err("Forced deploy position was already ocupied".into());
                }
            }

            self.warrior_data.push((deploy_position, warrior));

            return Ok(());
        } else {
            loop {
                let deploy_position = modulo(rand::random::<isize>(), self.core_size) as isize;
                let mut valid_pos = true;

                for (position, warrior) in self.warrior_data.iter() {
                    if check_segment_colision(
                        deploy_position,
                        w_len,
                        *position,
                        warrior.body.len() as isize,
                        core_size,
                    ) {
                        valid_pos = false;
                        break;
                    }
                }

                if valid_pos {
                    self.warrior_data.push((deploy_position, warrior));

                    return Ok(());
                }
            }
        }
    }
}

fn check_segment_colision(
    start_a: isize,
    len_a: isize,
    start_b: isize,
    len_b: isize,
    congruence: isize,
) -> bool {
    if len_a < len_b {
        check_segment_colision(start_b, len_b, start_a, len_a, congruence)
    } else {
        // len_a > len_b
        let n_start_b = (start_b - start_a) % congruence;
        let n_end_b = (n_start_b + len_b) % congruence;

        (n_start_b >= 0 && n_start_b <= len_b) || (n_end_b >= 0 && n_end_b <= len_b)
    }
}
