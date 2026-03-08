use crate::util::vec3::*;

struct Light {
    pub pos: Vec3,
    pub point: bool,
    pub intensity: f32,
}

fn light_new(p: &Vec3, w: u32, i: f32) -> Light {
    Light {
        pos: p.clone(),
        point: w != 0,
        intensity: i.clamp(0f32, 1f32),
    }
}
