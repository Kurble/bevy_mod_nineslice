use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::{RenderApp, RenderStage};
use bevy::ui::RenderUiSystem;

use loader::NineSliceLoader;

pub mod loader;
pub mod render;

pub struct NineSlicePlugin;

#[derive(Debug, TypeUuid)]
#[uuid = "c81fdf3e-1bc0-4ca9-8c65-1808db68e4e5"]
pub struct NineSlice {
    pub image: Handle<Image>,
    pub margins: Rect,
    pub content: Rect,
}

#[derive(Component)]
pub struct UiNineSlice {
    pub nine_slice: Handle<NineSlice>,
    pub mode: NineSliceMode,
}

pub enum NineSliceMode {
    Stretch,
    Repeat,
    Mirror,
}

impl Plugin for NineSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<NineSlice>();
        app.add_asset_loader(NineSliceLoader);

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_system_to_stage(
            RenderStage::Extract,
            render::extract_uinodes.after(RenderUiSystem::ExtractNode),
        );
    }
}
