mod event;

use std::collections::HashMap;
use std::sync::Arc;
use log::{debug};
use pyo3::{prelude::*};
use crate::agent::event::{EventManager};

#[derive(Debug, Clone)]
#[pyclass(subclass)]
pub struct Agent {
    #[pyo3(get)]
    pub domain: String,
    pub event_manager: Arc<EventManager>,
}

#[pymethods]
impl Agent {
    #[new]
    fn new(_py: Python<'_>, domain: String, id: Option<String>) -> Self {
        debug!("Initializing Agent");

        // Create domain with Id fo there is one
        let domain = match id {
            Some(id) => format!("{}:{}", domain, id),
            None => domain,
        };

        Agent {
            domain,
            event_manager: Arc::new(EventManager::new()),
        }
    }

    fn register_event_handler(&self, _py: Python<'_>, name: String, handler: Py<PyAny>) -> PyResult<()> {
        debug!("Registering event {:?}, {:?}",name, handler);
        Ok(())
    }


    pub fn start<'a>(&'a self, _py: Python<'a>) -> PyResult<&'a PyAny> {
        debug!("Starting agent: {:?}", self.domain);
        let event_manager = Arc::clone(&self.event_manager);
        pyo3_asyncio::async_std::future_into_py(_py, async move {
            debug!("Starting async");
            Python::with_gil(|py| {
                let event_manager = event_manager;
                event_manager.emit(py,"on_start".to_string()).unwrap();
            });
            Ok(Python::with_gil(|py| py.None()))
        })
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Agent: {}", self.domain))
    }
}

