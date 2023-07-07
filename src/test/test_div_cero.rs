#[cfg(test)]

mod tests {
    use crate::{
        core::CoreConfig, test::tests::compare_runtime_with_file, utils::ModUsize, warrior::Warrior,
    };

    #[test]
    fn test_div_cero() {
        const WARRIOR: &str = "div #0, #0
        nop
        nop
        jmp -1";

        const CORE_SIZE: usize = 8000;
        let mut core_conf = CoreConfig::new(CORE_SIZE);

        let code = match Warrior::parse(WARRIOR.into(), "code".into(), CORE_SIZE) {
            Ok(res) => res,
            Err(err) => panic!("el parsing del warrior a fallado: {}", err),
        };

        core_conf
            .deploy(code, Some(ModUsize::new(0, CORE_SIZE)))
            .unwrap();

        let mut runtime = core_conf.brawl();

        runtime.tick();

        if !runtime.done() {
            panic!("didn't die at division");
        }
    }
}
