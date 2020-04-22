use std::hash::{Hash, Hasher};

use pyo3::class::basic::CompareOp;
use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyModule};
use pyo3::{
    PyAny, PyCell, PyIterProtocol, PyMappingProtocol, PyObject, PyRefMut, PyResult, Python,
    ToPyObject,
};

use pyo3::types::IntoPyDict;
use pyrpds::Object;

use pyrpds::py_object_protocol;

#[pyclass]
#[derive(PartialEq, Eq, Hash)]
struct NodesContainer {
    value: crate::nodes_container::NodesContainer<Object>,
}

#[pymethods]
impl NodesContainer {
    #[new]
    fn new() -> Self {
        NodesContainer {
            value: crate::nodes_container::NodesContainer::new(),
        }
    }

    fn add_node(&self, node: PyObject) -> Self {
        NodesContainer {
            value: self.value.add_node(Object::new(node)),
        }
    }

    fn remove_node(&self, node: PyObject) -> Self {
        NodesContainer {
            value: self.value.remove_node(&Object::new(node)),
        }
    }

    #[call]
    fn __call__(&self) -> PyResult<pyrpds::iterators::PyObjectIterator> {
        let mut elements = std::vec::Vec::new();
        for element in self.value.iter() {
            elements.push(pyrpds::object::extract_py_object(Some(element))?)
        }

        Ok(pyrpds::iterators::PyObjectIterator::new(
            elements.into_iter(),
        ))
    }
}

#[pyproto]
impl PySequenceProtocol for NodesContainer {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.value.size();
        Ok(len)
    }

    fn __contains__(&self, py_object: PyObject) -> PyResult<bool> {
        Ok(self.value.contains(&Object::new(py_object)))
    }
}

#[pyproto]
impl PyMappingProtocol for NodesContainer {
    fn __getitem__(&self, _item: PyObject) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        // todo!("pyrpds::Map should be constructed here directly in Rust\
        //        Currently, that is not possible because another version of Map type gets created in Python\
        //        ");
        let locals = [("pyrpds", py.import("pyrpds")?)].into_py_dict(py);
        let code = "pyrpds.pmap()";
        let dict: &PyAny = py.eval(code, None, Some(&locals))?;
        let dict = dict.to_object(py);

        Ok(dict)
    }
}

#[pyproto]
impl PyIterProtocol for NodesContainer {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<pyrpds::iterators::PyObjectIterator> {
        let mut elements = std::vec::Vec::new();
        for element in slf.value.iter() {
            elements.push(pyrpds::object::extract_py_object(Some(element))?)
        }

        Ok(pyrpds::iterators::PyObjectIterator::new(
            elements.into_iter(),
        ))
    }
}

py_object_protocol!(NodesContainer);

pub fn py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<NodesContainer>()?;

    Ok(())
}
