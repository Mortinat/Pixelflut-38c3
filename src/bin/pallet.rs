use std::io::Write;
use std::net::TcpStream;
use std::thread;
use rand::Rng;

#[derive(Clone, Copy)]
struct Pallet {
    x: i32,
    y: i32,
    r: i32,
    g: i32,
    b: i32,
}

fn main() {
    let mut handles = vec![];
    const THREADS: i32= 10;
    for _ in 0..THREADS {
        let handle = thread::spawn(move || {
                match TcpStream::connect("table.c3pixelflut.de:1337") {
                    Ok(mut stream) => {
                        let mut tab: [Pallet; 50] = [Pallet{x: 0, y: 0, r: 0, g:0, b:0}; 50];
                        for index in 0..50{
                            let x = rand::thread_rng().gen_range(0..3840);
                            let y = rand::thread_rng().gen_range(0..1080);
                            let r = rand::thread_rng().gen_range(0..255);
                            let g = rand::thread_rng().gen_range(0..255);
                            let b = rand::thread_rng().gen_range(0..255);
                            tab[index] = Pallet{x: x, y: y, r: r, g: g, b: b};
                        }
                        loop {
                            let mut msg: String = "".to_string();
                            for index in 0..50{
                                msg += format!("OFFSET {} {}\n", tab[index].x, tab[index].y).as_str();
                                tab[index].x += rand::thread_rng().gen_range(0..25);
                                tab[index].x = tab[index].x % 3840;
                                tab[index].y += rand::thread_rng().gen_range(-25..25);
                                tab[index].y = tab[index].y % 1080;
                                for i in 0..50{
                                    for k in 0..50{
                                        if i > 45 || k > 45 || i < 5 || k < 5 {
                                            msg += format!("PX {} {} {:02x}{:02x}{:02x}\n", tab[index].x+i, tab[index].y+k, 0, 0, 0).as_str();
                                        } else {
                                        msg += format!("PX {} {} {:02x}{:02x}{:02x}\n", tab[index].x+i, tab[index].y+k, tab[index].r, tab[index].g, tab[index].b).as_str();
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