fn main() {

    // let width = 30;
    // let height = 50;

    // println!(
    //     "the area of the rectangle is {} pixels", area(width, height)
    // );

    // fn area (width: u32, height: u32) -> u32{
    //     width * height
    // }

    let rect = (30, 50);

    println!("the area of the rectangle is {}", area(rect));


    fn area (dimensions: (u32, u32)) -> u32{
        dimensions.0 * dimensions.1
    }
}
