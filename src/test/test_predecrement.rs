#[cfg(test)]

mod tests {
    use crate::{
        core::CoreConfig,
        instruction::instruction::Instruction,
        test::tests::{parse_ares_dump, ReadOnlyInstruction},
        warrior::Warrior,
    };

    #[test]
    fn test_imp() {
        let mut core_conf = CoreConfig::new(8000);

        let imp = match Warrior::parse(
            "mov 1, <1
dat 0, 7"
                .into(),
            "Imp".into(),
            8000,
        ) {
            Ok(res) => res,
            Err(err) => panic!("el parsing del warrior a fallado: {}", err),
        };

        core_conf.deploy(imp, Some(0)).unwrap();

        let mut runtime = core_conf.brawl();

        runtime.tick();

        let sim_result = runtime.core;
        let res = parse_ares_dump("src/test/checks/test_predecrement_check.red");

        println!("checking len");
        assert_eq!(sim_result.len(), res.len());

        for i in 0..8000 {
            // println!("checking pos {i} \n{:?}\n{:?}", sim_result[i], res[i]);
            println!("checking pos {i}");
            assert_eq!(
                <Instruction as Into<ReadOnlyInstruction>>::into(sim_result[i]),
                res[i]
            );
        }
    }
}
