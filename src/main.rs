mod app;
mod backend;
mod settlements;
mod ui;

use std::io;

fn main() -> Result<(), io::Error> {
    backend::create_and_run_app()?;
    Ok(())
}
