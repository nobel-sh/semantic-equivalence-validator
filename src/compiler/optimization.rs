use super::CompilerKind;

#[derive(Debug, Clone, Copy)]
pub enum Optimization {
    Zero,
    One,
    Two,
    Three,
    S,
    Z,
}

impl Optimization {
    pub fn numeric(self) -> &'static str{
        match self {
            Self::Zero => "0",
            Self::One => "1",
            Self::Two => "2",
            Self::Three => "3",
            Self::S => "s",
            Self::Z => "z",
        }
    }

    pub fn for_compiler(self, compiler: CompilerKind) -> Vec<String> {
        match compiler {
            CompilerKind::Gccrs => vec![format!("-O{}", self.numeric())],
            CompilerKind::Rustc => vec!["-C".to_string(), format!("opt-level={}", self.numeric())],
        }
    }
}
