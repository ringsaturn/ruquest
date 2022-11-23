use futures::{stream, StreamExt};
use lazy_static::lazy_static;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyDict;
use reqwest::Client;
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
async fn batch_get(_py: Python, urls: Vec<&str>) {
    // TODO(ringsaturn): add timeout control
    // TODO(ringsaturn): custom header
    // TODO(ringsaturn): return response

    // https://stackoverflow.com/a/51047786/6713916
    let bodies = stream::iter(&urls)
        .map(|url| {
            let client = &CLIENT;
            async move {
                client.get(*url).send().await
            }
        })
        .buffer_unordered(urls.len());

    bodies
        .for_each(|resp| async {
            match resp {
                // Ok(resp) => println!("Got {:?} bytes as {:?}", resp.text().await.unwrap(), &(resp.bytes().await.unwrap().to_vec())[..]),
                Ok(resp) => println!("Got {:?}", resp.text().await.unwrap()),
                Err(e) => eprintln!("Got an error: {}", e),
            }
        })
        .await;
}

/// A Python module implemented in Rust.
#[pymodule]
fn ruquest(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add_function(wrap_pyfunction!(batch_get, m)?)?;
    Ok(())
}
