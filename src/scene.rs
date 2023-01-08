use crate::Camera;
use crate::lights::LightList;
use crate::hit::HittableList;

pub struct Scene<'a> {
    camera: Box<dyn Camera>,
    objects: &'a HittableList,
    lights: &'a LightList
}

impl Scene<'_> {
    pub fn new<'a>(camera: Box<dyn Camera>, objects: &'a HittableList, lights: &'a LightList) -> Scene<'a> {
        Scene {
            camera,
            objects,
            lights
        }
    }
}