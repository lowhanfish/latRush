// Type Inference vs Manifest Typing
// pada type inference kita mendefinisikan variabel tanpa type data : e.g let datax = 5
// pada Manifest Typing kita secara eksplisit mendefinisikan tipe datanya : e.g let number:i8 = 5

use std::i8;

fn main() {
    // let (data1, data2, data3): (i8, i8, &str) = (12, 21, "Riswan");
    // println!("{},{},{}", data1, data2, data3);
    // let mince = i8::MIN;
    // let maxce = i8::MAX;
    // println!("Nilai Max = {} dan Nilai MIn = {}", mince, maxce);

    // let des: f32 = 3.1231312321;
    // println!("Nilai des {:.2}", des);

    // let bolex: bool = true;
    // println!("data boolean : {}", bolex);

    // // escape (Outputnya bersambung)
    // let escap: &str = " hello \
    //     \"ini\" \
    //     escape \
    // ya";
    // println!("{}", escap);

    // // (OUTPUNYA newline)
    // let escap1 = "
    //     Baris 1
    //     Baris 2
    //     Baris 3
    // ";
    // println!("{}", escap1);

    // let ecap3: &str = "
    // [
    //     {
    //     \"nama\" : \"Koko\"
    //     }
    // ]
    // ";
    // println!("{}", ecap3);

    hapus(13);
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

fn hapus(data: i16) {
    println!("jumlah perkalian = {}", data * 4);
}
