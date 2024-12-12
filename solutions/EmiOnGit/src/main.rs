use std::{collections::HashMap, env, io::BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = args.get(1).cloned().unwrap_or("weather_100.csv".into());
    let file_path = "../../samples/".to_string() + &file_name;
    let Ok(input_file) = std::fs::File::open(file_path.clone()) else {
        panic!("Could not read {file_path}");
    };
    let mut reader = std::io::BufReader::new(input_file);
    let mut line = String::new();
    let mut map: HashMap<String, Node> = HashMap::new();
    while let Ok(bytes_read) = reader.read_line(&mut line) {
        // EOF reached
        if bytes_read == 0 {
            break;
        }
        let Some((id, temperatur)) = line.split_once(";") else {
            line.clear();
            continue;
        };
        println!("split {temperatur}");
        // whitespaces might make the parsing fail so we split it off
        let Ok(temperatur) = temperatur.split_whitespace().next().unwrap().parse() else {
            line.clear();
            continue;
        };
        println!("push run");
        if let Some(v) = map.get_mut(id) {
            v.push(temperatur);
        } else {
            map.insert(id.to_string(), Node::new(id.to_string(), temperatur));
        }
        line.clear();
    }
    let mut expected: Vec<String> = map
        .into_iter()
        .map(|(id, value)| format!("{id}={}/{}/{},", value.min, value.max, value.mean))
        .collect();
    expected.sort();
    let expected_str = expected.join("\n");
    println!("{expected_str}");
    println!("finished");
}
struct Node {
    // pub id: String,
    pub max: f32,
    pub min: f32,
    pub mean: f32,
    pub count: usize,
}
impl Node {
    pub fn new(id: String, first_temp: f32) -> Node {
        Node {
            // id,
            max: first_temp,
            min: first_temp,
            mean: first_temp,
            count: 1,
        }
    }
    pub fn push(&mut self, temp: f32) {
        self.max = self.max.max(temp);
        self.min = self.min.min(temp);
        self.count = self.count + 1;
        let p = 1. / self.count as f32;
        self.mean = self.mean * (1. - p) + temp * p;
    }
}
