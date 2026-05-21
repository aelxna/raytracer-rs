use crate::raytrace::*;
use crate::util::vec3::*;
use anyhow::{Context, Result, anyhow};
use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use std::env;

use crate::io::scene::*;

pub mod io;
pub mod raytrace;
pub mod util;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(anyhow!(
            "Two parameters expected, only received {}",
            args.len() - 1
        ));
    }

    let file_in = &args[1];
    let file_out = &args[2];

    // TODO: generate config from lines
    let scene: Scene = Scene::from(file_in)?.clone();

    // dbg!(&scene);

    let width: u32 = scene.width.ok_or(anyhow!("No width argument supplied"))?;
    let height: u32 = scene.height.ok_or(anyhow!("No height argument supplied"))?;
    let eye: Vec3 = scene.eye.ok_or(anyhow!("No eye argument supplied"))?;
    let eta: f32 = scene.eta.ok_or(anyhow!("No eta argument supplied"))?;

    let ds: Dimensions = image_setup(&scene)?;

    // TODO: determine color values at each pixel
    let progress = ProgressBar::new((width * height) as u64);

    let img = RgbImage::from_par_fn(width, height, |x, y| {
        // create ray pointing to that pixel
        let loc: Vec3 = ds.ul + (ds.dx * (x as f32)) + (ds.dy * (y as f32));
        let r: Ray3 = Ray3::new(eye, (loc - eye).norm());
        let mut stack: Vec<f32> = Vec::new();
        stack.push(eta);

        // pixels[i] = shade_ray(r, c, 0, NULL, st, debug);
        let color = shade_ray(r, &scene, 0, None, &mut stack);
        progress.inc(1);
        Rgb::from(color.to_rgb())
    });

    progress.finish();

    img.save(file_out)
        .with_context(|| "Failed to save output file")?;

    Ok(())
}
