fn main() {
    println!("Hello, world!");
    print!("Hello new world!");
}
// println! is a macro not a function. Macros are idenitfied with the '!' at the end

// Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design.