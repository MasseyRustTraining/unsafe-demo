/// Sets the given element `i` of the array to 7.  More
/// performant than a normal access due to the lack of
/// bounds checking.
///
/// # Safety
///
/// `i` must be a valid index of `a`.
pub unsafe fn set_7(a: &mut [u16], i: usize) {
    let p = a.as_mut_ptr().add(i);
    *p = 7;
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let i = argv[1].parse().unwrap();
    let mut x = [0, 0];
    assert!(i < x.len());
    // Safety: `i` is in range.
    // Need: "Need to go fast."
    unsafe {
        set_7(&mut x, i);
    }
    println!("{:?}", x);
}
