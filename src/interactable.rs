use crate::prelude::*;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum InteractableType {
    Hammer,
    LineOfCode,
    LineOfCodeGlobule,
    Terminal,
}

#[derive(Component)]
pub struct Interactable {
    pub itype: InteractableType,
}

impl Interactable {
    pub fn itype(&self) -> InteractableType {
        self.itype
    }
}
