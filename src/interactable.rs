use crate::collider::PhysicsBundle;
use crate::prelude::*;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum InteractableType {
    Hammer,
    LineOfCode,
    LineOfCodeGlobule,
    Terminal,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Component)]
pub struct Interactable {
    pub itype: InteractableType,
}

impl Interactable {
    pub const HAMMER: Interactable = Interactable {
        itype: InteractableType::Hammer,
    };
    pub const LOC: Interactable = Interactable {
        itype: InteractableType::LineOfCode,
    };
    pub const LOCG: Interactable = Interactable {
        itype: InteractableType::LineOfCodeGlobule,
    };
    pub const TERMINAL: Interactable = Interactable {
        itype: InteractableType::Terminal,
    };

    pub fn itype(&self) -> InteractableType {
        self.itype
    }
}

impl Interactable {
    pub fn from_name(name: &str) -> Self {
        let name = name.strip_prefix("interactable_").unwrap();
        use InteractableType::*;
        let kind = match name {
            "terminal" => Terminal,
            other => panic!("interactible not regognised {other}"),
        };
        Self { itype: kind }
    }
}

#[derive(Component)]
pub struct LineOfCodeGlobule;

#[derive(Bundle)]
pub struct LineOfCodeGlobuleBundle {
    #[bundle]
    transform: TransformBundle,
    #[bundle]
    phyiscs: PhysicsBundle,
    globule: LineOfCodeGlobule,
}
