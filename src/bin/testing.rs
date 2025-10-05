use std::fs::File;
use std::io::{self, BufRead, BufReader};
use glam::{Vec2, Vec3};

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

// #[repr(C)]
struct Vertex {
    position: Vec3,     // 12 bytes
    normal: Vec3,       // 12 bytes
    tex_coords: Vec2    // 8 bytes
}

#[repr(C)]
struct VertexC {
    position: Vec3,     // 12 bytes
    normal: Vec3,       // 12 bytes
    tex_coords: Vec2    // 8 bytes
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

    let offset_field1 = core::mem::offset_of!(Vertex, position);
    let offset_field2 = core::mem::offset_of!(Vertex, normal);
    let offset_field3 = core::mem::offset_of!(Vertex, tex_coords);

    println!("Offset of field1: {}", offset_field1);
    println!("Offset of field2: {}", offset_field2);
    println!("Offset of field3: {}", offset_field3);

    let vertex_size: usize = size_of::<Vertex>();
    println!("Size of vertex struct: {}", vertex_size);

    let offset_field1 = core::mem::offset_of!(VertexC, position);
    let offset_field2 = core::mem::offset_of!(VertexC, normal);
    let offset_field3 = core::mem::offset_of!(VertexC, tex_coords);

    println!("Offset of field1: {}", offset_field1);
    println!("Offset of field2: {}", offset_field2);
    println!("Offset of field3: {}", offset_field3);

    let vertex_size: usize = size_of::<VertexC>();
    println!("Size of vertexC struct: {}", vertex_size);

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let mut doggy = "ಠ_ಠ";
    doggy = "pudlly";
    println!("{doggy}");


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