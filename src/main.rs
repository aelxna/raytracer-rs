use anyhow::{Context, Result, anyhow, bail};
use std::env;
use std::fs;
use std::str;

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
    let scene: Scene = Scene::from(file_in)?;

    dbg!(scene);

    // TODO: determine color values at each pixel

    // TODO: write header to output file

    // TODO: write pixels to output file
    //
    Ok(())
}
