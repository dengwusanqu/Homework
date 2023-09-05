fn main() {
    let original_vec: Vec<char> = vec!['a','b','c','d','e'];
    
    let new_vec:Vec<char>=original_vec
        .iter()
        .map(|&c|(c as u8+1) as char)
        .collect();
    
    println!("{:?}", new_vec);
}
