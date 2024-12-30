use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use rand::Rng;

mod utils;
use utils::text_to_pixel;

fn load_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    reader
    .lines()
    .map(|line| line.unwrap().parse::<String>().unwrap())
    .collect()
}


fn main() {
    let vect_pixels = load_from_file("../list");
    let mut handles = vec![];
    const THREADS: i32= 10;
    for k in 0..THREADS {
        let temp = &vect_pixels[(vect_pixels.len()/THREADS as usize)*(k as usize) ..(vect_pixels.len()/THREADS as usize)*((k+1) as usize)];
        let p = temp.to_vec();
        // let p = vect_pixels.clone();
        let handle = thread::spawn(move || {
                match TcpStream::connect("table.c3pixelflut.de:1337") {
                    Ok(mut stream) => {
                        loop {
                            let mut msg: String = "".to_string();
                            msg += format!("OFFSET 2500 300\n").as_str();

                            for x in 0..p.len() {
                                let split = p[x as usize].split(" ").collect::<Vec<_>>();
                                msg += format!("PX {} {} {}\n", split[1], split[2], split[3]).as_str();
                            }
                            match stream.write(msg.as_bytes()) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Failed to send data: {}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to connect: {}", e);
                    }
                }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|handle| handle.join().unwrap());
}