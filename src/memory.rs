pub fn run() {
    let mut x: i32 = 0;

    let mut inc = || {
        x += 1;
    };

    inc();

    let y = &mut x;
    *y += 1;

    // NOTE: this code will fail since x is already borrowed
    // inc();

    println!("{}", x);
}
