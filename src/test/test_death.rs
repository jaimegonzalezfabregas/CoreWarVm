#[cfg(test)]

mod tests {
    use crate::{core::CoreConfig, utils::ModUsize, warrior::Warrior};

    #[test]
    fn test_death() {
        const CORE_SIZE: usize = 8000;
        let mut core_conf = CoreConfig::new(CORE_SIZE);

        let code = match Warrior::parse(
            "spl 3, <5
JMN -1, 4
add.a #1, 1
jmp 0
dat 0
dat 10"
                .into(),
            "code".into(),
            CORE_SIZE,
        ) {
            Ok(res) => res,
            Err(err) => panic!("el parsing del warrior a fallado: {}", err),
        };

        core_conf
            .deploy(code, Some(ModUsize::new(0, CORE_SIZE)))
            .unwrap();

        let mut runtime = core_conf.brawl();

        for _ in 0..10000 {
            runtime.tick();
        }

        if !runtime.done() {
            panic!()
        }
    }
}
