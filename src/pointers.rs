// Pointers reference to a point in memory

pub fn run() {
    // Primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("{:?} values", (arr1, arr2));

    // Vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("{:?} values", (&vec1, vec2));
}