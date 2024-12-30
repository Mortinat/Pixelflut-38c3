use std::io::Write;
use std::net::TcpStream;
use std::thread;
use rand::Rng;

fn hue_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}


fn main() {
    let mut handles = vec![];
    const THREADS: i32= 10;
    for i in 0..THREADS {
        let k = i.clone();
        let handle = thread::spawn(move || {
                match TcpStream::connect("wall.c3pixelflut.de:1337") {
                    Ok(mut stream) => {
                        loop {
                            let mut msg: String = "".to_string();
                            msg += "OFFSET 1920 0\n";
                            for _ in (110*k)..(110*(k+1)){
                                let y = rand::thread_rng().gen_range((110*k)..(110*(k+1)));
                                for _ in 0..1920 {
                                    let x = rand::thread_rng().gen_range(0..1920);
                                    let (r, g, b) = hue_to_rgb(((x/4)%360) as f64, 1.0, 0.5);
                                    msg += format!("PX {} {} {:02x}{:02x}{:02x}\n", x, y, r, g, b).as_str();
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