#[cfg(test)]

mod tests {
    use crate::{
        core::CoreConfig, test::tests::compare_runtime_with_file, utils::ModUsize, warrior::Warrior,
    };

    #[test]
    fn test_arithmetic() {
        const WARRIOR: &str = "jmp 2
dat 0,7
add #1, -1
jmp 2
dat 0,-20
add #100, -1
jmp 2
dat 0,-72
add #-100, -1
jmp 2
dat 0,493
add #-562, -1
jmp 2
dat 0,7
mul #1, -1
jmp 2
dat 0,-20
mul #100, -1
jmp 2
dat 0,-72
mul #-100, -1
jmp 2
dat 0,493
mul #-562, -1
jmp 2
dat 0,700
div #14, -1
jmp 2
dat 0,-20
div #3, -1
jmp 2
dat 0,-72
div #8, -1
jmp 2
dat 0,493
div #-2, -1
jmp 2
dat 0,7
sub #7, -1
jmp 2
dat 0,-20
sub #3, -1
jmp 2
dat 0,-72
sub #-7, -1
jmp 2
dat 0,493
sub #-5, -1
jmp 2
dat 0,7
mod #3, -1
jmp 2
dat 0,-20
mod #9, -1
jmp 2
dat 0,-72
mod #-6, -1
jmp 2
dat 0,493
mod #-5, -1
";

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

        while !runtime.done() {
            runtime.tick();
        }

        compare_runtime_with_file("src/test/checks/test_arithmetic_check.red", &runtime, "");
    }
}
