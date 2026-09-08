//! Firmware-independent timezone configuration using canonical IANA zones.

pub use chrono_tz::Tz;
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
///
/// The zone is a [`Tz`] on every firmware, such as `Europe/Vienna` or
/// `Etc/GMT-2`. Each firmware backend translates to and from its native
/// representation and determines which zones it accepts.
///
/// Over the wire (serde, `model_dump`) the zones are their IANA names. From
/// Python the fields are `zoneinfo.ZoneInfo` objects; the constructor and the
/// pydantic validator take either a `ZoneInfo` or an IANA name.
pub struct TimezoneConfig {
    /// The configured timezone.
    pub timezone: Option<Tz>,
    /// The timezones the miner accepts.
    pub available: Vec<Tz>,
}

#[cfg(feature = "python")]
#[pymethods]
impl TimezoneConfig {
    #[new]
    #[pyo3(signature = (timezone: "tzinfo | str | None" = None, available: "list[tzinfo | str] | None" = None))]
    fn py_new(
        timezone: Option<&Bound<'_, PyAny>>,
        available: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        use asic_rs_pydantic::PyPydanticType;
        Ok(Self {
            timezone: timezone
                .map(<Tz as PyPydanticType>::from_pydantic)
                .transpose()?,
            available: available
                .map(<Vec<Tz> as PyPydanticType>::from_pydantic)
                .transpose()?
                .unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// On the wire the zones are their IANA names, and an unknown name is
    /// rejected on the way in rather than carried along as text.
    #[test]
    fn config_serializes_zones_as_iana_names() -> anyhow::Result<()> {
        let config = TimezoneConfig {
            timezone: Some(Tz::Europe__Vienna),
            available: vec![Tz::Europe__Vienna, Tz::Etc__GMTMinus2],
        };
        let json = serde_json::to_value(&config)?;
        assert_eq!(
            json,
            serde_json::json!({
                "timezone": "Europe/Vienna",
                "available": ["Europe/Vienna", "Etc/GMT-2"],
            })
        );

        let back: TimezoneConfig = serde_json::from_value(json)?;
        assert_eq!(back.timezone, Some(Tz::Europe__Vienna));
        assert_eq!(back.available, config.available);

        let empty: TimezoneConfig = serde_json::from_str(r#"{"timezone":null,"available":[]}"#)?;
        assert_eq!(empty.timezone, None);

        let bogus: Result<TimezoneConfig, _> =
            serde_json::from_str(r#"{"timezone":"GMT+2","available":[]}"#);
        assert!(bogus.is_err());
        Ok(())
    }
}
