extern crate image;

use colorsys::{Rgb};
use image::GenericImageView;
use std::path::Path;

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
        encoder(args[0].to_string(), args[1].to_string());
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

pub fn encoder(input: String, _output: String) {
    let imgpath = Path::new(&input[..]);
    let img = image::open(imgpath).unwrap();
    let (width, height) = img.dimensions();
    let mut img_data: String= String::new();
    
    let (mut row, mut col) = (0, 0);
    while row < height {
        let mut curc = "";
        let mut curcount = 1;
        let mut this_row: String = "[".to_string();
        while col < width {
            let pixel = img.get_pixel(col, row);
            let red = pixel[0] as f32;
            let green = pixel[1] as f32;
            let blue = pixel[2] as f32;
            let alpha = pixel[3] as f32;
            let colo;
            colo = Rgb::from((red, green, blue, alpha/255.0)).to_hex_string();
            let colou: &str = &colo[..];
            let mut pixel_data: String = String::new();
            if curc == "" {
                curc = &colou[..];
            } else if curc == colou {
                curcount += 1;
            } else {
                curc = &colou[..];
                curcount = 1;
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
    println!("\n\tImage data: {}", img_data);
    //println!("dimensions {:?}", img.get_pixel(0, 0));
}
