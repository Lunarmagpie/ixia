use core::f64::consts::E;

use getrandom::getrandom;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

create_exception!(bindings, OsRandomError, PyException);

#[inline]
fn random_bytes(bytes: &mut [u8]) -> PyResult<()> {
    match getrandom(bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(OsRandomError::new_err(e.to_string())),
    }
}

#[pyfunction]
fn random() -> PyResult<f64> {
    let mut bytes = [0_u8; 8];
    random_bytes(&mut bytes)?;
    let int = u64::from_be_bytes(bytes);
    Ok((int >> 11) as f64 * (2f64).powf(-53f64))
}

#[pyfunction]
fn normal_variate(mu: f64, sigma: f64) -> PyResult<f64> {
    let nv = 4. * E.powf(-0.5) / f64::sqrt(2.0);
    loop {
        let u1 = random()?;
        let u2 = 1. - random()?;

        let z = nv * (u1 - 0.5) / u2;
        if z * z / 4. <= -f64::ln(u2) {
            break Ok(mu + z * sigma);
        }
    }
}

#[pymodule]
fn ixia_bindings(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(random, m)?)?;
    m.add_function(wrap_pyfunction!(normal_variate, m)?)?;
    Ok(())
}
