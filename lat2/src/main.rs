use std::i8;

fn main() {
    let (data1, data2, data3): (i8, i8, &str) = (12, 21, "Riswan");

    println!("{},{},{}", data1, data2, data3);

    let mince = i8::MIN;
    let maxce = i8::MAX;

    print!("Nilai Max = {} dan Nilai MIn = {}", mince, maxce);
}

// fn main() {
//     println!("Hello, world!");
//     coba();
// }

// fn coba() {
//     println!("hy nama saya kiken");
//     println!("Ini dari fungsi lain ya...!!");
//     fungsi_dengan_argumen("Kiken");
//     fungsi_dengan_argumen("Lolo");
// }

// fn fungsi_dengan_argumen(nama: &str) {
//     println!(
//         "Ini pemanggilan dari fungsi yang berargumen, dipanggil oleh {}",
//         nama
//     );
// }
