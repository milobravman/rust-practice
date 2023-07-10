fn main() {
    println!("Hello, world!");

    enum Test{
        First(i32),
        Second(i32)
    }

    //let x = Test::First(30);
    //println!("{}",x);
    let y = 32;
    println!("{}",y);

    fn online(x: Option<i32>) -> Option<i32>{
        match x {
            None => None,
            Some(i) => Some(i+1)
        }
    }

    let k = online(Some(5));
    println!("{:?}", k)
}