#[cfg(test)]

mod tests {
    use core::panic;

    use crate::{
        core::CoreConfig, test::tests::compare_runtime_with_file, utils::ModUsize, warrior::Warrior,
    };

    #[test]
    fn test_chang_vs_mice() {
        const CORE_SIZE: usize = 8000;
        let mut core_conf = CoreConfig::new(CORE_SIZE);

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
            CORE_SIZE,
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
            CORE_SIZE,
        ) {
            Ok(res) => res,
            Err(err) => panic!("el parsing de mice ha fallado: {}", err),
        };
        core_conf
            .deploy(chang, Some(ModUsize::new(0, CORE_SIZE)))
            .unwrap();

        core_conf
            .deploy(mice, Some(ModUsize::new(4000, CORE_SIZE)))
            .unwrap();

        let mut runtime = core_conf.brawl();

        for i in 0..=50 {
            println!("check");
            compare_runtime_with_file(
                &format!("src/test/checks/test_chang_vs_mice_{i}_check.red"),
                &runtime,
                &format!("tick {i}"),
            );

            println!("step");

            runtime.tick();
            runtime.tick();
            if i == 20 {
                runtime.tick();
                runtime.tick();
                runtime.tick();
                runtime.tick();
            }
        }
    }
}
