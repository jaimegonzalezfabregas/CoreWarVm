#[derive(Clone, Copy, Debug)]
pub enum Decrement {
    None,
    Predecrement,
    Postdecrement,
}
impl Decrement {
    fn get_random() -> Decrement {
        use Decrement::*;
        [None, Predecrement, Postdecrement][rand::random::<usize>() % 3]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Field {
    Direct(isize),
    Inmediate(isize),

    AIndirect(isize, Decrement),
    BIndirect(isize, Decrement),
}
impl Field {
    fn get_random(ptr_range: isize, core_size: isize) -> Field {
        use Field::*;
        [
            Direct(rand::random::<isize>() % ptr_range),
            Inmediate(rand::random::<isize>() % core_size),
            AIndirect(rand::random::<isize>() % ptr_range, Decrement::get_random()),
            BIndirect(rand::random::<isize>() % ptr_range, Decrement::get_random()),
        ][rand::random::<usize>() % 4]
    }

    fn parse(line: String) -> Result<(Self, String), String> {
        println!("parsing field from: \"{line}\"");
        let line = line.trim();

        if line == "" {
            return Ok((Field::Direct(0), "".into()));
        }

        let mut splited = line.split(",");

        let ret: Self;

        if let Some(line) = splited.next() {
            use Decrement::*;

            let line = line.trim();
            if line.starts_with("#") {
                match str::parse(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::Inmediate(i);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("$") {
                match str::parse(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::Direct(i);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("*") {
                match str::parse(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::AIndirect(i, None);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("@") {
                match str::parse(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::BIndirect(i, None);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("{") {
                match str::parse(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::AIndirect(i, Predecrement);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("<") {
                match str::parse(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::BIndirect(i, Predecrement);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with("}") {
                match str::parse(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::AIndirect(i, Postdecrement);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else if line.starts_with(">") {
                match str::parse(line[1..].into()) {
                    Ok(i) => {
                        ret = Self::BIndirect(i, Postdecrement);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            } else {
                match str::parse(line) {
                    Ok(i) => {
                        ret = Self::Direct(i);
                    }
                    Err(_) => return Err("parsing number failed".into()),
                }
            }
        } else {
            return Err("cant parse a field from empty string".into());
        }

        Ok((ret, splited.collect::<Vec<&str>>().join(" ")))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Operation {
    pub code: OpCode,
    pub modifier: OpModifier,
    pub a: Field,
    pub b: Field,
}

impl Operation {
    pub fn get_random(ptr_range: isize, core_size: isize) -> Self {
        Self {
            code: OpCode::get_random(),
            modifier: OpModifier::get_random(),
            a: Field::get_random(ptr_range, core_size),
            b: Field::get_random(ptr_range, core_size),
        }
    }

    pub(crate) fn parse(line: String) -> Result<Self, String> {
        let mut line = line.trim_start().to_string();

        let (code, line) = OpCode::parse(line.into())?;

        let line = line.trim_start().to_string();

        let (modifier, line) = OpModifier::parse(line.into())?;

        let line = line.trim_start().to_string();

        let (a, line) = Field::parse(line.into())?;

        let line = line.trim_start().to_string();

        let (b, line) = Field::parse(line.into())?;

        Ok(Self {
            code,
            modifier,
            a,
            b,
        })
    }
}

/*

    MOV.A — moves the A-field of the source into the A-field of the destination
    MOV.B — moves the B-field of the source into the B-field of the destination
    MOV.AB — moves the A-field of the source into the B-field of the destination
    MOV.BA — moves the B-field of the source into the A-field of the destination
    MOV.F — moves both fields of the source into the same fields in the destination
    MOV.X — moves both fields of the source into the opposite fields in the destination
    MOV.I — moves the whole source instruction into the destination


DAT, NOP
    Always .F, but it's ignored.
MOV, SEQ, SNE, CMP
    If A-mode is immediate, .AB,
    if B-mode is immediate and A-mode isn't, .B,
    if neither mode is immediate, .I.
ADD, SUB, MUL, DIV, MOD
    If A-mode is immediate, .AB,
    if B-mode is immediate and A-mode isn't, .B,
    if neither mode is immediate, .F.
SLT, LDP, STP
    If A-mode is immediate, .AB,
    if it isn't, (always!) .B.
JMP, JMZ, JMN, DJN, SPL
    Always .B (but it's ignored for JMP and SPL).

*/

#[derive(Clone, Copy, Debug)]
pub enum OpModifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}
impl OpModifier {
    fn get_random() -> OpModifier {
        use OpModifier::*;
        [A, B, AB, BA, F, X, I][rand::random::<usize>() % 7]
    }

    pub fn default() -> OpModifier {
        return Self::F;
    }

    fn parse(line: String) -> Result<(Self, String), String> {
        if line.starts_with(".AB") {
            Ok((Self::AB, line[3..].into()))
        } else if line.starts_with(".BA") {
            Ok((Self::BA, line[3..].into()))
        } else if line.starts_with(".A") {
            Ok((Self::A, line[2..].into()))
        } else if line.starts_with(".B") {
            Ok((Self::B, line[2..].into()))
        } else if line.starts_with(".F") {
            Ok((Self::F, line[2..].into()))
        } else if line.starts_with(".X") {
            Ok((Self::X, line[2..].into()))
        } else if line.starts_with(".I") {
            Ok((Self::I, line[2..].into()))
        } else if line.starts_with(".") {
            Err("stray dot, specting op modifier after it".into())
        } else {
            Ok((Self::default(), line))
        }
    }
}

/*

DAT
    Originally, as its name shows, DAT was intended for storing data, just like in most languages. Since in Core War you want to minimise the number of instructions, storing pointers etc. in unused parts of other instructions is common. This means that the most important thing about DAT is that executing it kills a process. In fact, since the '94 standard has no illegal instructions, DAT is defined as a completely legal instruction, which removes the currently executing process from the process queue. Sounds like splitting hairs, maybe, but precisely defining the obvious can often save a lot of confusion.

    The modifiers have no effect on DAT, and in fact some MARSes remove them. However, remember that predecrementing and postincrementing are always done even if the value isn't used for anything. One unusual thing about DAT, a relic of the previous standards, is that if it has only one argument it's placed in the B-field.
MOV
    MOV copies data from one instruction to another. If you don't know everything about that already, you should probably re-read the earlier chapters. MOV is one of the few instructions that support .I, and that's its default behavior if no modifier is given (and if neither of the fields uses immediate addressing).
ADD
    ADD adds the source value(s) to the destination. The modifiers work like with MOV, except that .I isn't supported but behaves like .F. (What would MOV.AB+DJN.F be?) Also remember that all math in Core War is done modulo coresize.
SUB
    This instruction works exactly like ADD, except for one fairly obvious difference. In fact, all the "arithmetic-logical" instructions work pretty much the same...
MUL
    ...as is the case for MUL too. If you can't guess what it does, you've probably missed something very important.
DIV
    DIV too works pretty much the same as MUL and the others, but there are a few things to keep in mind. First of all, this is unsigned division, which can give surprising results sometimes. Division by zero kills the process, just like executing a DAT, and leaves the destination unchanged. If you use DIV.F or .X to divide two numbers at a time and one of the divisors is 0, the other division will still be done as normal.
MOD
    Everything I said about DIV applies here too, including the division by zero part. Remember that the result of a calculation like MOD.AB #10, #-1 depends on the size of the core. For the common 8000-instruction core the result would be 9 (7999 mod 10).
JMP
    JMP moves execution to the address its A-field points to. The obvious but important difference to the "math" instructions is that JMP only cares about the address, not the data that address points to. Another significant difference is that JMP doesn't use its B-field for anything (and so also ignores its modifier). Being able to jump (or split) into two addresses would simply be too powerful, and it'd make implementing the next three instructions quite difficult. Remember that you can still place an increment or a decrement in the unused B-field, with luck damaging your opponent's code.
JMZ
    This instruction works like JMP, but instead of ignoring its B-field, it tests the value(s) it points to and only jumps if it's zero. Otherwise the execution will continue at the next address. Since there's only one instruction to test, the choice of modifiers is fairly limited. .AB means the same as .B, .BA the same as .A, and .X and .I the same as .F. If you test both fields of an instruction with JMZ.F, it will jump only if both fields are zero.
JMN
    JMN works like JMZ, but jumps if the value tested is not zero (surprise, surprise...). JMN.F jumps if either of the fields is non-zero.
DJN
    DJN is like JMN, but the value(s) are decremented by one before testing. This instruction is useful for making a loop counter, but it can also be used to damage your opponent.
SPL
    This is the big one. The addition of SPL into the language was probably the most significant change ever made to Redcode, only rivalled perhaps by the introduction of the ICWS '94 standard. SPL works like JMP but the execution also continues at the next instruction, so that the process is "split" into two new ones. The process at the next instruction executes before the one which jumped to a new address, which is a small but very important detail. (Many, if not most, modern warriors wouldn't work without it!) If the max. number of processes has been reached, SPL works like NOP. Like JMP, SPL ignores its B-field and its modifier.
SEQ
    SEQ compares two instructions, and skips the next instruction if they are equal. (It always jumps only those two instructions forward, since there's no room for a jump address.) Since the instructions are compared only for equality, using the .I modifier is supported. Quite naturally, with the modifiers .F, .X and .I the next instruction will be skipped only if all the fields are equal.
SNE
    Ok, you guessed it. This instruction skips the next instruction if the instructions it compares are not equal. If you compare more than one field, the next instruction will be skipped if any pair of them aren't equal. (Sounds familiar, doesn't it? just like with JMZ and JMN...)
CMP
    CMP is an alias for SEQ. This was the only name of the instruction before SEQ and SNE were introduced. Nowadays it doesn't really matter which name you use, since the most popular MARS programs recognise SEQ even in '88 mode.
SLT
    Like the previous instructions, SLT skips the next instruction, this time if the first value is lower than the second. Since this is an arithmetical comparison instead of a logical one, it makes no sense to use .I. It might seem that there should be an instruction called SGT, (skip if greater than) but in most cases the same effect can be achieved simply by swapping the operands of SLT. Remember that all values are considered unsigned, so 0 is the smallest possible number and -1 is the largest.
NOP
    Well, this instruction does nothing. (And it does it really well, too.) It's almost never used in an actual warrior, but it's very useful in debugging. Remember that any in- or decrements are still evaluated.

*/

#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    DAT, // — data
    MOV, // — move (copies data from address1 to address2)
    ADD, // — add (adds one number to another)
    SUB, // — subtract (subtracts one number from another)
    MUL, // — multiply (multiplies one number with another)
    DIV, // — divide (divides one number with another)
    MOD, // — modulus (divides one number with another and gives the remainder)
    JMP, // — jump (continues execution from another address)
    JMZ, // — jump if zero (tests a number and jumps to an address if it's 0)
    JMN, // — jump if not zero (tests a number and jumps if it isn't 0)
    DJN, // — decrement and jump if not zero (decrements a number by one, and jumps unless the result is 0)
    SPL, // — split (starts a second process at another address)
    CMP, // — compare (same as SEQ)
    SEQ, // — skip if equal (compares two instructions, and skips the next instruction if they are equal)
    SNE, // — skip if not equal (compares two instructions, and skips the next instruction if they aren't equal)
    SLT, // — skip if lower than (compares two values, and skips the next instruction if the first is lower than the second)
    LDP, // — load from p-space (loads a number from private storage space)
    STP, // — save to p-space (saves a number to private storage space)
    NOP, // — no operation (does nothing)
}
impl OpCode {
    fn get_random() -> OpCode {
        use OpCode::*;
        [
            DAT, MOV, ADD, SUB, DAT, MOV, ADD, SUB, MUL, DIV, MOD, JMP, JMZ, JMN, DJN, SPL, CMP,
            SEQ, SNE, SLT, LDP, STP, NOP,
        ][rand::random::<usize>() % 23]
    }

    fn parse(line: String) -> Result<(Self, String), String> {
        let code = if line.starts_with("DAT") {
            OpCode::DAT
        } else if line.starts_with("MOV") {
            OpCode::MOV
        } else if line.starts_with("ADD") {
            OpCode::ADD
        } else if line.starts_with("SUB") {
            OpCode::SUB
        } else if line.starts_with("MUL") {
            OpCode::MUL
        } else if line.starts_with("DIV") {
            OpCode::DIV
        } else if line.starts_with("MOD") {
            OpCode::MOD
        } else if line.starts_with("JMP") {
            OpCode::JMP
        } else if line.starts_with("JMZ") {
            OpCode::JMZ
        } else if line.starts_with("JMN") {
            OpCode::JMN
        } else if line.starts_with("DJN") {
            OpCode::DJN
        } else if line.starts_with("SPL") {
            OpCode::SPL
        } else if line.starts_with("CMP") {
            OpCode::CMP
        } else if line.starts_with("SEQ") {
            OpCode::SEQ
        } else if line.starts_with("SNE") {
            OpCode::SNE
        } else if line.starts_with("SLT") {
            OpCode::SLT
        } else if line.starts_with("LDP") {
            OpCode::LDP
        } else if line.starts_with("STP") {
            OpCode::STP
        } else if line.starts_with("NOP") {
            OpCode::NOP
        } else {
            return Err("Unrecognizeable opcode".into());
        };

        Ok((code, line[3..].into()))
    }
}
