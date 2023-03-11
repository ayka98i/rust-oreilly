static mut STASH: &i32 = &128;
static WORTH_POINTING_AT: i32 = 1000;

fn main() {
    // f(&WORTH_POINTING_AT);
}

fn f(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

fn g<'a>(p: &'a i32) {
    todo!();
}
