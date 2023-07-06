#[cfg(test)]

mod tests {
    use core::panic;

    use crate::{
        core::CoreConfig,
        instruction::instruction::Instruction,
        test::tests::{parse_ares_dump, ReadOnlyInstruction},
        warrior::Warrior,
    };

    #[test]
    fn test_chang_vs_mice() {
        let mut core_conf = CoreConfig::new(8000);

        let chang = match Warrior::parse(
            "jmp 4
mov 2, -1
jmp -1
dat 9
spl -2
spl 4
add #-16, -3
mov -4, @-4
jmp -4
spl 2
jmp -1
mov 0, 1
"
            .into(),
            "CHANG1".into(),
            8000,
        ) {
            Ok(res) => res,
            Err(err) => panic!("el parsing de chang ha fallado: {}", err),
        };

        let mice = match Warrior::parse(
            "jmp 2
dat 0
mov #12, -1
mov @-2, <5
djn -1, -3
spl @3
add #653, 2
jmz -5, -6
dat 833
"
            .into(),
            "MICE  ".into(),
            8000,
        ) {
            Ok(res) => res,
            Err(err) => panic!("el parsing de mice ha fallado: {}", err),
        };

        core_conf.deploy(mice, Some(4073)).unwrap();
        core_conf.deploy(chang, Some(0)).unwrap();

        let mut runtime = core_conf.brawl();

        for i in 0..=11 {
            println!("check");
            let res = parse_ares_dump(&format!("src/test/test_chang_vs_mice_{i}_check.red"));

            for cell_i in 0..8000 {
                let a = <Instruction as Into<ReadOnlyInstruction>>::into(runtime.core[cell_i]);
                let b = res[cell_i];
                if a != b {
                    runtime.print_state(Some(cell_i.max(10) - 10..cell_i + 10));

                    panic!("checking pos {cell_i} at tick {i}: \n{a:?} \n!= \n{b:?}\n");
                }
            }
            println!("step");

            runtime.tick();
            runtime.tick();
        }
    }
}
