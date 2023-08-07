use crate::{
    instruction::{
        field::Field, instruction::Instruction, op_code::OpCode, op_modifier::OpModifier,
    },
    utils::{modulo, ModUsize},
    warrior::Warrior,
};

#[derive(Debug)]
pub struct CoreRuntime {
    pub core_size: usize,
    pub core: Vec<Instruction>,
    pub warriors: Vec<Warrior>,
}
#[derive(Debug, Clone)]

pub struct CoreConfig {
    core_size: usize,
    warrior_data: Vec<(ModUsize, Warrior)>,
}

impl CoreRuntime {
    pub fn done(&self) -> bool {
        self.warriors.len() == 0
    }

    pub fn tick(&mut self) {
        if self.warriors.len() == 0 {
            return;
        }

        let instruction_counter = self.warriors[0].get_next_instruction_counter();

        let instruction = self.get_instruction_at(&instruction_counter).clone();

        let field_a_solution = instruction.fields[0].solve(self, instruction_counter);

        let instruction_a = self.get_instruction_at(&field_a_solution).clone();

        let field_b_solution = instruction.fields[1].solve(self, instruction_counter);

        let instruction_b = self.get_instruction_at(&field_b_solution).clone();

        // print!("warrior {} is going to execute ", self.warriors[0].name);

        // instruction.print_state();

        // println!("");

        let mut die = false;

        // println!("[debug]: instruction is {:?}", instruction);

        let mut next_instruction = instruction_counter.inc(1);

        match instruction.code {
            OpCode::DAT => {
                die = true;
            }
            OpCode::MOV => {
                let (pipes, i_flag) = instruction.get_field_transmisions();

                if i_flag {
                    self.set_instruction_at(&field_b_solution, instruction_a);
                } else {
                    for (i_src, i_dst) in pipes {
                        let data = instruction_a.get_field(i_src);
                        self.write_field_val(&field_b_solution, i_dst, *data.get_val());
                    }
                }
            }
            OpCode::ADD => {
                let (pipes, _) = instruction.get_field_transmisions();

                for (i_src, i_dst) in pipes {
                    // println!("[debug]: pipe from {} to {} ", i_src, i_dst);
                    let operand = *instruction_a.get_field(i_src).get_val();
                    let old_value = *instruction_b.get_field(i_dst).get_val();
                    self.write_field_val(&field_b_solution, i_dst, old_value + operand);
                }
            }
            OpCode::SUB => {
                let (pipes, _) = instruction.get_field_transmisions();

                for (i_src, i_dst) in pipes {
                    let operand = *instruction_a.get_field(i_src).get_val();
                    let old_value = *instruction_b.get_field(i_dst).get_val();
                    self.write_field_val(&field_b_solution, i_dst, old_value - operand);
                }
            }
            OpCode::MUL => {
                let (pipes, _) = instruction.get_field_transmisions();

                for (i_src, i_dst) in pipes {
                    let operand = *instruction_a.get_field(i_src).get_val();
                    let old_value = *instruction_b.get_field(i_dst).get_val();
                    self.write_field_val(&field_b_solution, i_dst, old_value * operand);
                }
            }
            OpCode::DIV => {
                let (pipes, _) = instruction.get_field_transmisions();

                for (i_src, i_dst) in pipes {
                    let operand = *instruction_a.get_field(i_src).get_val();
                    let old_value = *instruction_b.get_field(i_dst).get_val();
                    if operand == 0 {
                        die = true;
                    } else {
                        self.write_field_val(&field_b_solution, i_dst, old_value / operand);
                    }
                }
            }
            OpCode::MOD => {
                let (pipes, _) = instruction.get_field_transmisions();

                for (i_src, i_dst) in pipes {
                    let operand = *instruction_a.get_field(i_src).get_val();
                    let old_value = *instruction_b.get_field(i_dst).get_val();
                    if operand == 0 {
                        die = true;
                    } else {
                        self.write_field_val(
                            &field_b_solution,
                            i_dst,
                            ModUsize::new(
                                old_value.val as isize % operand.val as isize,
                                operand.congruence,
                            ),
                        );
                    }
                }
            }
            OpCode::JMP => next_instruction = field_a_solution,
            OpCode::JMZ => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = true;

                for (_, i_dst) in pipes {
                    if *instruction_b.get_field(i_dst).get_val() != 0 {
                        jump = false;
                    }
                }

                if jump {
                    next_instruction = field_a_solution;
                }
            }
            OpCode::JMN => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = false;

                for (_, i_dst) in pipes {
                    if *instruction_b.get_field(i_dst).get_val() != 0 {
                        jump = true;
                    }
                }

                if jump {
                    next_instruction = field_a_solution;
                }
            }
            OpCode::DJN => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = false;

                for (_, i_dst) in pipes {
                    let val = *instruction_b.get_field(i_dst).get_val() - 1 as usize;
                    self.write_field_val(&field_b_solution, i_dst, val);

                    if val != 0 {
                        jump = true;
                    }
                }

                if jump {
                    next_instruction = field_a_solution;
                }
            }
            OpCode::SPL => (),
            OpCode::CMP | OpCode::SEQ => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = true;

                for (i_src, i_dst) in pipes {
                    let a = *instruction_a.get_field(i_src).get_val();
                    let b = *instruction_b.get_field(i_dst).get_val();
                    if a != b {
                        jump = false;
                    }
                }

                if jump {
                    next_instruction = next_instruction.inc(1);
                }
            }
            OpCode::SNE => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = false;

                for (i_src, i_dst) in pipes {
                    let a = *instruction_a.get_field(i_src).get_val();
                    let b = *instruction_b.get_field(i_dst).get_val();
                    if a != b {
                        jump = true;
                    }
                }

                if jump {
                    next_instruction = next_instruction.inc(1);
                }
            }
            OpCode::SLT => {
                let (pipes, _) = instruction.get_field_transmisions();

                let mut jump = false;

                for (i_src, i_dst) in pipes {
                    let a = *instruction_a.get_field(i_src).get_val();
                    let b = *instruction_b.get_field(i_dst).get_val();
                    if a.val < b.val {
                        jump = true;
                    }
                }

                if jump {
                    next_instruction = next_instruction.inc(1);
                }
            }
            // OpCode::LDP => todo!(),
            // OpCode::STP => todo!(),
            OpCode::NOP => (),
        }

        self.warriors[0].set_last_instruction_counter(next_instruction);

        if let OpCode::SPL = instruction.code {
            // println!("creating new thread at: {:?}", field_a_solution);
            self.warriors[0].new_thread(field_a_solution);
        }

        if die {
            self.warriors[0].kill_thread();
        }

        if self.warriors[0].dead() {
            self.warriors.remove(0);
        } else {
            self.warriors.rotate_left(1);
        }
    }

    pub fn get_instruction_at(&self, ptr: &ModUsize) -> &Instruction {
        &self.core[ptr.val]
    }

    pub fn get_mut_instruction_at(&mut self, ptr: &ModUsize) -> &mut Instruction {
        &mut self.core[ptr.val]
    }

    fn set_instruction_at(&mut self, ptr: &ModUsize, instruction: Instruction) {
        self.core[ptr.val] = instruction;
    }

    fn write_field_val(&mut self, ptr: &ModUsize, i_field: usize, data: ModUsize) {
        self.core[ptr.val].fields[i_field].set_val(data)
    }

    #[cfg(not(tarpaulin_include))]
    pub(crate) fn print_state(&self, range: Option<std::ops::Range<usize>>) {
        for w in &self.warriors {
            println!("{}: {:?}", w.name, w.get_counters())
        }

        let range = if let Some(range) = range {
            range
        } else {
            0..self.core_size as usize
        };

        for (i, cell) in self.core.iter().enumerate() {
            if range.contains(&i) {
                //print!("{i:0>6}: ");
                cell.print_state();
                for warr in self.warriors.iter() {
                    warr.print_state_at(i);
                }

                print!("\n");
            }
        }
    }
}

impl CoreConfig {
    pub fn new(core_size: usize) -> Self {
        Self {
            core_size,
            warrior_data: vec![],
        }
    }

    pub fn brawl(&self) -> CoreRuntime {
        let mut core = vec![
            Instruction {
                ptr_range: 0,
                core_size: 0,
                code: OpCode::DAT,
                fields: [
                    Field::Direct(ModUsize::new(0, self.core_size)),
                    Field::Direct(ModUsize::new(0, self.core_size))
                ],
                modifier: OpModifier::Default,
            };
            self.core_size as usize
        ];

        for (deploy_position, warrior) in self.warrior_data.iter() {
            for (i, op) in warrior.body.iter().enumerate() {
                core[modulo(deploy_position.val + i, self.core_size)] = op.to_owned();
            }
        }

        // println!("BRAAAAWL!");

        CoreRuntime {
            core_size: self.core_size,
            core,
            warriors: self.warrior_data.iter().map(|(_, e)| e).cloned().collect(),
        }
    }
    pub fn deploy(
        &mut self,
        mut warrior: Warrior,
        input_position: Option<ModUsize>,
    ) -> Result<(), String> {
        let w_len = warrior.body.len();
        let core_size = self.core_size;

        if let Some(deploy_position) = input_position {
            for (position, warrior) in self.warrior_data.iter() {
                if check_segment_colision(&deploy_position, w_len, position, warrior.body.len()) {
                    return Err("Forced deploy position was already ocupied".into());
                }
            }

            warrior.new_thread(deploy_position + warrior.org);

            self.warrior_data.push((deploy_position, warrior));

            return Ok(());
        } else {
            for _ in 0..self.core_size * 2 {
                let deploy_position = ModUsize::rand(core_size, 0..core_size);
                let mut valid_pos = true;

                for (position, warrior) in self.warrior_data.iter() {
                    if check_segment_colision(&deploy_position, w_len, position, warrior.body.len())
                    {
                        valid_pos = false;
                        break;
                    }
                }

                if valid_pos {
                    warrior.new_thread(deploy_position + warrior.org);

                    self.warrior_data.push((deploy_position, warrior));

                    return Ok(());
                }
            }

            Err("Core is likely full of warriors allready, cant deploy any more".into())
        }
    }
}

fn check_segment_colision(
    start_a: &ModUsize,
    len_a: usize,
    start_b: &ModUsize,
    len_b: usize,
) -> bool {
    if len_a < len_b {
        check_segment_colision(&start_b, len_b, &start_a, len_a)
    } else {
        // len_a > len_b
        let n_start_b = start_b.inc(-(start_a.val as isize));
        let n_end_b = start_b.inc(len_b as isize);

        (n_start_b.val <= len_b) || (n_end_b.val <= len_b)
    }
}
