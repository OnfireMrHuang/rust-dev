use std::io::Result;
use prost_build;

fn main() -> Result<()> {
    prost_build::compile_protos(&["src/proto/api.proto"], &["src/proto/"])?;
    Ok(())
}