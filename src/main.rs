use qrcode::render::unicode;
use qrcode::QrCode;
use std::io;
fn main() {
    println!("Enter the text to generate QR code:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    println!("Do you want to generate a QR code for the following input? (y/n)");

    let mut confirmation = String::new();
    io::stdin()
        .read_line(&mut confirmation)
        .expect("Failed to read confirmation line");

    if confirmation.trim().to_lowercase() == "y" {
        let code = QrCode::new(input.trim()).unwrap();
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();

        println!("{}", image);
    } else {
        println!("QR code generation canceled.");
    }
}
