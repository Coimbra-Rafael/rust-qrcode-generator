use std::io::stdin;
mod qrcode;

fn main() {
    println!("Enter the text to generate QR code:");

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    println!("Do you want to generate a QR code for the following input? (y/n)");

    let mut confirmation = String::new();
    stdin()
        .read_line(&mut confirmation)
        .expect("Failed to read confirmation line");

    if confirmation.trim().to_lowercase() == "y" {
        qrcode::generete_qrcode_file(qrcode::generate_qrcode(input).as_bytes());
    } else {
        println!("QR code generation canceled.");
    }
}
