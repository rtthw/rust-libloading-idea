


use anyhow::Result;



fn main() -> Result<()> {
    test_same_binary()
}



fn test_same_binary() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let lib_path = current_dir.join("target/release/liblib.so");

    unsafe {
        let lib = libloading::Library::new(lib_path)?;
        println!("FOUND LIB: {:?}", lib);
    }

    Ok(())
}

fn test_multiple_binaries() -> Result<()> {
    todo!()
}
