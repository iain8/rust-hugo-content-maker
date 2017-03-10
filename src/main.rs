extern crate image;
extern crate chrono;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use image::GenericImage;
use chrono::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() == 3 {
    println!("importing file {:?}", &args[1]);

    // TODO: error handling?
    let dimensions = image::open(&Path::new(&args[1])).unwrap().dimensions();

    let date: DateTime<UTC> = UTC::now();

    // TODO: are the \n chars necessary?
    let data = format!(
      "+++
title: \"{}\"
image: \"{}\"
width: {}
height: {}
date: {}
draft: false
+++\n",
      &args[2],
      &args[1],
      &dimensions.0,
      &dimensions.1,
      &date
    );

    let path = Path::new("../test.md");
    let display = path.display();

    let mut file = match File::create(&path) {
      Err(msg) => panic!("unable to create file {}: {}", display, msg.description()),
      Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
      Err(msg) => panic!("unable to write to file {}: {}", display, msg.description()),
      Ok(file) => file,
    }

    println!("DONE");
  } else {
    help();
  }
}

fn help() {
  println!("usage: uploader <path to file> <title>");
}