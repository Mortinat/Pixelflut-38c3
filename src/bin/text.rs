use std::io::Write;
use std::net::TcpStream;
use std::thread;
use client_pixelflut::utils::text_to_pixel;
use std::time;

fn main() {
    let ((width, _height), vect_pixels) = text_to_pixel("8C3 3", 128);
    let mut handles = vec![];
    let start = time::Instant::now();
    const THREADS: i32= 10;
    for k in 0..THREADS {
        let temp = &vect_pixels[(vect_pixels.len()/THREADS as usize)*(k as usize) ..(vect_pixels.len()/THREADS as usize)*((k+1) as usize)];
        let p = temp.to_vec();
        let start_t = start.clone();
        // let p = vect_pixels.clone();
        let handle = thread::spawn(move || {
                match TcpStream::connect("table.c3pixelflut.de:1337") {
                    Ok(mut stream) => {
                        loop {
                            let mut msg: String = "".to_string();
                            msg += format!("OFFSET 2500 300\n").as_str();
                            let elapsed = start_t.elapsed().as_millis();

                            for x in 0..p.len() {
                                let mut r = p[x as usize].2;
                                let mut g = p[x as usize].3;
                                let mut b = p[x as usize].4;
                                if r == 0 && g == 0 && b == 0 {
                                    r = 255;
                                    g = 255;
                                    b = 255;
                                }
                                msg += format!("PX {} {} {:02x}{:02x}{:02x}\n", ((p[x as usize].0)+width-((elapsed/10) as u32)%width)%width , p[x as usize].1, r, g, b).as_str();
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