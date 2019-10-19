use mtpng;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let time_start = Instant::now();

    let width: usize = 1200;
    let height: usize = 800;

    let mut header = mtpng::Header::new();
    header.set_size(width as u32, height as u32).unwrap();
    header
        .set_color(mtpng::ColorType::TruecolorAlpha, 8)
        .unwrap();
    let options = mtpng::encoder::Options::new();
    let path = Path::new(r"out_image.png");
    let file_writer = BufWriter::new(File::create(path)?);
    let mut encoder = mtpng::encoder::Encoder::new(file_writer, &options);

    let mut data: Vec<u8> = Vec::with_capacity(width * height * 4);
    data.resize(width * height * 4, 0);

    let time_a = Instant::now();

    for i in 0..height {
        for j in 0..width {
            data[4 * (i * width + j)] = ((i as f32) * 255_f32 / height as f32) as u8;
            data[4 * (i * width + j) + 1] = ((j as f32) * 255_f32 / width as f32) as u8;
            data[4 * (i * width + j) + 2] = 128_u8;
            data[4 * (i * width + j) + 3] = 255_u8;
        }
    }
    let time_b = Instant::now();
    //file_data.write_image_data(&*data).unwrap(); // Save
    encoder.write_header(&header).unwrap();
    encoder.write_image_rows(&data).unwrap();
    encoder.finish().unwrap();
    let time_c = Instant::now();
    println!(
        "{:?} {:?} {:?}",
        time_a - time_start,
        time_b - time_a,
        time_c - time_b,
    );

    Ok(())
}
