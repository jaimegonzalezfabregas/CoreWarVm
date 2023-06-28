use crate::{
    op::{Field, OpCode, OpModifier, Operation},
    warrior::Warrior,
};

pub struct Core {
    next_turn: usize,
    core: Vec<Operation>,
    warrior_data: Vec<(isize, Warrior)>,
}

impl Core {
    pub fn new(size: usize) -> Self {
        Self {
            core: vec![
                Operation {
                    code: OpCode::DAT,
                    a: Field::Inmediate(0),
                    b: Field::Inmediate(0),
                    modifier: OpModifier::default(),
                };
                size
            ],
            warrior_data: vec![],
            next_turn: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.core.len()
    }

    pub fn deploy(&mut self, warrior: Warrior) {
        let w_len = warrior.body.len() as isize;
        let core_size = self.len();

        loop {
            let deploy_position = rand::random::<isize>() % core_size as isize;

            for (position, warrior) in self.warrior_data.iter() {
                if check_segment_colision(
                    deploy_position,
                    w_len,
                    *position,
                    warrior.body.len() as isize,
                    self.core.len() as isize,
                ) {
                    continue;
                }
            }

            for (i, op) in warrior.body.iter().enumerate() {
                self.core[(deploy_position as usize + i) % core_size] = *op;
            }

            return;
        }
    }

    pub fn tick(&mut self) {
        let warrior = &self.warrior_data[self.next_turn];
        self.next_turn += 1;
        self.next_turn %= self.warrior_data.len()
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
