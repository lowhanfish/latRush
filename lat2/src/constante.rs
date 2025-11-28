// use std::thread::sleep;
// use std::time::Duration;

fn main() {
    let number_a: i8 = 12;
    let number_b: i8 = 14;

    let res_one = number_a == number_b;
    println!("res_one {}", res_one);

    continuex();
    // breakx();
    // loopx();
    // timerx();
}

fn continuex() {
    let mut i = 0;
    let max = 15;

    'nama_label: loop {
        i += 1;

        if i % 2 == 1 {
            continue;
        }

        println!("i : {}", i);

        if i >= max {
            break 'nama_label;
        }
    }
}

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
