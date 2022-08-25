use crate::{
    code::{Diff, LineOfCode},
    prelude::*,
};

const LEVELS: &'static str = include_str!("../assets/code/code.txt");
#[cfg(not(windows))]
const LEVEL_SEP: &'static str = "NEXT_LEVEL\n";
#[cfg(windows)]
const LEVEL_SEP: &'static str = "NEXT_LEVEL\r\n";

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Levels>();
    }
}

#[derive(Debug, Clone)]
pub struct Levels {
    pub levels: Vec<CodeBlock>,
    pub code_text: Vec<&'static str>,
    pub current: usize,
}

impl Default for Levels {
    fn default() -> Self {
        Self::load()
    }
}

impl Levels {
    fn load() -> Self {
        let levels = LEVELS.split(LEVEL_SEP).map(CodeBlock::from_str).collect();
        Self {
            levels,
            code_text: LEVELS.split(LEVEL_SEP).collect(),
            current: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub code: Vec<LineOfCode>,
}

impl CodeBlock {
    fn from_str(s: &str) -> Self {
        let mut lines_of_code = Vec::new();
        for ln in s.lines() {
            let diff = Diff::from_line(ln);
            let code = ln.strip_prefix(diff.prefix()).unwrap().trim();

            lines_of_code.push(LineOfCode {
                color: diff.to_color(),
                code: code.to_owned(),
                diff,
            })
        }
        CodeBlock {
            code: lines_of_code,
        }
    }
}
