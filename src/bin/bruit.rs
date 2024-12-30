use std::io::Write;
use std::net::TcpStream;
use std::thread;
use rand::Rng;

fn main() {
    let mut handles = vec![];
    const THREADS: i32= 10;
    for _ in 0..THREADS {
        let handle = thread::spawn(move || {
                match TcpStream::connect("table.c3pixelflut.de:1337") {
                    Ok(mut stream) => {
                        loop {
                            let mut msg: String = "".to_string();
                            msg += format!("OFFSET 975 0\n").as_str();
                            for _ in 0..20{
                                // let r = rand::thread_rng().gen_range(0..255);
                                // let g = rand::thread_rng().gen_range(0..255);
                                // let b = rand::thread_rng().gen_range(0..255);
                                for _ in 0..1000 {
                                    let x = rand::thread_rng().gen_range(0..500);
                                    let y = rand::thread_rng().gen_range(0..400);
                                    for i in 0..8{
                                        for k in 0..8{
                                            msg += format!("PX {} {} {:02x}{:02x}{:02x}\n", x+i, y+k, 255, 255, 255).as_str();
                                        }
                                    }
                                }
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