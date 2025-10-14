use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::fs::File;
use std::path::PathBuf;
use csv::StringRecord;

#[pyclass]
struct Account {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    acc_type: String,
    #[pyo3(get, set)]
    number: usize,
}

#[pymethods]
impl Account {
    #[new]
    fn new(name: String, acc_type: String, number: usize) -> Self {
        Account {
            name,
            acc_type,
            number,
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "Account(name='{}', acc_type='{}', number={})",
            self.name, self.acc_type, self.number
        )
    }

}

#[pyclass]
struct AccountReader {
    reader: csv::Reader<File>,
}

#[pymethods]
impl AccountReader {
    #[new]
    fn new(path: PathBuf) -> PyResult<Self> {
        let file = File::open(&path)
            .map_err(|e| PyValueError::new_err(format!("open {}: {}", path.display(), e)))?;
        let reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        Ok(Self { reader })
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> PyResult<Option<Account>> {
        let mut rec = StringRecord::new();
        match slf.reader.read_record(&mut rec) {
            Ok(true) => {
                // Suppose des en-têtes: name,type,number
                // (en démo, on peut aussi utiliser rdr.deserialize::<AccountRow>())
                let name = rec.get(0).unwrap_or_default().to_string();
                let acc_type = rec.get(1).unwrap_or_default().to_string();
                let number = 
                    match rec.get(2).unwrap_or_default().parse::<usize>() {
                        Ok(n) => n,
                        Err(_) => {
                            return Err(PyValueError::new_err(format!("Invalid number: {}", rec.get(2).unwrap_or_default())));
                        }
                    };

                let acc = Account { name, acc_type, number };
                Ok(Some(acc))
            }
            Ok(false) => Ok(None), // fin d'itération
            Err(err) => Err(PyValueError::new_err(format!("CSV error: {}", err))),
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn piledger(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Account>()?;
    m.add_class::<AccountReader>()?;
    Ok(())
}