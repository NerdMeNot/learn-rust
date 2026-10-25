fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Turn proto/greeter.proto into Rust code at build time.
    tonic_prost_build::compile_protos("proto/greeter.proto")?;
    Ok(())
}
