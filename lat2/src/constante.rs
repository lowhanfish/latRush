// cargo run --bin constante

// use std::thread::sleep;
// use std::time::Duration;

fn main() {
    refresh_brain("============== REFRESH BRAIN ==============");
    get_object();
    // arrar_ex();
    // continuex();
    // breakx();
    // loopx();
    // timerx();
}

fn refresh_brain(text: &str) {
    println!("{}", text);

    // https://gemini.google.com/app/289373e9a1945207?is_sa=1&is_sa=1&android-min-version=301356232&ios-min-version=322.0&campaign_id=bkws&utm_source=sem&utm_source=google&utm_medium=paid-media&utm_medium=cpc&utm_campaign=bkws&utm_campaign=2024idID_gemfeb&pt=9008&mt=8&ct=p-growth-sem-bkws&gclsrc=aw.ds&gad_source=1&gad_campaignid=20437330464&gbraid=0AAAAApk5Bhniwrx629KzPCu_JNnwKrDYZ&gclid=Cj0KCQiAuvTJBhCwARIsAL6DemhD-K8GNtaNg3gSff2w5BO4_bFqqa3m9RElxsDYAquEViFCX_2gfPoaAtx0EALw_wcB
}

fn get_object() {
    struct Pengguna {
        id: i8,
        nama: String,
        usia: i32,
    }

    let user_data: Vec<Pengguna> = vec![
        Pengguna {
            id: 1,
            nama: String::from("Kiken"),
            usia: 39,
        },
        Pengguna {
            id: 2,
            nama: String::from("Rizwan"),
            usia: 29,
        },
        Pengguna {
            id: 3,
            nama: String::from("dayat"),
            usia: 29,
        },
    ];

    for data in user_data.iter() {
        println!(
            "id : {}, Nama : {}, Usia : {}",
            data.id, data.nama, data.usia
        );
    }

    println!("Panjang user_data = {}", user_data.len());
}

// fn arrar_multiple_ex(text: &str) {
//     println!("{}", text);
//     let a: [[&str; 4]; 2] = [["a", "b", "c", "d"], ["e", "f", "g", "h"]];

//     let mut i = 0;
//     loop {
//         if i >= a.len() {
//             break;
//         }

//         let mut j = 0;
//         loop {
//             println!("Array ke-[{}][{}] = {}", i, j, a[i][j]);
//             j += 1;

//             if j >= a[i].len() {
//                 break;
//             }
//         }

//         i += 1;
//     }
// }

// fn arrar_ex() {
//     let letters = ["a", "b", "c", "d"];
//     let letter = letters[3];
//     println!("data letter pertama adalah : {}", letter);
// }

// fn continuex() {
//     // let mut i = 0;
//     // let max = 15;

//     // 'nama_label: loop {
//     //     i += 1;

//     //     if i % 2 == 1 {
//     //         continue;
//     //     }

//     //     println!("i : {}", i);

//     //     if i >= max {
//     //         break 'nama_label;
//     //     }
//     // }

//     let mut i: i16 = 0;
//     'nama_label: loop {
//         i += 1;
//         sleep(Duration::from_secs(1));
//         println!("Ini adalah angka ke {}", i);

//         if i == 5 {
//             println!("Semua data telah selesai di print");
//             break 'nama_label;
//         }
//     }
// }

//  fn breakx() {
//     let mut i: i8 = 0;
//     loop {
//         println!("i={}", i);
//         i += 1;
//         if i > 5 {
//             break;
//         }
//     }
//     println!("Pengulangan selesai");
// }

// fn loopx() {
//     let mut i = 0;
//     let max = 5;
//     while i < max {
//         let mut j = 0;
//         let max_inner = i;
//         while j <= max_inner {
//             print!("* ");
//             j += 1;
//         }
//         println!("");
//         i += 1;
//     }
// }

// fn timerx() {
//     let mut i = 0;
//     let max = 5;
//     while i < max {
//         println!("Nilai i : {}", i);
//         i += 1;
//         sleep(Duration::from_secs(1));
//     }
// }
