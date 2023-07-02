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

#[derive(Debug)]
pub struct CorePtr(pub isize);

impl CoreRuntime {
    pub fn done(&self) -> bool {
        self.warriors.len() == 0
    }

    pub fn tick(&mut self) {
        if self.warriors.len() == 0 {
            return;
        }
        let instruction_counter = self.warriors[0].get_next_instruction_counter();

        let instruction = self.core[instruction_counter as usize];

        let field_a_solution = instruction.fields[0].solve(&mut self.core, instruction_counter);
        let field_b_solution = instruction.fields[1].solve(&mut self.core, instruction_counter);

        let mut die = false;

        // println!("[debug]: instruction is {:?}", instruction);

        let mut next_instruction = instruction_counter + 1;

        match instruction.code {
            OpCode::DAT => {
                die = true;
            }
            OpCode::MOV => {
                let src = field_a_solution;
                let dst = field_b_solution;

                let (pipes, i_flag) = instruction.get_field_transmisions();

                if i_flag {
                    let instruction = self.get_instruction_at(&src);
                    self.set_instruction_at(&dst, instruction);
                } else {
                    for (i_src, i_dst) in pipes {
                        let data = self.read_field(&src, i_src);
                        self.write_field(&dst, i_dst, data);
                    }
                }
            }
            OpCode::ADD => {
                let src = field_a_solution;
                let dst = field_b_solution;

                let (pipes, _) = instruction.get_field_transmisions();

                for (i_src, i_dst) in pipes {
                    // println!("[debug]: pipe from {} to {} ", i_src, i_dst);
                    let operand = self.read_field(&src, i_src);
                    let old_value = self.read_field(&dst, i_dst);
                    self.write_field(
                        &dst,
                        i_dst,
                        modulo(old_value + operand, self.core_size) as isize,
                    );
                }
            }
            OpCode::SUB => {
                let src = field_a_solution;
                let dst = field_b_solution;

                let (pipes, _) = instruction.get_field_transmisions();

                for (i_src, i_dst) in pipes {
                    let operand = self.read_field(&src, i_src);
                    let old_value = self.read_field(&dst, i_dst);
                    self.write_field(
                        &dst,
                        i_dst,
                        modulo(old_value - operand, self.core_size) as isize,
                    );
                }
            }
            OpCode::MUL => {
                let src = field_a_solution;
                let dst = field_b_solution;

                let (pipes, _) = instruction.get_field_transmisions();

                for (i_src, i_dst) in pipes {
                    let operand = self.read_field(&src, i_src);
                    let old_value = self.read_field(&dst, i_dst);
                    self.write_field(
                        &dst,
                        i_dst,
                        modulo(old_value * operand, self.core_size) as isize,
                    );
                }
            }
            OpCode::DIV => {
                let src = field_a_solution;
                let dst = field_b_solution;

                let (pipes, _) = instruction.get_field_transmisions();

                for (i_src, i_dst) in pipes {
                    let operand = self.read_field(&src, i_src);
                    let old_value = self.read_field(&dst, i_dst);
                    if operand == 0 {
                        die = true;
                    } else {
                        self.write_field(
                            &dst,
                            i_dst,
                            modulo(old_value / operand, self.core_size) as isize,
                        );
                    }
                }
            }
            OpCode::MOD => {
                let src = field_a_solution;
                let dst = field_b_solution;

                let (pipes, _) = instruction.get_field_transmisions();

                for (i_src, i_dst) in pipes {
                    let operand = self.read_field(&src, i_src);
                    let old_value = self.read_field(&dst, i_dst);
                    if operand == 0 {
                        die = true;
                    } else {
                        self.write_field(
                            &dst,
                            i_dst,
                            modulo(old_value / operand, self.core_size) as isize,
                        );
                    }
                }
            }
            OpCode::JMP => next_instruction = field_a_solution.0,
            OpCode::JMZ => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = true;

                for (_, i_dst) in pipes {
                    if self.read_field(&field_b_solution, i_dst) != 0 {
                        jump = false;
                    }
                }

                if jump {
                    next_instruction = field_a_solution.0;
                }
            }
            OpCode::JMN => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = false;

                for (_, i_dst) in pipes {
                    if self.read_field(&field_b_solution, i_dst) != 0 {
                        jump = true;
                    }
                }

                if jump {
                    next_instruction = field_a_solution.0;
                }
            }
            OpCode::DJN => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = false;

                for (_, i_dst) in pipes {
                    let val = self.read_field(&field_b_solution, i_dst) - 1;
                    self.write_field(&field_b_solution, i_dst, val);

                    if val != 0 {
                        jump = true;
                    }
                }

                if jump {
                    next_instruction = field_a_solution.0;
                }
            }
            OpCode::SPL => {
                self.warriors[0].new_thread(field_a_solution.0);
            }
            OpCode::CMP | OpCode::SEQ => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = true;

                for (i_src, i_dst) in pipes {
                    if self.read_field(&field_b_solution, i_dst)
                        != self.read_field(&field_a_solution, i_src)
                    {
                        jump = false;
                    }
                }

                if jump {
                    next_instruction += 1;
                }
            }
            OpCode::SNE => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = false;

                for (i_src, i_dst) in pipes {
                    if self.read_field(&field_b_solution, i_dst)
                        != self.read_field(&field_a_solution, i_src)
                    {
                        jump = true;
                    }
                }

                if jump {
                    next_instruction += 1;
                }
            }
            OpCode::SLT => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = false;

                for (i_src, i_dst) in pipes {
                    if self.read_field(&field_a_solution, i_src)
                        < self.read_field(&field_b_solution, i_dst)
                    {
                        jump = true;
                    }
                }

                if jump {
                    next_instruction += 1;
                }
            }
            OpCode::LDP => todo!(),
            OpCode::STP => todo!(),
            OpCode::NOP => (),
        }

        self.warriors[0].set_instruction_counter(next_instruction, self.core.len() as isize);

        if die {
            self.warriors.remove(0);
        } else {
            self.warriors.rotate_left(1);
        }
    }

    fn get_instruction_at(&self, ptr: &CorePtr) -> Instruction {
        match *ptr {
            CorePtr(i) => self.core[modulo(i, self.core_size)],
        }
    }

    fn set_instruction_at(&mut self, ptr: &CorePtr, instruction: Instruction) {
        match *ptr {
            CorePtr(i) => self.core[modulo(i, self.core_size)] = instruction,
        };
    }

    fn read_field(&self, ptr: &CorePtr, i_field: usize) -> isize {
        match *ptr {
            CorePtr(i) => self.core[modulo(i, self.core_size)].fields[i_field].get_val(),
        }
    }

    fn write_field(&mut self, ptr: &CorePtr, i_field: usize, data: isize) {
        match *ptr {
            CorePtr(i) => self.core[modulo(i, self.core_size)].fields[i_field].set_val(data),
        }
    }

    pub(crate) fn print_state(&self) {
        for (i, cell) in self.core.iter().enumerate() {
            print!("{i:0>6}: ");
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
                fields: [Field::Direct(0), Field::Direct(0)],
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
        mut warrior: Warrior,
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

            warrior.new_thread(warrior.org + deploy_position);

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
                    warrior.new_thread(warrior.org + deploy_position);

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
