mod core;
mod op;
mod warrior;

const CORE_SIZE: usize = 8000;

fn main() -> Result<(), (usize, String)> {
    rand::thread_rng();

    let core = core::Core::new(CORE_SIZE);
    // let warrior_a = warrior::Warrior::create(14, CORE_SIZE);
    // let warrior_b = warrior::Warrior::create(14, CORE_SIZE);

    // core.deploy(warrior_a);
    // core.deploy(warrior_b);

    let imp = warrior::Warrior::parse(
        "ADD #4, 3
        MOV 2, @2
        JMP -2
        DAT #0, #0"
            .into(),
    )?;

    println!("{imp:?}");

    Ok(())
}
