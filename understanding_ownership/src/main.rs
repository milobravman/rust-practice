fn main() {
    let first = String::from("name");

    // after 'let second ...' any reference to first does not work because first no longer owns the string  
    let second = first; 
    println!("{second}");
    // println! ("{first}") would cause an error because we can no longer reference this variable

    // need to add example on references and borrowing

    // example on ownership errors Referencing out of a collection
    let first: Vec<String> = vec![String::from("hello world")];
    let s_ref: &String = &first[0];
    println!("{s_ref}");
    //let s: String = *s_ref; this line will not work because the heap will get freed twice


    // manipulating arrays and seeing how & and * work
    // keeping track of permissions and when to use & and * is really confusing
    let mut point = [0, 1];
    let mut x = point[0];
    let y = &mut point[1];
    x += 1;
    *y += 1;
    println!("{} {} {}", point[0], point[1], x);



}
