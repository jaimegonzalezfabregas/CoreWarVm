#[cfg(test)]

mod tests {
    use core::panic;

    use crate::{core::CoreConfig, test::parse_ares_dump, warrior::Warrior};

    #[test]
    fn test_dwarf() {
        let mut core_conf = CoreConfig::new(8000);

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

        core_conf.deploy(dwarf, Some(0)).unwrap();

        let mut runtime = core_conf.brawl();

        for _ in 0..((8000 + 3480) / 4 * 3) {
            runtime.tick();
        }

        runtime.print_state();

        let sim_result = runtime.core;

        let res = parse_ares_dump("test_dwarf_8610_check.red");

        println!("checking len");
        assert_eq!(sim_result.len(), res.len());

        for i in 0..8000 {
            // println!("checking pos {i} \n{:?}\n{:?}", sim_result[i], res[i]);
            println!("checking pos {i}");
            assert_eq!(sim_result[i], res[i]);
        }

        println!("checking full");
        assert_eq!(sim_result, res);
    }
}