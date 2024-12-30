use image::ImageReader;
use text_to_png::TextRenderer;
use std::io::Write;


pub fn get_pixel(path: &str) -> ((u32, u32), Vec<(u32, u32, u8, u8, u8)>) {
    // let renderer = TextRenderer::default();

    // let text_png = renderer.render_text_to_png_data(
    //     "Rénder this, brö",
    //     64,
    //     "Dark Turquoise");
    let mut command = vec![];
    let mut x = 0;
    let mut y = 0;
    match ImageReader::open(path) {
        Ok(img) => {
            let img = img.decode().unwrap();
            x = img.width();
            y = img.height();
            let tmp = img.to_rgb8();
            for i in 0..x {
                for j in 0..y {
                    let pixel = tmp.get_pixel(i, j);
                    // if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
                    //     continue;
                    // }
                    // if pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255 {
                    //     continue;
                    // }
                    command.push((i, j, pixel[0], pixel[1], pixel[2]));
                }
            }
        }
        Err(e) => {
            println!("Failed to open image: {}", e);
        }
    }
    return ((x, y), command);
}

pub fn text_to_pixel(text: &str, size: u8) -> ((u32, u32), Vec<(u32, u32, u8, u8, u8)>) {
    let renderer = TextRenderer::default();
    let text_png = renderer.render_text_to_png_data(
        text,
        size,
        "red").unwrap();
    Write::write_all(&mut std::fs::File::create("../text.png").unwrap(), &text_png.data).unwrap();

    
    let command = get_pixel("../text.png");
    // let mut command = vec![];
    // let img = ImageReader::new(std::io::Cursor::new(text_png.data));
    // let img = img.decode().unwrap();
    // let x = img.width();
    // let y = img.height();
    // let tmp = img.to_rgb8();
    // for i in 0..x {
    //     for j in 0..y {
    //         let pixel = tmp.get_pixel(i, j);
    //         command.push(format!("PX {} {} {:02x}{:02x}{:02x}\n", i, j, pixel[0], pixel[1], pixel[2]));
    //     }
    // }
    return command;
}


pub fn gif_to_pixel(path: &str) -> Vec<Vec<(u32, u32, u8, u8, u8)>> {
    let mut command = vec![];
    let mut paths: Vec<_> = std::fs::read_dir(path).unwrap()
                                              .map(|r| r.unwrap())
                                              .collect();
    paths.sort_by_key(|dir| dir.path());
    for path in paths {
        let ((_, _), res) = get_pixel(path.path().to_str().unwrap());
        command.push(res);
    }
    return command;
}

pub fn test() {
    let mut command = vec![];
    let mut x = 0;
    let mut y = 0;
    match ImageReader::open("../200w.gif") {
        Ok(img) => {
            let img = img.decode().unwrap();
            x = img.width();
            y = img.height();
            let tmp = img.to_rgb8();
            for i in 0..x {
                for j in 0..y {
                    let pixel = tmp.get_pixel(i, j);
                    // if pixel[0] == 0 && pixel[1] == 0 && pixel[2] == 0 {
                    //     continue;
                    // }
                    // if pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255 {
                    //     continue;
                    // }
                    command.push((i, j, pixel[0], pixel[1], pixel[2]));
                }
            }
        }
        Err(e) => {
            println!("Failed to open image: {}", e);
        }
    }
}