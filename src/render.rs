use bevy::render::Extract;
use bevy::ui::*;
use bevy::{math::vec2, prelude::*};

use crate::{NineSlice, NineSliceMode, UiNineSlice};

pub fn extract_uinodes(
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    images: Extract<Res<Assets<Image>>>,
    nine_slices: Extract<Res<Assets<NineSlice>>>,
    ui_stack: Extract<Res<UiStack>>,
    ui_scale: Extract<Res<UiScale>>,
    uinode_query: Extract<
        Query<(
            &Node,
            &GlobalTransform,
            &UiNineSlice,
            &NineSliceMode,
            &ComputedVisibility,
            Option<&CalculatedClip>,
        )>,
    >,
) {
    for (stack_index, entity) in ui_stack.uinodes.iter().enumerate() {
        if let Ok((uinode, global_transform, nine_slice, mode, visibility, clip)) =
            uinode_query.get(*entity)
        {
            if !visibility.is_visible() {
                continue;
            }

            // Skip loading nineslices
            let Some(nine_slice) = nine_slices.get(&nine_slice.0) else {
                continue;
            };

            // Skip loading images
            let Some(image) = images.get(&nine_slice.image) else {
                continue;
            };

            let margins = nine_slice.margins;
            let is = image.size();
            let ix = [0.0, margins.min.x, is.x - margins.max.x, is.x];
            let iy = [0.0, margins.min.y, is.y - margins.max.y, is.y];
            let bs = uinode.size() / ui_scale.scale as f32;
            let bx = [0.0, margins.min.x, bs.x - margins.max.x, bs.x];
            let by = [0.0, margins.min.y, bs.y - margins.max.y, bs.y];

            let mut draw = |uv: Rect, rect: Rect| {
                let scale = (rect.size() / uv.size()) * ui_scale.scale as f32;

                extracted_uinodes.uinodes.push(ExtractedUiNode {
                    stack_index,
                    transform: global_transform.compute_matrix()
                        * Mat4::from_scale(scale.extend(1.0))
                        * Mat4::from_translation((bs * -0.5 + rect.center()).extend(0.0)),
                    color: Color::WHITE,
                    rect: uv,
                    image: nine_slice.image.clone_weak(),
                    atlas_size: Some(image.size()),
                    clip: clip.map(|clip| clip.clip),
                    flip_x: false,
                    flip_y: false,
                });
            };

            for y in 0..3 {
                for x in 0..3 {
                    if x == 1 || y == 1 {
                        match mode {
                            NineSliceMode::Stretch => (),
                            NineSliceMode::Repeat => {
                                todo!()
                            }
                            NineSliceMode::Mirror => {
                                todo!()
                            }
                        }
                    }

                    draw(
                        Rect {
                            min: vec2(ix[x], iy[y]),
                            max: vec2(ix[x + 1], iy[y + 1]),
                        },
                        Rect {
                            min: vec2(bx[x], by[y]),
                            max: vec2(bx[x + 1], by[y + 1]),
                        },
                    );
                }
            }
        }
    }
}
