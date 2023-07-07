#[cfg(test)]

mod tests {
    use core::panic;

    use crate::{core::CoreConfig, utils::ModUsize, warrior::Warrior};

    #[test]
    fn test_warrior_colision() {
        const CORE_SIZE: usize = 8000;
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

        let mut core_conf = CoreConfig::new(CORE_SIZE);
        core_conf
            .deploy(dwarf.clone(), Some(ModUsize::new(0, CORE_SIZE)))
            .unwrap();
        match core_conf.deploy(dwarf.clone(), Some(ModUsize::new(1, CORE_SIZE))) {
            Ok(_) => panic!("la colision no ha sido detectada 1"),
            Err(_) => (),
        }

        let mut core_conf = CoreConfig::new(CORE_SIZE);
        core_conf
            .deploy(dwarf.clone(), Some(ModUsize::new(7998, CORE_SIZE)))
            .unwrap();
        match core_conf.deploy(dwarf.clone(), Some(ModUsize::new(0, CORE_SIZE))) {
            Ok(_) => panic!("la colision no ha sido detectada 2"),
            Err(_) => (),
        }

        let mut core_conf = CoreConfig::new(CORE_SIZE);
        core_conf
            .deploy(dwarf.clone(), Some(ModUsize::new(3001, CORE_SIZE)))
            .unwrap();
        match core_conf.deploy(dwarf.clone(), Some(ModUsize::new(3000, CORE_SIZE))) {
            Ok(_) => panic!("la colision no ha sido detectada 3"),
            Err(_) => (),
        }

        let mut core_conf = CoreConfig::new(CORE_SIZE);

        let mut filled_core = false;

        for _ in 0..CORE_SIZE {
            match core_conf.deploy(dwarf.clone(), None) {
                Ok(_) => (),
                Err(_) => {
                    filled_core = true;
                    break;
                }
            }
        }

        if !filled_core {
            panic!("filled core was undetected");
        }
    }
}
