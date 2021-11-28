use pyo3::prelude::*;

mod client;
mod executor;
mod models;

use client::*;
use executor::*;
use models::*;

#[pymodule]
fn piston_rspy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Runtime>()?;
    m.add_class::<File>()?;
    Ok(())
}
