extern crate image;

use colorsys::{Rgb};
use image::GenericImageView;
use std::path::Path;
use std::io::Write;

gflags::define! {
    -f,--input = false
}
gflags::define! {
    -o,--output = false
}

fn main() {
    let args = gflags::parse();
    
    println!("\n\tWelcome to SIF Encoder!");

    if args.len() > 1 {
        println!("\n\tInput file: {:?}", args[0]);
        println!("\tOutput file: {:?}", args[1]);
        let data = encoder(args[0].to_string());
        let imgdata = &data[0];
        let exif = &data[1];
        //println!("\n\tImage data: {}", imgdata[..27].to_string()+"...");
        let img = "{".to_string()+&imgdata+&"}".to_string();
        let exifdata = &exif;
        save_image(args[1].to_string(), exifdata.to_string()+&img);
    } else if args.len() == 1 {
        println!("\n\tNumber of arguments not satisfied.\n\tplease provide both input and output file names.");
    } else {
        help();
    }
}

pub fn help() {
    println!("\n\tUsage: sif-encoder [options] <input file> <output file>");
    println!("\n\tOptions:");
    println!("\t--input,-f:   Input file name");
    println!("\t--output,-o:  Output file name");
}
pub fn gen_exif(opts: Vec<i32>) -> String {return format!("{{\"width\": {},\"height\": {}}}", opts[0], opts[1]);}
pub fn encoder(input: String) -> Vec<String> {
    let imgpath = Path::new(&input[..]);
    print!("\tReading image file: {}\n", imgpath.to_str().unwrap());
    let img = image::open(imgpath).unwrap();
    let (width, height) = img.dimensions();
    let mut img_data: String= String::new();
    
    let (mut row, mut col) = (0, 0);
    while row < height {
        let mut curc: String = "".to_string();
        let mut curcount = 1;
        let mut this_row: String = "[".to_string();
        while col < width {
            let pixel = img.get_pixel(col, row);
            let red = pixel[0] as f32;
            let green = pixel[1] as f32;
            let blue = pixel[2] as f32;
            let alpha = pixel[3] as f32;
            let colo = Rgb::from((red, green, blue, alpha/255.0)).to_hex_string()[1..].to_string();
            //copy thr colo variable to the colou vaiable
            let colou = colo;
            let mut pixel_data: String = String::new();
            if curc == "" {
                curc = colou;
            } else if curc == colou {
                curcount += 1;
            } else {
                pixel_data = curcount.to_string()+"-".to_string().as_str()+curc.to_string().as_str()+",";
                curc = colou;
                curcount = 1;
            }
            if col == width-1 {
                pixel_data = curcount.to_string()+"-".to_string().as_str()+curc.to_string().as_str();
            }
            let new_row = this_row.to_string()+pixel_data.to_string().as_str();
            this_row = new_row;
            col += 1;
        }
        row += 1;
        col = 0;
        img_data = img_data.to_string()+&this_row.to_string()+&"]".to_string();
    }
    let mut meta: Vec<i32> = Vec::new();
    meta.push(width as i32);
    meta.push(height as i32);
    let exif = gen_exif(meta);
    let mut data: Vec<String> = Vec::new();
    data.push(img_data);
    data.push(exif);
    return data;
}
/*
fn duplicate<T>(x: T) -> T { x }
*/
fn save_image(output: String, data: String) {
    let file = Path::new(&output[..]);
    let mut f = std::fs::File::create(file).unwrap();
    f.write_all(data.as_bytes()).unwrap();
}
