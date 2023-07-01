use glam::{DVec2, Vec2};

pub type GlamPoint = Vec2;
pub type GlamLine = (Vec2, Vec2);
pub type GlamLineString = Vec<Vec2>;
pub type GlamPolygon = (GlamLineString, Vec<GlamLineString>);

pub type DGlamPoint = DVec2;
pub type DGlamLine = (DVec2, DVec2);
pub type DGlamLineString = Vec<DVec2>;
pub type DGlamPolygon = (DGlamLineString, Vec<DGlamLineString>);
