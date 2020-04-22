use std::hash::{Hash, Hasher};

use pyo3::class::{PyObjectProtocol, PySequenceProtocol};
use pyo3::prelude::{pyclass, pymethods, pyproto, PyModule};
use pyo3::{
    IntoPy, PyAny, PyCell, PyIterProtocol, PyMappingProtocol, PyObject, PyRefMut, PyResult, Python,
    ToPyObject,
};

use pyo3::types::IntoPyDict;
use pyrpds::Object;

use pyrpds::py_object_protocol;

#[pyclass]
#[derive(PartialEq, Eq, Hash)]
struct DiGraph {
    value: crate::digraph::DiGraph<Object>,
}

#[pymethods]
impl DiGraph {
    #[new]
    fn new() -> Self {
        DiGraph {
            value: crate::digraph::DiGraph::new(),
        }
    }

    fn add_node(&self, node: PyObject) -> Self {
        DiGraph {
            value: self.value.add_node(Object::new(node)),
        }
    }

    fn remove_node(&self, node: PyObject) -> Self {
        DiGraph {
            value: self.value.remove_node(&Object::new(node)),
        }
    }

    fn add_edge(&self, u: PyObject, v: PyObject) -> Self {
        DiGraph {
            value: self.value.add_edge(&Object::new(u), &Object::new(v)),
        }
    }

    fn remove_edge(&self, u: PyObject, v: PyObject) -> Self {
        DiGraph {
            value: self.value.remove_edge(&Object::new(u), &Object::new(v)),
        }
    }

    fn edges(&self) -> PyResult<pyrpds::Vector> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let edges = self.value.edges();
        let mut python_edges = pyrpds::Vector::new();
        for (_, neighbors) in edges.iter() {
            let mut python_neighbors = pyrpds::Vector::new();
            for neighbor in neighbors {
                python_neighbors = python_neighbors.append(neighbor.py_object.clone_ref(py))?;
            }
            let python_neighbors = python_neighbors.into_py(py);
            python_edges = python_edges.append(python_neighbors)?;
        }
        let python_edges = python_edges;
        Ok(python_edges)
    }
}

#[pyproto]
impl PySequenceProtocol for DiGraph {
    fn __len__(&self) -> PyResult<usize> {
        let len = self.value.size();
        Ok(len)
    }

    fn __contains__(&self, py_object: PyObject) -> PyResult<bool> {
        Ok(self.value.contains(&Object::new(py_object)))
    }
}

#[pyproto]
impl PyMappingProtocol for DiGraph {
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
impl PyIterProtocol for DiGraph {
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

py_object_protocol!(DiGraph);

pub fn py_binding(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DiGraph>()?;

    Ok(())
}
