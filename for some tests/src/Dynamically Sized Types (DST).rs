use std::mem::size_of;

// Dynamically Sized Types (DST)

// no DST
fn main() {
    let nums: &[i32; 3] = &[1, 2, 3];

    assert_eq!(8, size_of::<&[i32; 3]>());
    
    for num in nums {
        println!("{}", num)
    }

// DST
    let nums: &[i32] = &[1, 2, 3];
    // fat pointer
    assert_eq!(16, size_of::<&[i32]>());

    for num in nums {
        println!("{}", num)
    }

}

