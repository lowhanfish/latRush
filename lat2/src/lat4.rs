fn main() {
    println!("Okeeey");
    asign_if();

    println!("=============================");

    ifcondition(10);
}

fn asign_if() {
    let databool = false;
    let x = if databool { 1 } else { 3 };

    println!("nilai x = {}", x);
}

fn ifcondition(a: i8) {
    let b: i8 = 9;

    if a > b {
        println!("a greater then b");
    } else {
        if a < b {
            println!("a not greater then b");
        } else {
            println!("a equals b");
        }
    }
}
