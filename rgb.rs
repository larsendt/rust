use std::vec;
use std::io::{File};

struct PPMImg {
    width: uint,
    height: uint,
    data: ~[u8]
}

fn set_pixel(img: & mut PPMImg, x: uint, y: uint, r: u8, g: u8, b:u8) {
    let loc = ((y * img.width) + x) * 3;
    img.data[loc] = r;
    img.data[loc+1] = g;
    img.data[loc+2] = b;
}

fn get_pixel(img: & PPMImg, x: uint, y: uint) -> (u8, u8, u8) {
    let loc = ((y * img.width) + x) * 3;
    (img.data[loc], img.data[loc+1], img.data[loc+2])
}

fn make_base_img(width: uint, height: uint) -> ~PPMImg {
    let sz = width * height * 3;
    let data = vec::from_elem(sz, 0u8);
    let img = ~PPMImg { width: width, height: height, data: data };
    return img;
}

fn write_img(img: ~PPMImg) {
    let path = Path::new("./bitmap.ppm");
    let mut file = File::create(&path);
    let header = format!("P6 {} {} 255\n", img.width, img.height);
    file.write(header.as_bytes());
    file.write(img.data);
}

fn set_rgb_colors(img: & mut PPMImg) {
    for x in range(0, img.width) {
        for y in range(0, img.height) {
            let loc = ((y * img.width) + x);
            let r: u8 = ((loc >> 14) & 127) as u8;
            let g: u8 = ((loc >> 7) & 127) as u8;
            let b: u8 = (loc & 127) as u8;
            set_pixel(img, x, y, r, g, b);
        }
    }
}

fn main() {
    let mut img = make_base_img(2048, 1024);
    set_rgb_colors(img);
    write_img(img);
    println!("RGB generator");
}
