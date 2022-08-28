use crate::prelude::*;

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Component)]
pub enum ViewModelHold {
    Empty,
    Hammer,
    LoC,
    LoCBundle,
}

#[derive(Component)]
pub struct ViewModel {
    pub holding: ViewModelHold,
}

impl ViewModel {
    pub fn holding(&self) -> ViewModelHold {
        self.holding
    }

    pub fn change_holding(&mut self, new_hold: ViewModelHold) {
        self.holding = new_hold;
    }
}

#[derive(Component)]
pub struct ViewModelCurrentlyHeld;

#[derive(Bundle)]
pub struct ViewModelBundle {
    #[bundle]
    pub transform: TransformBundle,
    pub viewmodel: ViewModel,
}

#[derive(Component)]
pub struct ViewMdlCamera;

// TODO: Polish: Render on top
//
// #[derive(AsBindGroup, TypeUuid, Clone, Default)]
// #[uuid = "7494888b-c082-457b-aacf-517228cc0c23"]
// pub struct ViewModelMaterial {
//     standard_material: StandardMaterial,
// }
//
// impl From<StandardMaterial> for ViewModelMaterial {
//     fn from(sm: StandardMaterial) -> Self {
//         ViewModelMaterial {
//             standard_material: sm,
//         }
//     }
// }
//
// impl From<ViewModelMaterial> for StandardMaterial {
//     fn from(vmm: ViewModelMaterial) -> Self {
//         vmm.standard_material
//     }
// }
//
// impl Material for ViewModelMaterial {}
