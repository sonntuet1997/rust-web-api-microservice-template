use glob::glob;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Find all .proto files within the proto/ directory
    let proto_files: Vec<String> = glob("proto/*.proto")?
        .filter_map(|entry| {
            entry
                .ok()
                .and_then(|path| path.into_os_string().into_string().ok())
        })
        .collect();

    // Compile the found .proto files
    proto_files.iter().for_each(|proto_file| {
        println!("cargo:rerun-if-changed={}", proto_file);
        tonic_build::compile_protos(proto_file).unwrap();
    });

    Ok(())
}
