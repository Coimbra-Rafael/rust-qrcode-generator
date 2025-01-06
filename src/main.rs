use std::io;
mod qrcode;

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
     qrcode::generate_qrcode(input);
    } else {
        println!("QR code generation canceled.");
    }
}
