#[macro_use]
extern crate lazy_static;
extern crate image;
extern crate chrono;
extern crate regex;
extern crate walkdir;

use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use image::GenericImage;
use chrono::prelude::*;
use walkdir::WalkDir;
use regex::Regex;

/// initialise regex
lazy_static! {
    static ref REPLACE_HEIGHT: Regex = Regex::new(r"height = [0-9]+").unwrap();
    static ref REPLACE_ORDER: Regex = Regex::new(r"order = [0-9]+").unwrap();
    static ref REPLACE_WIDTH: Regex = Regex::new(r"width = [0-9]+").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 4 {
        println!("importing files from path {:?}", &args[1]);

        let max_width = format!("{}px", &args[3]);
        let mut category = String::from("uncategorised");
        let mut width = String::from("0px");
        let mut order = 0;

        for entry in WalkDir::new(&args[1]) {
            let entry = entry.unwrap();
            let file_name = entry.file_name().to_str().unwrap();

            // remove file extension
            let title = file_name.split(".").collect::<Vec<_>>()[0];

            if entry.file_type().is_dir() {
                if file_name.contains("px") {
                    width = file_name.to_owned();
                } else {
                    order = 0;
                    category = file_name.to_owned();

                    create_category_dir(&category, &args[2]);
                }
            } else if width == max_width {
                order += 1;
                
                let image_data = get_image(&entry.path());

                if !image_data.is_none() {
                    let dimensions = image::open(&entry.path()).unwrap().dimensions();
                    let date: DateTime<UTC> = UTC::now();

                    let filename = format!("./photos/{}/{}.md", &category, &title);
                    let path = Path::new(&filename);

                    if path.exists() {
                        update_entry(&path, &order, &dimensions);

                        println!("updated entry {:?}", &title);
                    } else {
                        let data = format!("+++
title = \"{}\"
image = \"{}\"
width = {}
height = {}
date = \"{}\"
draft = false
photos = [\"{}\"]
order = {}
+++\n",
                        &title,
                        &file_name,
                        &dimensions.0,
                        &dimensions.1,
                        &date.to_rfc3339(),
                        &category,
                        &order);

                        save_entry(&path, &data);

                        println!("created entry {:?}", &title);
                    }
                }
            }
        }

        println!("DONE");
    } else {
        help();
    }
}

/// Output command line help
fn help() {
    println!("usage: uploader <path to files> <path to output> <size>");
}

/// Create a category directory (if it doesn't exist already)
fn create_category_dir(category: &str, base_path: &str) {
    let category_dir = format!("{}/{}", &base_path, &category);

    // create category dir if it doesn't exist already
    match fs::metadata(&category_dir) {
        Err(msg) => {
            if msg.description() == "entity not found" {
                println!("creating directory {}", &category);

                // TODO: handle this!
                fs::create_dir(&category_dir);
            }
        }
        Ok(metadata) => {
            if !metadata.is_dir() {
                panic!("unable to create dir {}", &category_dir);
            }
        }
    }
}

/// Get an image object if the file is an image
fn get_image(path: &Path) -> Option<image::DynamicImage> {
    match image::open(&path) {
        Err(msg) => {
            println!("skipping file {} ({})", path.display(), msg.description());

            None
        }
        Ok(img) => {
            Some(img)
        }
    }
}

/// Create a new entry
fn save_entry(path: &Path, data: &str) {
    let display = path.display();

    let mut file = match fs::File::create(&path) {
        Err(msg) => panic!("unable to create file {}: {}", display, msg.description()),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(msg) => panic!("unable to write to file {}: {}", display, msg.description()),
        Ok(file) => file,
    }
}

/// Update an existing entry
fn update_entry(path: &Path, order: &u8, dimensions: &(u32, u32)) {
    let mut file = fs::File::open(&path).unwrap();
    let input: Vec<u8> = Vec::new();

    let content = String::from_utf8(input).unwrap();

    let width = dimensions.0.to_string();
    let height = dimensions.1.to_string();
    
    let data = REPLACE_WIDTH.replace_all(&content, &width[..]);
    let data = REPLACE_HEIGHT.replace_all(&data, &height[..]);
    let data = REPLACE_ORDER.replace_all(&data, &order.to_string()[..]);

    match file.write_all(data.as_bytes()) {
        Err(msg) => panic!("unable to write to file {}: {}", path.display(), msg.description()),
        Ok(file) => file,
    }
}
