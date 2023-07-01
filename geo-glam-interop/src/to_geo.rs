use crate::glam_alias::*;

pub trait ConvertToGeo {
    type GeoType;
    fn to_geo(&self) -> Self::GeoType;
}

// f32

impl ConvertToGeo for GlamPoint {
    type GeoType = geo::Coord<f32>;
    fn to_geo(&self) -> Self::GeoType {
        geo::Coord::from((self.x, self.y))
    }
}

impl ConvertToGeo for GlamLine {
    type GeoType = geo::Line<f32>;
    fn to_geo(&self) -> Self::GeoType {
        geo::Line::new(self.0.to_geo(), self.1.to_geo())
    }
}

impl ConvertToGeo for GlamLineString {
    type GeoType = geo::LineString<f32>;
    fn to_geo(&self) -> Self::GeoType {
        geo::LineString::new(self.iter().map(ConvertToGeo::to_geo).collect::<Vec<_>>())
    }
}

// Do we really need this?
impl ConvertToGeo for GlamPolygon {
    type GeoType = geo::Polygon<f32>;
    fn to_geo(&self) -> Self::GeoType {
        geo::Polygon::new(
            self.0.to_geo(),
            self.1.iter().map(ConvertToGeo::to_geo).collect::<Vec<_>>(),
        )
    }
}

// f64

impl ConvertToGeo for DGlamPoint {
    type GeoType = geo::Coord<f64>;
    fn to_geo(&self) -> Self::GeoType {
        geo::Coord::from((self.x, self.y))
    }
}

impl ConvertToGeo for DGlamLine {
    type GeoType = geo::Line<f64>;
    fn to_geo(&self) -> Self::GeoType {
        geo::Line::new(self.0.to_geo(), self.1.to_geo())
    }
}

impl ConvertToGeo for DGlamLineString {
    type GeoType = geo::LineString<f64>;
    fn to_geo(&self) -> Self::GeoType {
        geo::LineString::new(self.iter().map(ConvertToGeo::to_geo).collect::<Vec<_>>())
    }
}

// Do we really need this?
impl ConvertToGeo for DGlamPolygon {
    type GeoType = geo::Polygon<f64>;
    fn to_geo(&self) -> Self::GeoType {
        geo::Polygon::new(
            self.0.to_geo(),
            self.1.iter().map(ConvertToGeo::to_geo).collect::<Vec<_>>(),
        )
    }
}
