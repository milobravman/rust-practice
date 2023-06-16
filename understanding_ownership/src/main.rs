fn main() {
    let first = String::from("name");

    // after 'let second ...' any reference to first does not work because first no longer owns the string  
    let second = first; 
    println!("{second}");
}
