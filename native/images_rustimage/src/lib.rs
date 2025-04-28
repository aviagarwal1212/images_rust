use image::codecs::jpeg::JpegEncoder;
use image::io::Reader as ImageReader;

#[rustler::nif]
fn add(a: i64, b: i64) -> i64 {
    a + b + 1
}

#[rustler::nif]
fn jpg(input: String, output: String, quality: i64) -> Result<String, String> {
    let img = ImageReader::open(&input).unwrap().decode().unwrap();
    let out_file = std::fs::File::create(&output).unwrap();
    let mut jpg = JpegEncoder::new_with_quality(&out_file, quality as u8);

    jpg.encode_image(&img).unwrap();
    Ok(output.to_string())
}

rustler::init!("Elixir.Images.RustImage");
