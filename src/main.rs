extern crate image;
extern crate num;
use num::complex::Complex;

fn main() {
    let xa: f64 = -2.0;
    let xb: f64 = 1.0;
    let ya: f64 = -1.5;
    let yb: f64 = 1.5;

    let maxit: i32 = 255;

    let imgx: u32 = 4096;
    let imgy: u32 = 4096;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let mut z: Complex<f64>;
    let mut c: Complex<f64>;

    for y in 0..imgy {
        let  zy = y as f64 * (yb - ya) / (imgy - 1) as f64 + ya;
        for x in 0..imgx {
            let  zx = x as f64 * (xb - xa) / ((imgx - 1) as f64) + xa;
            z = Complex::new(zx, zy) * Complex::from(1.0);
            c = z;
            for i in 0..maxit {
                if z.norm() > 2.0 {
                    break;
                } else {
                    z = z * z + c;
                }
                let pix = image::Rgb([
                    ((i % 4) * 64)as u8,
                    ((i % 8) * 32)as u8,
                    ((i % 16) * 16)as u8,
                ]);
                imgbuf.put_pixel(x, y, pix);
            }
        }
    }
    imgbuf.save("fractal.png").unwrap();
}
