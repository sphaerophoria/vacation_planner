use std::env;
use std::path::PathBuf;

fn main() -> Result<(), ()>
{
    let dst = cmake::Config::new("gui")
                    .build();
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=dylib=vacation_planner_gui");

    let bindings = bindgen::builder().header("gui/gui.h")
        .generate()?;


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
