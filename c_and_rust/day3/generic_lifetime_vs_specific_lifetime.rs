static mut STASH: &i32 = &128;
fn f<'a>(p: &<'a> i32) {
    unsafe {
        STASH = p;
    }
}