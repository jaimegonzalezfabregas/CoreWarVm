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

use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpModifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
    Default,
}
impl OpModifier {
    pub fn get_random() -> OpModifier {
        use OpModifier::*;
        [A, B, AB, BA, F, X, I]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    }

    pub fn parse(line: String) -> Result<(Self, String), String> {
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
            Ok((Self::Default, line))
        }
    }

    pub fn print(&self) {
        print!(
            "{}",
            match self {
                OpModifier::A => ".A",
                OpModifier::B => ".B",
                OpModifier::AB => ".AB",
                OpModifier::BA => ".BA",
                OpModifier::F => ".F",
                OpModifier::X => ".X",
                OpModifier::I => ".I",
                OpModifier::Default => "",
            },
        );
    }
}
