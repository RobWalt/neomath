use neo_aabb::d2::def::AABB2D;
use neo_aabb::d3::def::AABB3D;

pub trait NeoBounded2D {
    fn aabb(&self) -> AABB2D {
        AABB2D::from(([self.min_x(), self.min_y()], [self.max_x(), self.max_y()]))
    }

    fn min_x(&self) -> f32;
    fn min_y(&self) -> f32;
    fn max_x(&self) -> f32;
    fn max_y(&self) -> f32;
}

pub trait NeoBounded3D {
    fn aabb(&self) -> AABB3D {
        AABB3D::from((
            [self.min_x(), self.min_y(), self.min_z()],
            [self.max_x(), self.max_y(), self.max_z()],
        ))
    }

    fn min_x(&self) -> f32;
    fn min_y(&self) -> f32;
    fn min_z(&self) -> f32;
    fn max_x(&self) -> f32;
    fn max_y(&self) -> f32;
    fn max_z(&self) -> f32;
}
