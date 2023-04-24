mod app;
mod crossterm;
mod ui;

use crate::crossterm::create_and_run_app;
use std::io;

fn main() -> Result<(), io::Error> {
    create_and_run_app()?;
    Ok(())
}
