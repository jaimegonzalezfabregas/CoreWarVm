#[cfg(test)]

mod tests {
    use core::panic;

    use crate::{
        core::CoreConfig,
        instruction::instruction::Instruction,
        test::tests::{parse_ares_dump, ReadOnlyInstruction},
        warrior::Warrior, utils::ModUsize,
    };

    #[test]
    fn test_dwarf() {
        let core_size = 8000;
        let mut core_conf = CoreConfig::new(core_size);

        let dwarf = match Warrior::parse(
            "  ADD #4, 3        
        MOV 2, @2
        JMP -2, 0
        DAT #0, #0"
                .into(),
            "Dwarf".into(),
            8000,
        ) {
            Ok(res) => res,
            Err(err) => panic!("el parsing del warrior a fallado: {}", err),
        };

        core_conf.deploy(dwarf, Some(ModUsize::new(0, core_size))).unwrap();

        let mut runtime = core_conf.brawl();

        for _ in 0..((8000 + 3480) / 4 * 3) {
            runtime.tick();
        }

        runtime.print_state(None);

        let sim_result = runtime.core;

        let res = parse_ares_dump("src/test/test_dwarf_8610_check.red");

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
