use std::{error::Error, fmt};

/// Transform a geometry from one CRS to another.
///
/// # Examples
///
/// ```
/// use geo::{self, prelude::*};
///
/// let point: geo::Point<f32> = geo::point!(x: -36.508, y: -54.2815);
///
/// assert_eq!(
///     point.transform("EPSG:4326", "EPSG:3857").unwrap(),
///     geo::point!(x: -4064052.0, y: -7223650.5)
/// );
/// ```
pub trait Transform<T> {
    type Output;

    fn transform(&self, source_srs: &str, target_srs: &str)
        -> Result<Self::Output, TransformError>;
}

#[derive(Debug)]
pub enum TransformError {
    UnknownCrs,
    ProjError(Box<dyn Error + Send + Sync>),
}

impl fmt::Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransformError::UnknownCrs => write!(f, "Unknown CRS"),
            TransformError::ProjError(err) => write!(f, "{}", err),
        }
    }
}

impl Error for TransformError {}

impl<T, G> Transform<T> for G
where
    T: crate::CoordFloat,
    G: crate::algorithm::map_coords::TryMapCoords<T, T>,
{
    type Output = G::Output;
    fn transform(
        &self,
        source_srs: &str,
        target_srs: &str,
    ) -> Result<Self::Output, TransformError> {
        let transformer = proj::Proj::new_known_crs(source_srs, target_srs, None)
            .ok_or(TransformError::UnknownCrs)?;

        self.try_map_coords(|&(x, y)| {
            transformer
                .convert((x, y))
                .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)
        })
        .map_err(|e| TransformError::ProjError(e))
    }
}
