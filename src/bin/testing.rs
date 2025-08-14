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