use crate::math::Matrix4;

/// 3D components able to be projected into a 2D space
///
/// # Generic
/// Output type of the projection
///
pub trait Projectable<T> {

    /// Returns the two-dimensional projection of the component
    ///
    /// # Arguments
    /// * `matrix` - Projection matrix
    /// * `offset` - Z-Offset to apply for screen distance
    ///
    fn get_projection(&self, matrix: &Matrix4, offset: f32) -> T;

}
