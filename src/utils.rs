pub fn sleep(n: u32) {
    for _ in 0..n {
        core::hint::black_box(());
    };
}
