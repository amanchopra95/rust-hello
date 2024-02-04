use std::mem;

pub fn run(){
    let numbers: [i32; 4] = [1,2,3,4];
    println!("{:?}: numbers", numbers);

    // Get single value
    println!("{} 0th index", numbers[0]);

    let mut mut_numbers: [i32; 5] = [1,2,3,4,5];
    mut_numbers[1] = 3;

    println!("{} 1st index", mut_numbers[1]);

    // Arrays are stack allocated;
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Slices from an array
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);
}