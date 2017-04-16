extern crate image;
extern crate chrono;
extern crate walkdir;

use std::env;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use image::GenericImage;
use chrono::prelude::*;
use walkdir::WalkDir;

// TODO: doc comments
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        println!("importing files from path {:?}", &args[1]);

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
                }
            } else if width == "3200px" {
                order += 1;

                let dimensions = image::open(&entry.path()).unwrap().dimensions();
                let date: DateTime<UTC> = UTC::now();
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

                save_entry(&category, &title, &data, &args[2]);

                println!("created entry {:?}", &title);
            }
        }

        // TODO: error handling?

        println!("DONE");
    } else {
        help();
    }
}

fn help() {
    println!("usage: uploader <path to files> <path to output>");
}

fn save_entry(category: &str, title: &str, data: &str, base_path: &str) {
    let category_dir = format!("{}/{}", &base_path, &category);

    // create category dir if it doesn't exist already
    match fs::metadata(&category_dir) {
        Err(msg) => {
            // TODO: handle this!
            fs::create_dir(&category_dir);
        }
        Ok(metadata) => {
            if !metadata.is_dir() {
                panic!("unable to create dir {}", &category_dir);
            }
        }
    }

    let filename = format!("./photos/{}/{}.md", &category, &title);
    let path = Path::new(&filename);
    let display = path.display();

    // TODO: don't create file if exists already - (in future update!)

    let mut file = match fs::File::create(&path) {
        Err(msg) => panic!("unable to create file {}: {}", display, msg.description()),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(msg) => panic!("unable to write to file {}: {}", display, msg.description()),
        Ok(file) => file,
    }

    // TODO: return value
}
