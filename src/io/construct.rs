use crate::util::vec2::Vec2;
use crate::util::vec3::Vec3;
use anyhow::{Result, bail};

pub trait Construct: Sized {
    fn construct<'a, I>(it: &mut I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>;
}

// implements Construct for primitive types
macro_rules! impl_construct {
    ($ty:ty) => {
        impl Construct for $ty {
            fn construct<'a, I>(it: &mut I) -> Result<Self>
            where
                I: Iterator<Item = &'a str>,
            {
                match it.next() {
                    None => bail!("No value present"),
                    Some(x) => Ok(x.parse()?),
                }
            }
        }
    };
}

impl_construct!(usize);
impl_construct!(u32);
impl_construct!(i32);
impl_construct!(f32);

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
