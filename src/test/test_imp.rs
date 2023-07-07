#[cfg(test)]

mod tests {
    use crate::{
        core::CoreConfig, instruction::instruction::Instruction,
        test::tests::{parse_ares_dump, ReadOnlyInstruction}, warrior::Warrior, utils::ModUsize,
    };

    #[test]
    fn test_imp() {
        let core_size = 8000;
        let mut core_conf = CoreConfig::new(core_size);

        let imp = match Warrior::parse("mov 0, 1".into(), "Imp".into(), core_size) {
            Ok(res) => res,
            Err(err) => panic!("el parsing del warrior a fallado: {}", err),
        };

        core_conf.deploy(imp, Some(ModUsize::new(0,core_size))).unwrap();

        let mut runtime = core_conf.brawl();

        for _ in 0..9 {
            runtime.tick();
        }

        let sim_result = runtime.core;
        let res = parse_ares_dump("src/test/checks/test_imp_9_check.red");

        println!("checking len");
        assert_eq!(sim_result.len(), res.len());

        for i in 0..core_size {
            // println!("checking pos {i} \n{:?}\n{:?}", sim_result[i], res[i]);
            println!("checking pos {i}");
            assert_eq!(
                <Instruction as Into<ReadOnlyInstruction>>::into(sim_result[i]),
                res[i]
            );
        }
    }
}