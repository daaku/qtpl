pub use qtpl_macros::{child, render, tpl, tplfn};
use std::io::{Result, Write};

pub trait Render {
    fn render(self, destination: &mut dyn Write) -> Result<()>;
}

impl<F> Render for F
where
    F: FnOnce(&mut dyn Write) -> Result<()>,
{
    fn render(self, destination: &mut dyn Write) -> Result<()> {
        self(destination)
    }
}
