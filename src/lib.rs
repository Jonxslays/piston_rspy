use pyo3::prelude::*;

mod client;
mod executor;
mod models;

pub use client::Client;
pub use executor::ExecResponse;
pub use executor::ExecResult;
pub use executor::Executor;
pub use models::File;
pub use models::Runtime;

#[pymodule]
pub fn piston_rspy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Runtime>()?;
    m.add_class::<File>()?;
    m.add_class::<ExecResult>()?;
    m.add_class::<ExecResponse>()?;
    m.add_class::<Executor>()?;
    m.add_class::<Client>()?;

    Ok(())
}
