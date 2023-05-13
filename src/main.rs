use serde_json::{json, Value};

fn pluck(key: &str, data: Value) -> Vec<u64> {
    data.as_array().unwrap().iter().map(|x| x[key].as_u64().unwrap()).collect::<Vec<_>>()
}

fn main() {
    let data = json!([
        {"year": 2018, "sales": 1_000},
        {"year": 2019, "sales": 1_234},
        {"year": 2020, "sales": 1_500},
        {"year": 2021, "sales": 1_750},
    ]);
    let plucked = pluck("sales", data);
    println!("{:?}", plucked);
}
