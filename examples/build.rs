use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .type_attribute(".", "#[derive(::prost_arrow::ToArrow)]")
        .compile_protos(&["proto/routeguide/route_guide.proto"], &["proto/"])?;
    Ok(())
}
