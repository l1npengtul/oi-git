use crate::prelude::*;
use super::{OfficeAssets, OfficeAssetBuilder, OfficeAssetKind};

pub fn spawn_office(assets: Res<OfficeAssets>) {
    for (name, _) in assets.assets.iter() {
        info!("{name}");
    }
    panic!();
}