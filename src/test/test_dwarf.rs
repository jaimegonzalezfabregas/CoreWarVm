#[cfg(test)]

mod tests {
    use core::panic;

    use crate::{
        core::CoreConfig, test::tests::compare_runtime_with_file, utils::ModUsize, warrior::Warrior,
    };

    #[test]
    fn test_dwarf() {
        const CORE_SIZE: usize = 8000;
        let mut core_conf = CoreConfig::new(CORE_SIZE);

        let dwarf = match Warrior::parse(
            "  ADD #4, 3        
        MOV 2, @2
        JMP -2, 0
        DAT #0, #0"
                .into(),
            "Dwarf".into(),
            CORE_SIZE,
        ) {
            Ok(res) => res,
            Err(err) => panic!("el parsing del warrior a fallado: {}", err),
        };

        core_conf
            .deploy(dwarf, Some(ModUsize::new(0, CORE_SIZE)))
            .unwrap();

        let mut runtime = core_conf.brawl();

        for _ in 0..((8000 + 3480) / 4 * 3) {
            runtime.tick();
        }

        compare_runtime_with_file("src/test/checks/test_dwarf_8610_check.red", &runtime, "");
    }
}
