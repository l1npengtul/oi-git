use crate::prelude::*;

pub struct CodePlugin;

impl Plugin for CodePlugin {
    fn build(&self, _: &mut App) {}
}

#[derive(Component, Debug, Clone)]
pub struct LineOfCode {
    pub diff: Diff,
    pub color: CodeColor,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Diff {
    Pos,
    Neg,
    Eq,
    Rem,
}

impl Diff {
    pub fn prefix(&self) -> &'static str {
        match self {
            Diff::Pos => "++",
            Diff::Neg => "--",
            Diff::Eq => "==",
            Diff::Rem => "!!",
        }
    }

    pub fn from_line(s: &str) -> Self {
        macro_rules! match_starts_with_prefix {
            ($s:expr => { $($variant:expr),* $(,)? }) => {
                match $s {
                    $(s if s.starts_with($variant.prefix()) => $variant,)*
                    s => panic!("prefix not found in line: {s}")
                }
            };
        }
        match_starts_with_prefix!(s => {
            Diff::Pos,
            Diff::Neg,
            Diff::Eq,
            Diff::Rem,
        })
    }

    pub fn to_color(&self) -> CodeColor {
        use CodeColor::*;
        use Diff::*;
        match self {
            Rem => None,
            Neg => Red,
            Pos => Green,
            Eq => Normal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeColor {
    Green,
    Red,
    Normal,
    None,
}
