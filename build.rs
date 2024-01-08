use anyhow::Result;
use glob::glob;
use protobuf_codegen::{Codegen, Customize};
use std::path::PathBuf;

fn main() -> Result<()> {
    // Does not fail if build did not succeed
    protobuf_codegen().unwrap_or_else(|err| println!("Silent error: {err}"));
    Ok(())
}

fn protobuf_codegen() -> Result<()> {
    let inputs: Vec<PathBuf> = glob("sarus_data_spec/protobuf/*.proto")?
        .filter_map(|pathbuf| pathbuf.ok())
        .collect();
    Codegen::new()
        .pure()
        .out_dir("src/protobuf")
        .inputs(inputs)
        .include("")
        .customize(
            Customize::default()
                .generate_accessors(true)
                .gen_mod_rs(false),
        )
        .run()?;
    Ok(())
}
