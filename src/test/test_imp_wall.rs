#[cfg(test)]

mod tests {
    use crate::{
        core::CoreConfig, test::tests::compare_runtime_with_file, utils::ModUsize, warrior::Warrior,
    };

    #[test]
    fn test_imp_wall() {
        const CORE_SIZE: usize = 8000;

        let mut core_conf = CoreConfig::new(CORE_SIZE);

        let imp_wall = match Warrior::parse("jmp 0, <-1".into(), "Imp Wall".into(), CORE_SIZE) {
            Ok(res) => res,
            Err(err) => panic!("el parsing del warrior a fallado: {}", err),
        };

        core_conf
            .deploy(imp_wall, Some(ModUsize::new(0, CORE_SIZE)))
            .unwrap();

        let mut runtime = core_conf.brawl();

        for _ in 0..9 {
            runtime.tick();
        }

        compare_runtime_with_file("src/test/checks/test_impwall_9_check.red", &runtime, "");
    }
}
