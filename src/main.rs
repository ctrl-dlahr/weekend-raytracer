use std::fs::File;
use std::io::{BufWriter, self, Write};

fn main() {
    // Image
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    
    let file = File::create("img/image.ppm").expect("unable to create image.ppm");
    let mut writer = BufWriter::new(&file);
    // Render
    write!(&mut writer, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).expect("unable to write to file.");
    
    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining {}", j);
        io::stdout().flush().unwrap();
        for i in 0..IMAGE_WIDTH {

            let r = (i as f64) / ((IMAGE_WIDTH-1) as f64);
            let g = (j as f64) / ((IMAGE_HEIGHT-1) as f64);
            let b = 0.25;
            
            let ir: i32 = (255.999 * r as f32) as i32;
            let ig: i32 = (255.999 * g as f32) as i32;
            let ib: i32 = (255.999 * b as f32) as i32;
            
            write!(&mut writer, "{} {} {}\n", ir, ig, ib).expect("unable to write to file.");
        }
    }
}
