#[cfg(feature = "python")]
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg_attr(
    feature = "python",
    pyclass(from_py_object, get_all, module = "asic_rs")
)]
#[cfg_attr(feature = "python", asic_rs_pydantic::py_pydantic_model)]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Timezone configuration.
pub struct TimezoneConfig {
    /// The configured timezone (a named IANA zone on BraiinsOS, or a fixed UTC offset like "GMT+1" on VNish).
    pub timezone: Option<String>,
    /// The timezones the miner accepts (named zones where available; may be a default fallback list).
    pub available: Vec<String>,
}

#[cfg(feature = "python")]
#[pymethods]
impl TimezoneConfig {
    #[new]
    #[pyo3(signature = (timezone = None, available = None))]
    fn py_new(timezone: Option<String>, available: Option<Vec<String>>) -> Self {
        Self {
            timezone,
            available: available.unwrap_or_default(),
        }
    }
}
