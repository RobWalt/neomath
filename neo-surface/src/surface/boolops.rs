use geo::{MapCoords, SpadeBoolops};

use crate::surface::def::NeoSurface;

fn option_difference(
    p1: &geo::Polygon<f32>,
    p2: &geo::Polygon<f32>,
) -> Option<geo::MultiPolygon<f32>> {
    geo::Polygon::difference(p1, p2).ok()
}

fn option_union(p1: &geo::Polygon<f32>, p2: &geo::Polygon<f32>) -> Option<geo::MultiPolygon<f32>> {
    geo::Polygon::union(p1, p2).ok()
}

fn option_intersection(
    p1: &geo::Polygon<f32>,
    p2: &geo::Polygon<f32>,
) -> Option<geo::MultiPolygon<f32>> {
    geo::Polygon::intersection(p1, p2).ok()
}

impl NeoSurface {
    fn lax_boolop(
        &self,
        rhs: &Self,
        boolop: fn(&geo::Polygon<f32>, &geo::Polygon<f32>) -> Option<geo::MultiPolygon<f32>>,
    ) -> Option<Vec<Self>> {
        let diff_2d = self.shape_origin - rhs.shape_origin;
        let rhs_translated = rhs.shape.map_coords(|c| c + diff_2d);
        let union = boolop(&self.shape, &rhs_translated)?;
        let neo_surfaces = union
            .into_iter()
            .map(|p| NeoSurface {
                shape: p,
                ..self.clone()
            })
            .collect::<Vec<_>>();
        Some(neo_surfaces)
    }

    fn strict_boolop(
        &self,
        rhs: &Self,
        boolop: fn(&geo::Polygon<f32>, &geo::Polygon<f32>) -> Option<geo::MultiPolygon<f32>>,
    ) -> Option<Vec<Self>> {
        (self.coordinate_system == rhs.coordinate_system)
            .then(|| self.lax_boolop(rhs, boolop))
            .flatten()
    }

    /// unions the two surfaces and makes sure that they are in the same coordinate system
    pub fn strict_union(&self, rhs: &Self) -> Option<Vec<Self>> {
        self.strict_boolop(rhs, option_union)
    }

    /// subtracts first from second surface and makes sure that they are in the same coordinate
    /// system
    pub fn strict_difference(&self, rhs: &Self) -> Option<Vec<Self>> {
        self.strict_boolop(rhs, option_difference)
    }

    /// intersects the two surfaces and makes sure that they are in the same coordinate
    /// system
    ///
    /// A more detailed cas analysis for the intersection can be achived by using the
    /// `NeoIntersectable` trait instead of this
    pub fn strict_intersection(&self, rhs: &Self) -> Option<Vec<Self>> {
        self.strict_boolop(rhs, option_intersection)
    }

    /// unions the two surfaces even if they are not located in the same coordinate system the
    /// right hand side argument surface is transformed to the coordinate system of the left hand
    /// side. This possibly results in a skewed geometry if the two coordinate systems are very
    /// different
    pub fn lax_union(&self, rhs: &Self) -> Option<Vec<Self>> {
        self.lax_boolop(rhs, option_union)
    }

    /// subtracts the first from the second surface even if they are not located in the same
    /// coordinate system the right hand side argument surface is transformed to the coordinate
    /// system of the left hand side. This possibly results in a skewed geometry if the two
    /// coordinate systems are very different
    pub fn lax_difference(&self, rhs: &Self) -> Option<Vec<Self>> {
        self.lax_boolop(rhs, option_difference)
    }

    /// intersects the two surfaces even if they are not located in the same
    /// coordinate system the right hand side argument surface is transformed to the coordinate
    /// system of the left hand side. This possibly results in a skewed geometry if the two
    /// coordinate systems are very different
    pub fn lax_intersection(&self, rhs: &Self) -> Option<Vec<Self>> {
        self.lax_boolop(rhs, option_intersection)
    }
}
