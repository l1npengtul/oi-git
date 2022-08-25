use crate::collider::PhysicsBundle;
use crate::prelude::*;
use bevy_rapier3d::prelude::InteractionGroups;

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
