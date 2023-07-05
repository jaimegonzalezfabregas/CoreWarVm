#[cfg(test)]

mod tests {

    use crate::{core::CoreConfig, warrior::Warrior};

    #[test]
    fn test_normal_run() {

        let mut core_conf = CoreConfig::new(8000);

        let warrior1 = Warrior::random_create(20, 8000);
        let warrior2 = Warrior::random_create(20, 8000);

        core_conf.deploy(warrior1, None).unwrap();
        core_conf.deploy(warrior2, None).unwrap();

        let mut runtime = core_conf.brawl();

        for _ in 0..10000 {
            runtime.tick();
        }
    }
}
