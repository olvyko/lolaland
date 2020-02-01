use amethyst::{
    core::math::Vector2,
    ecs::{Component, DenseVecStorage},
};

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Rect {
    pub half_extents: Vector2<f32>,
}

impl Rect {
    pub fn from_size(width: f32, height: f32) -> Self {
        Self {
            half_extents: Vector2::new(width / 2.0, height / 2.0),
        }
    }
}

// pub fn is_overlaps(rect1: &Rect, pos1: &Position, rect2: &Rect, pos2: &Position) -> bool {
//     if (pos1.x - pos2.x).abs() > rect1.half_extents.x + rect2.half_extents.x {
//         return false;
//     }
//     if (pos1.y - pos2.y).abs() > rect1.half_extents.y + rect2.half_extents.y {
//         return false;
//     }
//     return true;
// }
