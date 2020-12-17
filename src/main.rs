use geo::algorithm::contains::Contains;
use geo::prelude::HaversineDistance;
use geo_types::point;

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
    let boundaries_data = std::fs::read_to_string("map.geojson")?;
    let geojson = boundaries_data.parse::<geojson::GeoJson>()?;
    let collection: geo_types::GeometryCollection<f64> = geojson::quick_collection(&geojson)?;

    let boundaries = match &collection[0] {
        geo_types::Geometry::Polygon(p) => p,
        _ => panic!("Oh no"),
    };

    let data: Data = serde_json::from_str(&std::fs::read_to_string("data.json")?)?;

    let mut data_elements: HashMap<i64, geo_types::Coordinate<f64>> = HashMap::new();
    let mut response = Vec::new();

    let mut total = 0.0;

    for element in data.elements {
        match element {
            DataElement::Node { id, lat, lon } => {
                let coord = geo_types::Coordinate { x: lon, y: lat };

                if !boundaries.contains(&coord) {
                    continue;
                }

                data_elements.insert(id, coord);
            }
            DataElement::Way { nodes, .. } => {
                for w in nodes.windows(2) {
                    if !data_elements.contains_key(&w[0]) || !data_elements.contains_key(&w[1]) {
                        continue;
                    }

                    let first_node = data_elements.get(&w[0]).unwrap();
                    let second_node = data_elements.get(&w[1]).unwrap();
                    let mut i_response = Vec::new();

                    let p1 = point!(x: first_node.x, y: first_node.y);
                    let p2 = point!(x: second_node.x, y: second_node.y);

                    let distance = p1.haversine_distance(&p2);
                    total += distance;

                    i_response.push((first_node.y, first_node.x));
                    i_response.push((second_node.y, second_node.x));
                    response.push(i_response);
                }
            }
        }
    }

    println!("Total: {:?}km", total / 1000.0);

    let file = std::fs::File::create("data-processed.json")?;
    serde_json::to_writer(file, &response)?;

    Ok(())
}
