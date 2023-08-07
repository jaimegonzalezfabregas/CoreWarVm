mod core;
mod instruction;
mod test;
mod utils;
mod warrior;

use utils::ModUsize;
use warrior::Warrior;

const CORE_SIZE: usize = 8000;

const POOL_SIZE: usize = 10;

const ROUNDS: usize = 100;

fn main() -> Result<(), String> {
    let mut pool: Vec<Warrior> = (0..POOL_SIZE)
        .map(|_| Warrior::random_create(20, CORE_SIZE))
        .collect();

    for _ in 0..ROUNDS {
        let mut scores = [0; POOL_SIZE];

        for (a, template_warr_a) in pool.iter().enumerate() {
            for (b, template_warr_b) in pool.iter().enumerate() {
                let warr_a = template_warr_a.to_owned();
                let warr_b = template_warr_b.to_owned();

                if warr_a != warr_b {
                    let op_winner = decide_winner(warr_a.to_owned(), warr_b.to_owned(), 1000);
                    match op_winner {
                        Some(winner) => {
                            if winner == warr_a {
                                scores[a] += 1
                            } else {
                                scores[b] += 1
                            }
                        }
                        None => (),
                    }
                }
            }
        }

        let mut scored_pool: Vec<(i32, Warrior)> = scores
            .iter()
            .zip(pool.iter())
            .map(|(a, b)| (a.to_owned(), b.to_owned()))
            .collect();

        scored_pool.sort_by_key(|(k, _)| k.to_owned());

        for i in 0..POOL_SIZE / 2 {
            if POOL_SIZE / 2 + i < pool.len() {
                pool[POOL_SIZE / 2 + i] = pool[i].mutate();
            }
        }
    }

    let result = pool[0].clone();

    let result_len = result.body.len();

    let mut core_conf = core::CoreConfig::new(result_len);
    let _ = core_conf.deploy(
        result,
        Some(ModUsize {
            congruence: result_len,
            val: 0,
        }),
    );
    
    core_conf.brawl().print_state(None);

    Ok(())
}

fn decide_winner(
    a: warrior::Warrior,
    b: warrior::Warrior,
    max_t: usize,
) -> Option<warrior::Warrior> {
    let mut core_conf = core::CoreConfig::new(CORE_SIZE);
    let _ = core_conf.deploy(a, None);
    let _ = core_conf.deploy(b, None);

    let mut runtime = core_conf.brawl();

    for _ in 0..max_t {
        runtime.tick();
        if runtime.warriors.len() == 1 {
            return Some(runtime.warriors[0].clone());
        }
    }

    None
}
