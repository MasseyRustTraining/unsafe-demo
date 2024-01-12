fn main() {
    // # Safety
    // This block actually has no unsafe code in it.
    // # Need
    // This is a demo.
    #[allow(unused_unsafe)]
    unsafe {
        let x = "x".to_string();
        let y = &x;
        // drop(x);
        println!("{}", *y);
    }

    let x = 12u8;
    let p: *const u8 = &x;
    let q: *const u8 = &x;
    // # Safety
    // `p` and `q` point at unique valid mutable data on the
    // stack in this block.
    // # Need
    // This is a demo.
    unsafe {
        let x = (*p).wrapping_add(*q);
        println!("{}", x);
    }

    let a: [i32; 2] = [0, 1];
    let mut p: *const i32 = &a[0];
    // # Safety
    // `p` points at a valid value at the start of this block
    // and after the increment.
    // # Need
    // This is a demo.
    unsafe {
        p = p.add(1);
        // // This is wildly UB.
        // let q = (p as *mut u8).add(1) as *mut u32;
        // *q += 1;
        println!("{}", *p);
    }
}
