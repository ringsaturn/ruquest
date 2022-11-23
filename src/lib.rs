use futures::future;
use lazy_static::lazy_static;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyDict;
use reqwest::Client;
use std::iter::Iterator;
use tokio;

#[pyfunction]
fn get(_py: Python, url: String) -> PyResult<&PyDict> {
    // TODO(ringsaturn): add timeout
    // TODO(ringsaturn): custom header
    let res = reqwest::blocking::get(url).unwrap();

    let d: &PyDict = PyDict::new(_py);
    d.set_item("status_code", res.status().as_u16())?;
    d.set_item(
        "content",
        PyBytes::new(_py, &(res.bytes().unwrap().to_vec())[..]),
    )?;
    return Ok(d);
}

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

#[tokio::main]
#[pyfunction]
async fn batch_get<'a>(_py: Python<'a>, urls: Vec<&str>) -> PyResult<Vec<&'a PyDict>> {
    // TODO(ringsaturn): add timeout control
    // TODO(ringsaturn): custom header
    // TODO(ringsaturn): pass idx to await process

    // https://stackoverflow.com/a/51047786/6713916

    let llen = urls.len();
    let urls_for_loop = urls.to_owned();

    let bodies = future::join_all(urls.into_iter().map(|url| {
        let client = &CLIENT;
        async move { client.get(url).send().await }
    }))
    .await;

    let mut results: Vec<&'a PyDict> = vec![PyDict::new(_py); llen];

    for b in bodies {
        match b {
            Ok(resp) => {
                let mut result_idx: usize = 0;
                for (idx, url) in urls_for_loop.iter().enumerate() {
                    // println!("loop: {:?} {:?} {:?}", idx, resp.url().as_str(), url);
                    if resp.url().as_str() == *url {
                        result_idx = idx as usize
                    }
                }
                // println!("{:?} {:?} {:?}", result_idx, resp.url().as_str(), resp.status());
                let d: &PyDict = PyDict::new(_py);
                d.set_item("status_code", resp.status().as_u16()).unwrap();
                d.set_item(
                    "content",
                    PyBytes::new(_py, &(resp.bytes().await.unwrap().to_vec())[..]),
                )
                .unwrap();
                results[result_idx] = d;
            }
            Err(e) => eprintln!("Got an error: {}", e),
        }
    }

    return Ok(results);
}

/// A Python module implemented in Rust.
#[pymodule]
fn ruquest(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_function(wrap_pyfunction!(batch_get, m)?)?;
    Ok(())
}
