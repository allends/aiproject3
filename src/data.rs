
use crate::field::Field;
use crate::make_distance_map_dog;
use crate::make_distance_map_sheep;
use crate::solve_markov;

use rand::Rng;

use std::fs::File;

use std::io::Read;
use std::io::Result;

use std::collections::HashMap;

use std::io::Write;
use std::path::Path;

use crate::field::Dog;

use crate::field::Sheep;

pub(crate) fn to_key(key: (Sheep, Dog)) -> String {
    serde_json::to_string(&key).unwrap_or("".to_string())
}

pub(crate) fn from_serialized(json: String) -> (Sheep, Dog) {
    serde_json::from_str(&json).unwrap_or((Sheep::new(), Dog::new()))
}

pub(crate) fn save_utility_map<P: AsRef<Path>>(
    path: P,
    map: HashMap<(Sheep, Dog), f32>,
) -> Result<()> {
    let mut new_map = HashMap::new();
    for (key, value) in &map {
        new_map.insert(to_key(*key), value);
    }
    let mut f = File::create(path)?;
    println!("created the file");
    match serde_json::to_vec(&map) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    let buf = serde_json::to_vec(&new_map)?;
    println!("serialized the data structure");
    f.write_all(&buf[..])?;
    println!("wrote to the file");
    Ok(())
}

pub(crate) fn inner_load_utility_map<P: AsRef<Path>>(path: P) -> Option<HashMap<String, f32>> {
    if let Ok(mut file) = File::open(path) {
        let mut buf = vec![];
        if file.read_to_end(&mut buf).is_ok() {
            if let Ok(map) = serde_json::from_slice(&buf[..]) {
                return Some(map);
            }
        }
    }
    None
}

pub(crate) fn load_utility_map<P: AsRef<Path>>(path: P) -> HashMap<(Sheep, Dog), f32> {
    match inner_load_utility_map(path) {
        Some(map) => {
            let mut new_map = HashMap::new();
            for (key, value) in &map {
                new_map.insert(from_serialized(key.clone()), *value);
            }
            return new_map;
        }
        None => return solve_markov::generate_optimal_utlility(),
    }
}

pub(crate) fn partition_data(
    map: &HashMap<(Sheep, Dog), f32>,
) -> (
    Vec<((Sheep, Dog), f32)>,
    Vec<((Sheep, Dog), f32)>,
    Vec<((Sheep, Dog), f32)>,
) {
    let mut training_data = Vec::new();
    let mut testing_data = Vec::new();
    let mut validation_data = Vec::new();
    let mut rng = rand::thread_rng();

    for ((sheep, dog), value) in map {
        let field = Field::with(*sheep, *dog);
        if !field.is_valid() {
            continue;
        }
        let probability = rng.gen_range(0..100);
        if probability < 70 {
            training_data.push(((*sheep, *dog), *value));
        } else if probability < 85 {
            testing_data.push(((*sheep, *dog), *value));
        } else {
            validation_data.push(((*sheep, *dog), *value));
        }
    }
    (training_data, testing_data, validation_data)
}

pub(crate) fn save_partitioned_data<P: AsRef<Path>>(
    path: P,
    data: (
        Vec<((Sheep, Dog), f32)>,
        Vec<((Sheep, Dog), f32)>,
        Vec<((Sheep, Dog), f32)>,
    ),
) -> Result<()> {
    let mut f = File::create(path)?;
    println!("created the file");
    match serde_json::to_vec(&data) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    let buf = serde_json::to_vec(&data)?;
    println!("serialized the data structure");
    f.write_all(&buf[..])?;
    println!("wrote to the file");
    Ok(())
}

pub(crate) fn load_partitioned_data<P: AsRef<Path>>(
    path: P,
    map: &HashMap<(Sheep, Dog), f32>,
) -> (
    Vec<((Sheep, Dog), f32)>,
    Vec<((Sheep, Dog), f32)>,
    Vec<((Sheep, Dog), f32)>,
) {
    if let Ok(mut file) = File::open(path) {
        let mut buf = vec![];
        if file.read_to_end(&mut buf).is_ok() {
            if let Ok(partitioned_data) = serde_json::from_slice(&buf[..]) {
                return partitioned_data;
            }
        }
    }
    return partition_data(map);
}

pub(crate) fn save_distance_data<P: AsRef<Path>>(
    path: P,
    data: (
        HashMap<Sheep, f32>,
        HashMap<(Sheep, Dog), f32>
    ),
) -> Result<()> {
    let mut f = File::create(path)?;
    println!("created the file");
    match serde_json::to_vec(&data) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
    let buf = serde_json::to_vec(&data)?;
    println!("serialized the data structure");
    f.write_all(&buf[..])?;
    println!("wrote to the file");
    Ok(())
}

pub(crate) fn load_distance_data<P: AsRef<Path>>(
    path: P,
) -> (
    HashMap<Sheep, f32>,
    HashMap<(Sheep, Dog), f32>
) {
    if let Ok(mut file) = File::open(path) {
        let mut buf = vec![];
        if file.read_to_end(&mut buf).is_ok() {
            if let Ok(partitioned_data) = serde_json::from_slice(&buf[..]) {
                return partitioned_data;
            }
        }
    }
    return (make_distance_map_sheep(), make_distance_map_dog());
}