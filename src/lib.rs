use std::error::Error;
use std::fmt::{self, Write};

pub trait Because: Error {
    fn because(&self) -> std::result::Result<String, fmt::Error> {
        let mut w = String::new();
        let mut idx = 1;
        let mut source = self.source();
        while source.is_some() {
            let src = source.unwrap();
            writeln!(w, "    {}: {}", idx, src)?;
            idx += 1;
            source = src.source()
        }

        Ok(w)
    }
}

impl<T> Because for T where T: Error + ?Sized {}
