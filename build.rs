use glob::glob;
use protobuf_codegen::{Codegen, Customize};
use std::{result::Result, path::PathBuf};

fn main() -> Result<(), String> {
    protobuf_codegen()?;
    Ok(())
}

fn protobuf_codegen() -> Result<(), String> {
    let inputs: Vec<PathBuf> = glob("sarus_data_spec/protobuf/*.proto")?
        .filter_map(|pathbuf| pathbuf.ok())
        .collect();
    Codegen::new()
        .pure()
        .out_dir("src/protobuf")
        .inputs(inputs)
        .include("")
        .customize(Customize::default()
            .generate_accessors(true)
            .gen_mod_rs(false)
        )
        .run().expect("Generated proto");
    Ok(())
}
