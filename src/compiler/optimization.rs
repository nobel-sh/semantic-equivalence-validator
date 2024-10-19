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
    pub fn as_str(self) -> &'static str{
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
            CompilerKind::Gccrs => vec![format!("-O{}", self.as_str())],
            CompilerKind::Rustc => vec!["-C".to_string(), format!("opt-level={}", self.as_str())],
        }
    }
}
