fn main() {


// Most basic version
    // let width = 30;
    // let height = 50;

    // println!(
    //     "the area of the rectangle is {} pixels", area(width, height)
    // );

    // fn area (width: u32, height: u32) -> u32{
    //     width * height
    // }

// Using tuples version
    // let rect = (30, 50);

    // println!("the area of the rectangle is {}", area(rect));


    // fn area (dimensions: (u32, u32)) -> u32{
    //     dimensions.0 * dimensions.1
    // }

// Using structs version
struct Rect {
    width: i32,
    height: i32
}

let rec = Rect {
    width: 30,
    height: 50
};

// if not borrowed here than the next print would not work
println!("the area of the rectangle is {}", area(&rec));

fn area (rect: &Rect) -> i32{
    rect.height*rect.width
}
println!("the area of the rectangle is {}", area(&rec));

}
