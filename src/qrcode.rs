use qrcode::render::unicode;
use qrcode::QrCode;
use std::{io::Write, u8};

pub fn generate_qrcode(input: &str) -> String {
    let code = QrCode::new(input.trim()).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    println!("{}", image);
    image
}

pub fn generete_qrcode_file(qrcode_bytes: &[u8]) {
    let file = std::fs::File::create("qrcode.txt");
    file.unwrap().write_all(qrcode_bytes).unwrap();
}
