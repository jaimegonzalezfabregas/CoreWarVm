#[cfg(test)]

mod tests {
    use core::panic;

    use crate::{core::CoreConfig, warrior::Warrior};

    #[test]
    fn test_warrior_colision() {
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

        let mut core_conf = CoreConfig::new(8000);
        core_conf.deploy(dwarf.clone(), Some(0)).unwrap();
        match core_conf.deploy(dwarf.clone(), Some(1)) {
            Ok(_) => panic!("la colision no ha sido detectada 1"),
            Err(_) => (),
        }

        let mut core_conf = CoreConfig::new(8000);
        core_conf.deploy(dwarf.clone(), Some(7998)).unwrap();
        match core_conf.deploy(dwarf.clone(), Some(0)) {
            Ok(_) => panic!("la colision no ha sido detectada 2"),
            Err(_) => (),
        }

        let mut core_conf = CoreConfig::new(8000);
        core_conf.deploy(dwarf.clone(), Some(3001)).unwrap();
        match core_conf.deploy(dwarf.clone(), Some(3001)) {
            Ok(_) => panic!("la colision no ha sido detectada 3"),
            Err(_) => (),
        }

        let mut core_conf = CoreConfig::new(8000);

        let mut filled_core = false;

        for _ in 0..8000 {
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
