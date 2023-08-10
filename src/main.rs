pub mod ray;

use nalgebra::Vector3;

type Color = Vector3<f32>;


// 使用 `cargo run > image.ppm`
fn main() {
    let img_width = 256;
    let img_height = 256;

    println!("P3\n{} {}\n255\n", img_width, img_height);
    for j in (0..img_height).rev() {
        // 这个 \r 可以清空当前一行
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..img_width {

            let r = i as f32 / (img_width - 1) as f32;
            let g = j as f32 / (img_height - 1) as f32;
            let b = 0.25f32;

            let color = Color::new(r, g, b);
            write_color(color);
        }
    }
    // clear
    eprint!("\r");                     
}


fn write_color(color: Color) {
    let ir = (255.99 * color.x) as i32;
    let ig = (255.99 * color.y) as i32;
    let ib = (255.99 * color.z) as i32;

    println!("{} {} {}", ir, ig, ib);
}
