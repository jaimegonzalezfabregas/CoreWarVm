mod core;
mod op;
mod utils;
mod warrior;

const CORE_SIZE: isize = 8;

const TEST_MODE: bool = true;

fn main() -> Result<(), String> {
    rand::thread_rng();
    let mut core_conf = core::CoreConfig::new(CORE_SIZE);

    if TEST_MODE {
        let imp = warrior::Warrior::parse("MOV 0, 1".into(), "Imp".into())?;
        let dwarf = warrior::Warrior::parse(
            "ADD #4, 3
        MOV 2, @2
        JMP -2
        DAT #0, #0"
                .into(),
            "Dwarf".into(),
        )?;

        core_conf.deploy(dwarf, Some(0))?;
    } else {
        let warrior_a = warrior::Warrior::random_create(14, CORE_SIZE);
        let warrior_b = warrior::Warrior::random_create(14, CORE_SIZE);

        core_conf.deploy(warrior_a, None)?;
        core_conf.deploy(warrior_b, None)?;
    }

    let mut runtime = core_conf.brawl();

    // println!("{:#?}", runtime);

    for _ in 0..10 {
        runtime.tick();
        println!("state:");
        runtime.print_state();
    }

    println!("{runtime:?}");

    Ok(())
}
