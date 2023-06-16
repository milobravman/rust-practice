fn main() {
    let first = String::from("name");

    // after 'let second ...' any reference to first does not work because first no longer owns the string  
    let second = first; 
    println!("{second}");
    // println! ("{first}") would cause an error because we can no longer reference this variable

    // need to add example on references and borrowing

    // need add example on ownership errors
}
