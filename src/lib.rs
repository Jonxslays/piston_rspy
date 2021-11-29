use pyo3::prelude::*;

mod client;
mod executor;
mod models;

pub use models::Runtime;
pub use models::File;
pub use executor::ExecResponse;
pub use executor::ExecResult;
pub use executor::Executor;

#[pymodule]
pub fn piston_rspy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<models::Runtime>()?;
    m.add_class::<models::File>()?;
    m.add_class::<executor::ExecResult>()?;
    m.add_class::<executor::ExecResponse>()?;
    m.add_class::<executor::Executor>()?;
    Ok(())
}
