use std::io::Write;
use std::net::TcpStream;
use std::thread;
use client_pixelflut::utils::gif_to_pixel;
use std::time;
use rand::Rng;

fn main() {
    let vect_pixels = gif_to_pixel("../test");
    
    let mut handles = vec![];
    let start = time::Instant::now();
    const THREADS: u32= 10;
    for k in 0..THREADS {
        let n_frames = vect_pixels.len();
        let p = vect_pixels.to_vec();
        let start_t = start.clone();
        let size_tread = (p[0].len()/THREADS as usize) as u32;
        // let p = vect_pixels.clone();
        let handle = thread::spawn(move || {
                match TcpStream::connect("table.c3pixelflut.de:1337") {
                    Ok(mut stream) => {
                        loop {
                            let mut msg: String = "".to_string();
                            msg += format!("OFFSET 2700 100\n").as_str();
                            let elapsed = start_t.elapsed().as_millis();
                            let frame_index = ((elapsed/10) as u32) % n_frames as u32;
                            
                            for _ in (size_tread*k)..(size_tread*(k+1)) {
                                let x = rand::thread_rng().gen_range((size_tread*k)..(size_tread*(k+1)));
                                msg += format!("PX {} {} {:02x}{:02x}{:02x}\n",
                                p[frame_index as usize][x as usize].0,
                                p[frame_index as usize][x as usize].1,
                                p[frame_index as usize][x as usize].2,
                                p[frame_index as usize][x as usize].3,
                                p[frame_index as usize][x as usize].4).as_str();
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