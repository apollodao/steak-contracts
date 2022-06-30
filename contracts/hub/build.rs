use std::io::Result;

fn main() -> Result<()> {
    prost_build::compile_protos(
        &["../../proto/osmosis/tokenfactory/v1beta1/tx.proto"],
        &["../../proto/"], // NOTE: must have the slash in the end, i.e. `proto/` not `proto`
    )?;
    Ok(())
}
