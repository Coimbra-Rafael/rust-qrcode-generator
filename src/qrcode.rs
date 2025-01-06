use qrcode::render::unicode;
use qrcode::QrCode;

pub fn generate_qrcode(input: &str) {
    let code = QrCode::new(input.trim()).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    println!("{}", image);
}