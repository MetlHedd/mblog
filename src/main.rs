#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::response::NamedFile;
use rocket_contrib::json::{JsonValue};
use std::path::{Path, PathBuf};
use std::fs::{File, read_dir};
use std::io::prelude::*;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("client/dist/index.html")).ok()
}

#[allow(unused_variables)]
#[get("/post/<name>")]
fn post(name: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("client/dist/index.html")).ok()
}

#[allow(unused_variables)]
#[get("/page/<name>")]
fn page(name: String) -> Option<NamedFile> {
    NamedFile::open(Path::new("client/dist/index.html")).ok()
}

#[get("/api/post/<name>")]
fn api_post(name: String) -> JsonValue {
    /* Check if the post exists */
    if Path::new(&format!("blog/posts/{}", name)).exists() {
        let mut post_content = String::new();

        match File::open(format!("blog/posts/{}/content.md", name)) {
            Err(why) => return json!({
                "status": "error",
                "content": format!("Could not open /blog/posts/{}/conten.md, details: {}", name, why.to_string())
            }),
            
            Ok(mut file) => {
                match file.read_to_string(&mut post_content) {
                    Err(why) => return json!({
                        "status": "error",
                        "content": format!("Could read the file /blog/posts/{}/conten.md as string, details: {}", name, why.to_string())
                    }),
                    Ok(_) => {}
                }
            }
        };

        let have_image = Path::new(&format!("blog/posts/{}/image.jpg", name)).exists();

        json!({
            "status": "ok",
            "title": name,
            "content": post_content,
            "haveImage": have_image
        })
    } else {
        json!({
            "status": "error",
            "content": format!("Could not open /blog/posts/{}/", name)
        })
    }
}

#[get("/api/post/image/<name>")]
fn api_post_image(name: String) -> Option<NamedFile> {
    NamedFile::open(Path::new(&format!("blog/posts/{}/image.jpg", name))).ok()
}

#[get("/api/posts")]
fn all_posts() -> JsonValue {
    let files = read_dir(Path::new("blog/posts/")).unwrap().filter_map(|entry| {
        entry.ok().and_then(|e| {
            e.path().file_name().and_then(|n| n.to_str().map(|s| String::from(s)))
        })
    }).collect::<Vec<String>>();;

    json!({
        "files": files
    })
}

#[get("/api/page/<name>")]
fn api_page(name: String) -> JsonValue {
    if Path::new(&format!("blog/pages/{}.md", name)).exists() {
        let mut page_content = String::new();

        match File::open(format!("blog/pages/{}.md", name)) {
            Err(why) => return json!({
                "status": "error",
                "content": format!("Could not open /blog/pages/{}.md, details: {}", name, why.to_string())
            }),
            
            Ok(mut file) => {
                match file.read_to_string(&mut page_content) {
                    Err(why) => return json!({
                        "status": "error",
                        "content": format!("Could read the file /blog/pages/{}.md as string, details: {}", name, why.to_string())
                    }),
                    Ok(_) => {}
                }
            }
        };


        json!({
            "status": "ok",
            "title": name,
            "content": page_content
        })
    } else {
        json!({
            "status": "error",
            "content": format!("Could not open /blog/pages/{}.md", name)
        })
    }
}

#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("client/dist/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, post, all_posts, page, api_post, api_post_image, api_page, files]).launch();
}