use anyhow::{Context, Result, anyhow, bail};
use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use std::env;
use util::vec3::*;

use crate::io::scene::*;

pub mod io;
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

    dbg!(&scene);

    let width: u32 = match &scene.width {
        None => bail!("No width argument supplied"),
        Some(w) => w.clone(),
    };
    let height: u32 = match &scene.height {
        None => bail!("No height argument supplied"),
        Some(h) => h.clone(),
    };

    // TODO: determine color values at each pixel
    let progress = ProgressBar::new((width * height) as u64);

    let img = RgbImage::from_par_fn(width, height, |x, y| {
        let color = Vec3::new(0.0, 0.0, 0.0);
        progress.inc(1);
        Rgb::from(color.to_rgb())
    });

    progress.finish();

    img.save(file_out)
        .with_context(|| "Failed to save output file")?;

    Ok(())
}
