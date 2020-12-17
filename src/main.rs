use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DataElement {
    #[serde(rename = "node")]
    Node { id: i64, lat: f64, lon: f64 },
    #[serde(rename = "way")]
    Way { id: i64, nodes: Vec<i64> },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    version: f64,
    generator: String,
    osm3s: Value,
    elements: Vec<DataElement>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string("data.json")?;
    let data: Data = serde_json::from_str(&contents)?;

    let mut data_elements: HashMap<i64, (f64, f64)> = HashMap::new();

    let mut response = Vec::new();

    for element in data.elements {
        match element {
            DataElement::Node { id, lat, lon } => {
                data_elements.insert(id, (lat, lon));
            }
            DataElement::Way { nodes, .. } => {
                for w in nodes.windows(2) {
                    let first_node = data_elements.get(&w[0]).unwrap();
                    let second_node = data_elements.get(&w[1]).unwrap();
                    let mut i_response = Vec::new();

                    i_response.push((first_node.0, first_node.1));
                    i_response.push((second_node.0, second_node.1));
                    response.push(i_response);
                }
            }
        }
    }

    let file = std::fs::File::create("data-processed.json")?;
    serde_json::to_writer(file, &response)?;

    Ok(())
}
