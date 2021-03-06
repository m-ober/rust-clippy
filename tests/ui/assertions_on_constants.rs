macro_rules! assert_const {
    ($len:expr) => {
        assert!($len > 0);
        debug_assert!($len < 0);
    };
}

fn main() {
    assert!(true);
    assert!(false);
    assert!(true, "true message");
    assert!(false, "false message");

    const B: bool = true;
    assert!(B);

    const C: bool = false;
    assert!(C);

    debug_assert!(true);
    // Don't lint this, since there is no better way for expressing "Only panic in debug mode".
    debug_assert!(false); // #3948
    assert_const!(3);
    assert_const!(-1);
}
