use glam::{DVec2, Vec2};

use crate::glam_alias::*;

pub trait ConvertToGlam {
    type GlamType;
    fn to_glam(&self) -> Self::GlamType;
}

// f32

impl ConvertToGlam for geo::Coord<f32> {
    type GlamType = GlamPoint;
    fn to_glam(&self) -> Self::GlamType {
        Vec2::new(self.x, self.y)
    }
}

impl ConvertToGlam for geo::Line<f32> {
    type GlamType = GlamLine;
    fn to_glam(&self) -> Self::GlamType {
        (self.start.to_glam(), self.end.to_glam())
    }
}

impl ConvertToGlam for geo::LineString<f32> {
    type GlamType = GlamLineString;
    fn to_glam(&self) -> Self::GlamType {
        self.0
            .iter()
            .map(ConvertToGlam::to_glam)
            .collect::<Vec<_>>()
    }
}

impl ConvertToGlam for geo::Polygon<f32> {
    type GlamType = GlamPolygon;
    fn to_glam(&self) -> Self::GlamType {
        (
            self.exterior().to_glam(),
            self.interiors()
                .iter()
                .map(ConvertToGlam::to_glam)
                .collect::<Vec<_>>(),
        )
    }
}

// f64

impl ConvertToGlam for geo::Coord<f64> {
    type GlamType = DGlamPoint;
    fn to_glam(&self) -> Self::GlamType {
        DVec2::new(self.x, self.y)
    }
}

impl ConvertToGlam for geo::Line<f64> {
    type GlamType = DGlamLine;
    fn to_glam(&self) -> Self::GlamType {
        (self.start.to_glam(), self.end.to_glam())
    }
}

impl ConvertToGlam for geo::LineString<f64> {
    type GlamType = DGlamLineString;
    fn to_glam(&self) -> Self::GlamType {
        self.0
            .iter()
            .map(ConvertToGlam::to_glam)
            .collect::<Vec<_>>()
    }
}

impl ConvertToGlam for geo::Polygon<f64> {
    type GlamType = DGlamPolygon;
    fn to_glam(&self) -> Self::GlamType {
        (
            self.exterior().to_glam(),
            self.interiors()
                .iter()
                .map(ConvertToGlam::to_glam)
                .collect::<Vec<_>>(),
        )
    }
}
