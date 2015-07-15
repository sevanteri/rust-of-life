struct Grid {
    cells: *mut [bool],
}

fn main() {
    use std::mem;
    //println!("{:?}", mem::size_of::<bool>());
    //println!("{:?}", mem::min_align_of::<bool>());

    let x = 5usize;
    let y = [false; x];

    println!("{:?}", y);
}
