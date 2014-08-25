extern crate tar;
extern crate regex;

use std::{io};
use std::io::{fs, File};
use std::fmt;
use url::Url;

use http::client::RequestWriter;
use http::method::{ Get };
//use http::status;
use http::headers::HeaderEnum;

// use utils;

pub fn init(lesson_name: &str, locale: &str) {
    let lesson_folder = format_args!(fmt::format, "{lesson_name}_{locale}_lesson", lesson_name=lesson_name, locale=locale);

    let dir = &Path::new(lesson_folder);

    if dir.is_dir() {
        println!("Directory already exists!");
    } else {
        init_lesson_skeleton(dir);
    }
}

pub fn login(hexlet_api_key: &str) {
    println!("Login! {}", hexlet_api_key);
    let url = Url::parse("http://hexlet.io/user/check_auth.json").ok().expect("Invalid URL :-(");
    println!("{}", url);

    let mut request: RequestWriter = RequestWriter::new(Get, url).unwrap();
    request.headers.

    println!("Request");
    println!("=======");
    println!("");
    println!("[URL: {}", request.url);
    println!("[Remote address: {}", request.remote_addr);
    println!("[Method: {}", request.method);
    println!("[Headers:");
    for header in request.headers.iter() {
        println!(" - {}: {}", header.header_name(), header.header_value());
    }

    println!("");
    println!("[Response");
    println!("[========");
    println!("");
    let mut response = match request.read_response() {
        Ok(response) => response,
        Err(_request) => fail!("This example can progress no further with no response :-("),
    };
    println!("");
    println!("[Status: {}", response.status);
    println!("[Headers:");
    for header in response.headers.iter() {
        println!(" - {}: {}", header.header_name(), header.header_value());
    }
    println!("");
    println!("[Body:");
    let _ = match response.read_to_end() {
        Ok(body) => body,
        Err(err) => fail!("Reading response failed: {}", err),
    };

}

pub fn submit(path: &str) {
    println!("Submit! {}", path);
    let _archive = generate_lesson_tarball(path);
}

fn init_lesson_skeleton(dir: &Path) {
    if !dir.is_dir() {
        let tarball_path = "data/skeletons/lesson.tar";
        let _lesson_skeleton = &Path::new(tarball_path);
        let _ = fs::mkdir(dir, io::UserRWX);

        /*
         * Here we need extract archive or copy directory
         */
    }
}

fn generate_lesson_tarball(path: &str) -> Path {
    let tarball_directory = format_args!(fmt::format, "{}/dist", path);
    let tarball_filename = format_args!(fmt::format, "{}/lesson.tar", tarball_directory);

    let dir = &Path::new(tarball_directory);

    if dir.is_dir() {
        let _ = fs::rmdir_recursive(dir);
    }

    let _ = fs::mkdir(dir, io::UserRWX);

    let archive_file = File::create(&Path::new(tarball_filename.clone())).unwrap();
    let a = tar::Archive::new(archive_file);

    let exclude_files = [
        r"^.*dist.*$",
        r".+\.pyc$"
            ];

    let lesson_directory = &Path::new(path);

    match fs::walk_dir(&Path::new(lesson_directory)) {
        Err(why) => println!("! {}", why.kind),
        Ok(mut paths) => for file in paths {
            println!("> {}", file.display());

            let mut fm = false;

            for rule in exclude_files.iter() {
                let re = match regex::Regex::new(rule.as_slice()) {
                    Ok(re) => re,
                    Err(err) => fail!("{}", err),
                };

                fm = fm || re.is_match(file.as_str().unwrap())
            }
            if !fm {
                println!(">> {}", file.display());
                let _ = a.append(file.as_str().unwrap(), &mut File::open(&file).unwrap());
                println!("");
            }
        },
    }

    let _ = a.finish();

    return Path::new(tarball_filename)
}
