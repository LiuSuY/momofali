use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(0.1,0.2);
    let r = a + b;
    println!("{} + {}i", r.re, r.im);
}
