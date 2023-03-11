use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::RenderApp;
use bevy::ui::{FocusPolicy, RenderUiSystem};

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

#[derive(Component, Clone, Debug, Default)]
pub struct UiNineSlice(pub Handle<NineSlice>);

#[derive(Component, Clone, Debug, Default)]
pub enum NineSliceMode {
    #[default]
    Stretch,
    Repeat,
    Mirror,
}

/// A UI node that is a nine slice
#[derive(Bundle, Clone, Debug, Default)]
pub struct NineSliceBundle {
    /// Describes the size of the node
    pub node: Node,
    /// Describes the style including flexbox settings
    pub style: Style,
    /// Configures how the nine slice should scale
    pub image_mode: NineSliceMode,
    /// The calculated size based on the given image
    pub calculated_size: CalculatedSize,
    /// The image of the node
    pub nine_slice: UiNineSlice,
    /// Whether this node should block interaction with lower nodes
    pub focus_policy: FocusPolicy,
    /// The transform of the node
    ///
    /// This field is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub transform: Transform,
    /// The global transform of the node
    ///
    /// This field is automatically managed by the UI layout system.
    /// To alter the position of the `NodeBundle`, use the properties of the [`Style`] component.
    pub global_transform: GlobalTransform,
    /// Describes the visibility properties of the node
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
    /// Indicates the depth at which the node should appear in the UI
    pub z_index: ZIndex,
}

impl Plugin for NineSlicePlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<NineSlice>();
        app.add_asset_loader(NineSliceLoader);

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.add_system(
            render::extract_uinodes
                .after(RenderUiSystem::ExtractNode)
                .in_schedule(ExtractSchedule),
        );
    }
}
