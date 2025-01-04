use qrcode::QrCode;
use qrcode::render::unicode;
use std::io;
fn main() {
    println!("Enter the text to generate QR code:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let code = QrCode::new(input.trim()).unwrap();
    let image = code.render::<unicode::Dense1x2>().build();
    println!("{}", image);
}
