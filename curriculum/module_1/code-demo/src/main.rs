// Arrays and slices
fn main() {
    let zeroes: [u8; 6] = [0, 0, 0, 0, 0, 0];
    let zeroes_short: [u8; 6] = [0; 6];
    assert_eq!(zeroes, zeroes_short);

    let arr = [1,2,3,4,5,6];
    let slice = &arr[2..5];
    println!("arr = {:?}", arr);
    println!("slice = {:?}", slice);

    let mut xs = [2,4,6];
    let mut_slice = &mut xs[0..2];
    mut_slice[1] = -45;
    println!("{:?}", xs);
}
