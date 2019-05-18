
use crate::color::Color;
use crate::ray::{Ray, Hit, Intersectable};
use crate::lights::DirectionalLight;

/// Holds all information about the scene
pub struct Scene {
    /// Background color, assigned to pixels that are not covered by any object in the scene
    pub clear_color: Color,
    pub objects: Vec<Box<dyn Intersectable>>,
    pub light: DirectionalLight,
}

impl Scene {
    /// Check ray intersections against all objects in the scene and return the closest hit
    pub fn trace(&self, ray: &Ray) -> Option<Hit> {
        self.objects.iter()
            .filter_map(|obj| obj.intersect(ray))
            .min_by(|hit1, hit2| hit1.cmp(hit2))
    }
}
