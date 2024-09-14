struct MessageStructure {
    magic: u32,
    payload_len: u32, // 32 = 4 * 8 = 4 bytes
    payload: Vec<u8>,
    command: [u8; 12], //
    checksum: u32,
}
fn main() {
    let zeros = vec![0, 0, 0];

    let mut nums = vec![1,2];
    println!("len = {}", nums.len());
    nums.push(3);
    println!("len = {}", nums.len());
}