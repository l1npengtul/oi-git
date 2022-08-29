use crate::code::LineOfCode;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct TotalPoints {
    pub sum: f64,
    pub total: f64,
}

/// `cor` is the correct answer, `sub` is the submitted answer
pub fn score(cor: &[LineOfCode], sub: &[LineOfCode]) -> f32 {
    let mut correct = 0_u32;
    let mut sub = sub.iter().peekable();
    'outer: for ln in cor.iter() {
        if sub.peek().is_none() {
            break;
        }
        'inner: loop {
            let cur = match sub.next() {
                Some(v) => v,
                None => break 'outer,
            };
            if eq(ln, cur) {
                correct += 1;
                break 'inner;
            }
        }
    }
    correct as f32 / cor.len() as f32
}

fn eq(a: &LineOfCode, b: &LineOfCode) -> bool {
    a.code == b.code && a.color == b.color
}
