use std::fs::{File, read_to_string};
use tera::{Tera, Context};

// TODO: this should probably use a template
fn main () {
    println!("cargo:rerun-if-changed=techniques");
    let cont = read_to_string("techniques/techniques").unwrap();
    let techs: Vec<&str> = cont.lines().map(|l| l.trim()).collect();
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