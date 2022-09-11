pub trait ReqlOpsGeometry: ReqlOps {
    fn distance<A>(&self, geometry: A) -> cmd::distance::DistanceBuilder
    where
        A: ReqlOpsGeometry + Serialize,
    {
        cmd::distance::DistanceBuilder::new(geometry)._with_parent(self.get_parent())
    }
    
    fn to_geojson<A>(&self) -> cmd::to_geojson::ToGeoJsonBuilder<A>
    where
        A: Unpin + Serialize + DeserializeOwned + Clone,
    {
        cmd::to_geojson::ToGeoJsonBuilder::new()._with_parent(self.get_parent())
    }

    fn includes<A>(&self, geometry: A) -> cmd::includes::IncludesBuilder<bool>
    where
        A: ReqlOpsGeometry + Serialize,
    {
        cmd::includes::IncludesBuilder::new(geometry)._with_parent(self.get_parent())
    }

    fn intersects<T>(&self, geometry: T) -> cmd::intersects::geometry::IntersectsBuilder
    where
        T: ReqlOpsGeometry + Serialize,
    {
        cmd::intersects::geometry::IntersectsBuilder::new(geometry)._with_parent(self.get_parent())
    }
}
