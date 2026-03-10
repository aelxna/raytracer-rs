use crate::util::vec2::Vec2;
use crate::util::vec3::Vec3;
use anyhow::{Result, bail};

pub trait Construct: Sized {
    fn construct<'a, I>(it: &mut I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>;
}

impl Construct for usize {
    fn construct<'a, I>(it: &mut I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        match it.next() {
            None => bail!("No value present"),
            Some(u) => Ok(u.parse()?),
        }
    }
}

impl Construct for f32 {
    fn construct<'a, I>(it: &mut I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        match it.next() {
            None => bail!("No value present"),
            Some(f) => Ok(f.parse()?),
        }
    }
}

impl Construct for Vec3 {
    fn construct<'a, I>(it: &mut I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        let x = <f32>::construct(it)?;
        let y = <f32>::construct(it)?;
        let z = <f32>::construct(it)?;

        Ok(Vec3::new(x, y, z))
    }
}

impl Construct for Vec2 {
    fn construct<'a, I>(it: &mut I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        let x = <f32>::construct(it)?;
        let y = <f32>::construct(it)?;

        Ok(Vec2::new(x, y))
    }
}
