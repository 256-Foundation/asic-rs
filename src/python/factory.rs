use crate::miners::backends::traits::Miner as MinerTrait;
use crate::miners::factory::MinerFactory as MinerFactory_Base;
use crate::python::miner::Miner;

use futures::{Stream, StreamExt};
use pyo3::exceptions::{PyConnectionError, PyStopAsyncIteration, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyType;
use pyo3_async_runtimes::tokio::future_into_py;
use std::net::IpAddr;
use std::pin::Pin;
use std::str::FromStr;
use std::sync::Arc;

#[pyclass]
pub struct PyMinerStream {
    inner: Arc<tokio::sync::Mutex<Pin<Box<dyn Stream<Item = Box<dyn MinerTrait>> + Send>>>>,
}

impl PyMinerStream {
    fn new(inner: Pin<Box<dyn Stream<Item = Box<dyn MinerTrait>> + Send>>) -> Self {
        Self {
            inner: Arc::new(tokio::sync::Mutex::new(inner)),
        }
    }
}
#[pymethods]
impl PyMinerStream {
    pub fn __aiter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    pub fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        future_into_py(py, async move {
            let mut stream = inner.lock().await;
            if let Some(miner) = stream.next().await {
                Ok(Miner::from(miner))
            } else {
                Err(PyStopAsyncIteration::new_err("stream complete"))
            }
        })
    }
}

#[pyclass]
pub struct PyMinerStreamWithIP {
    inner: Arc<
        tokio::sync::Mutex<
            Pin<Box<dyn Stream<Item = (IpAddr, Option<Box<dyn MinerTrait>>)> + Send>>,
        >,
    >,
}

impl PyMinerStreamWithIP {
    fn new(
        inner: Pin<Box<dyn Stream<Item = (IpAddr, Option<Box<dyn MinerTrait>>)> + Send>>,
    ) -> Self {
        Self {
            inner: Arc::new(tokio::sync::Mutex::new(inner)),
        }
    }
}
#[pymethods]
impl PyMinerStreamWithIP {
    pub fn __aiter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    pub fn __anext__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.inner.clone();
        future_into_py(py, async move {
            let mut stream = inner.lock().await;
            if let Some((ip, miner_opt)) = stream.next().await {
                Ok((ip, miner_opt.map(Miner::new)))
            } else {
                Err(PyStopAsyncIteration::new_err("stream complete"))
            }
        })
    }
}

#[pyclass(module = "asic_rs")]
pub(crate) struct MinerFactory {
    inner: Arc<MinerFactory_Base>,
}

#[pymethods]
impl MinerFactory {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(MinerFactory_Base::new()),
        }
    }

    #[classmethod]
    pub fn from_subnet(_cls: &Bound<'_, PyType>, subnet: String) -> PyResult<Self> {
        let factory = MinerFactory_Base::new().with_subnet(&subnet);
        match factory {
            Ok(f) => Ok(Self { inner: Arc::new(f) }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    pub fn with_subnet(&mut self, subnet: &str) -> PyResult<()> {
        let inner = Arc::<MinerFactory_Base>::make_mut(&mut self.inner).clone();
        self.inner = Arc::new(
            inner
                .with_subnet(subnet)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        );
        Ok(())
    }

    #[classmethod]
    pub fn from_octets(
        _cls: &Bound<'_, PyType>,
        octet1: String,
        octet2: String,
        octet3: String,
        octet4: String,
    ) -> PyResult<Self> {
        let factory = MinerFactory_Base::new().with_octets(&octet1, &octet2, &octet3, &octet4);
        match factory {
            Ok(f) => Ok(Self { inner: Arc::new(f) }),
            Err(e) => Err(PyValueError::new_err(e.to_string())),
        }
    }

    pub fn with_octets(
        &mut self,
        octet1: String,
        octet2: String,
        octet3: String,
        octet4: String,
    ) -> PyResult<()> {
        let inner = Arc::<MinerFactory_Base>::make_mut(&mut self.inner).clone();
        self.inner = Arc::new(
            inner
                .with_octets(&octet1, &octet2, &octet3, &octet4)
                .map_err(|e| PyValueError::new_err(e.to_string()))?,
        );
        Ok(())
    }

    pub fn scan<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let miners = inner.scan().await;
            match miners {
                Ok(miners) => Ok(miners.into_iter().map(Miner::from).collect::<Vec<Miner>>()),
                Err(e) => Err(PyValueError::new_err(e.to_string())),
            }
        })
    }

    pub fn scan_stream<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyMinerStream>> {
        let inner = Arc::clone(&self.inner);
        Bound::new(py, PyMinerStream::new(inner.scan_stream()))
    }

    pub fn scan_stream_with_ip<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Bound<'py, PyMinerStreamWithIP>> {
        let inner = Arc::clone(&self.inner);
        Bound::new(py, PyMinerStreamWithIP::new(inner.scan_stream_with_ip()))
    }

    pub fn get_miner<'a>(&self, py: Python<'a>, ip: String) -> PyResult<Bound<'a, PyAny>> {
        let inner = Arc::clone(&self.inner);
        future_into_py(py, async move {
            let miner = inner.get_miner(IpAddr::from_str(&ip)?).await;
            match miner {
                Ok(Some(miner)) => Ok(Some(Miner::from(miner))),
                Ok(None) => Ok(None),
                Err(e) => Err(PyConnectionError::new_err(e.to_string())),
            }
        })
    }
}
