use std::fs::{File, read_to_string};
use tera::{Tera, Context};


fn main () {
    println!("cargo:rerun-if-changed=techniques");
    let cont = read_to_string("techniques/techniques").unwrap();
    let techs: Vec<&str> = cont.lines().filter_map(|l| {
        let trimmed = l.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    }).collect();

    let tera = Tera::new("techniques/*.tera").unwrap();
    let mut structs_file = File::create("src/techniques.rs").unwrap();

    let mut ctx = Context::new();
    ctx.insert("techniques", &techs);
    match tera.render_to("techniques.rs.tera", &ctx, &mut structs_file) {
        Ok(_) => {},
        Err(e) => {
            println!("cargo:warning={}", e);
            std::process::exit(1);
        }
    }
}