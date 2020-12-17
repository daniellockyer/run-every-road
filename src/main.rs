use geo::{algorithm::contains::Contains, line_string, prelude::HaversineLength};
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
    let boundaries_data = std::fs::read_to_string("data/map.geojson")?;
    let geojson = boundaries_data.parse::<geojson::GeoJson>()?;
    let collection: geo::GeometryCollection<f64> = geojson::quick_collection(&geojson)?;

    let boundaries = match &collection[0] {
        geo::Geometry::Polygon(p) => p,
        _ => panic!("Oh no"),
    };

    let data: Data = serde_json::from_str(&std::fs::read_to_string("data/data.json")?)?;

    let mut data_elements: HashMap<i64, geo::Coordinate<f64>> = HashMap::new();
    let mut lines = Vec::new();

    let mut total = 0.0;

    for element in data.elements {
        match element {
            DataElement::Node { id, lat, lon } => {
                let coord = geo::Coordinate { x: lon, y: lat };

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

                    let line = line_string![*first_node, *second_node];
                    total += line.haversine_length();

                    lines.push(geo::Geometry::LineString(line));
                }
            }
        }
    }

    let feature_collection = geojson::FeatureCollection {
        features: vec![geojson::Feature {
            geometry: Some(geojson::Geometry {
                value: geojson::Value::from(&geo::GeometryCollection(lines)),
                bbox: None,
                foreign_members: None,
            }),
            bbox: None,
            id: None,
            properties: None,
            foreign_members: None,
        }],
        bbox: None,
        foreign_members: None,
    };

    println!("Total: {:?}km", total / 1000.0);

    let file = std::fs::File::create("data/data-processed.json")?;
    serde_json::to_writer(file, &feature_collection)?;

    Ok(())
}
