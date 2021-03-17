use std::io::Error;

fn main() -> Result<(),Error> {
    prost_build::compile_protos(&["src/items.proto"], &["src/"])?;
    prost_build::compile_protos(&["src/config.proto"], &["src/"])?;
    Ok(())
}