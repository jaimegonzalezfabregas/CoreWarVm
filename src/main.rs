mod core;
mod instruction;
mod test;
mod utils;
mod warrior;

use crate::core::CoreConfig;
use std::process::Command;

use warrior::Warrior;

const CORE_SIZE: isize = 40;

const TEST_MODE: bool = true;

fn main() -> Result<(), String> {
    let mut core_conf = core::CoreConfig::new(CORE_SIZE);

    if TEST_MODE {
        let _imp = warrior::Warrior::parse("MOV 0, 1".into(), "Imp".into(), CORE_SIZE)?;
        let _dwarf = warrior::Warrior::parse(
            "  ADD #4, 3        
        MOV 2, @2
        JMP -2, 0
        DAT #0, #0"
                .into(),
            "Dwarf".into(),
            CORE_SIZE,
        )?;

        core_conf.deploy(_dwarf, Some(0))?;
    } else {
        let warrior_a = warrior::Warrior::random_create(14, CORE_SIZE);
        let warrior_b = warrior::Warrior::random_create(14, CORE_SIZE);

        core_conf.deploy(warrior_a, None)?;
        core_conf.deploy(warrior_b, None)?;
    }

    let mut runtime = core_conf.brawl();

    // println!("{:#?}", runtime);

    for _ in 0..20 {
        runtime.tick();
        println!("state:");
        runtime.print_state();
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
        if runtime.done() {
            break;
        }
    }

    Ok(())
}
