## reference

https://vyznev.net/corewar/guide.html

## llvm coverage cheatsheet 
https://doc.rust-lang.org/rustc/instrument-coverage.html    

set RUSTFLAGS= -C instrument-coverage  
set LLVM_PROFILE_FILE=test/results/formatjson5.profraw

cargo test

cargo profdata -- merge -sparse test/results/formatjson5.profraw -o test/merged/formatjson5.profdata

cargo cov -- show -Xdemangler=rustfilt target\debug\deps\core_war_vm-d479dd518e1bd670.exe -instr-profile=test/merged/formatjson5.profdata -show-line-counts-or-regions -show-instantiations -name=test_dwarf

## tarpaulin
https://github.com/xd009642/tarpaulin

cargo tarpaulin --out Html