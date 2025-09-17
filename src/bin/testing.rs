use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

/// Read integers from a text file.
/// The file should have one number on each line.
fn read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;         // reading lines can fail
        numbers.push(line.parse()?);     // parsing integers can fail
    }
    Ok(numbers)
}

fn main () -> io::Result<()> {
    let s = String::from("I am a new string");
    let path = "src/bin/numbers.txt";
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let numbers: Vec<i64> = read_numbers(&mut reader).unwrap();
    println!("{:?}", numbers);
    
    let one: bool = true;
    let one_int: u32 = one.into();
    println!("Into bool: {}", one_int);

    let point_light_fields = vec!["ambient", "diffuse", "specular", "constant", "linear", "quadratic"];
    let mut point_light_properties: HashMap<&str, Vec<Vec<f32>>> = HashMap::new();

    // properties setup
    point_light_properties.insert("ambient", vec![vec![0.1, 0.1, 0.1], vec![0.1, 0.1, 0.1], vec![0.1, 0.1, 0.1], vec![0.1, 0.1, 0.1]]);
    point_light_properties.insert("diffuse", vec![vec![1.0, 0.65, 0.0], vec![0.4, 0.3, 0.1], vec![1.0, 0.0, 0.0], vec![0.0, 0.0, 1.0]]);
    point_light_properties.insert("specular", vec![vec![1.0, 1.0, 1.0]; 4]);
    point_light_properties.insert("constant", vec![vec![1.0]; 4]);
    point_light_properties.insert("linear", vec![vec![0.09]; 4]);
    point_light_properties.insert("quadratic", vec![vec![0.032]; 4]);

    // point lights setup
    for field in &point_light_fields {
        for i in 0..4 {
            let mut point_light_name = format!("pointLights[{i}].");
            point_light_name.push_str(field);
            let property = point_light_properties.get(field).unwrap();
            println!("Property: {} -- Value:{:?}", point_light_name, property[i]);
        }
    }
    

    // let bytes: &[u8] = s.as_ref();
    // let contents: &str = s.as_ref();
    // println!("{:?}", bytes);
    // println!("{:?}", contents);
    // 
    // let mut external_vec: Vec<u8> = create_vector();
    // external_vec.push(5);
    // external_vec.push(6);
    // println!("Vector: {:?}", external_vec);

    Ok(())
}

fn create_vector() -> Vec<u8> {
    let vec: Vec<u8> = vec![1,2,3,4];
    vec
}