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
    // let rec = Rect {
    //     width: 30,
    //     height: 50
    // };
    // if not borrowed here than the next print would not work
    // println!("the area of the rectangle is {}", area(&rec));
    // fn area (rect: &Rect) -> i32{
        //     rect.height*rect.width
        // }
        // println!("the area of the rectangle is {}", area(&rec));

// Using methods version
    let rect1 = Rect{
        width: 30,
        height: 50,
    };

    let rect2 = Rect{
        width: 10,
        height: 40,
    };

    let rect3 = Rect{
        width: 60,
        height: 45,
    };

    println!("can rect1 hold rect2? {}", rect1.can_contain(&rect2));
    println!("can rect1 hold rect2? {}", rect1.can_contain(&rect3));


}
    
    struct Rect {
        width: i32,
        height: i32
    }

    impl Rect {
        fn can_contain(&self, other: &Rect) -> bool{
            if self.area() > other.area() 
            {
                true
            }
            else {
                false
            }
        }

        fn area(&self) -> i32 {
            self.width*self.height
        }

        fn square(size: i32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    
    }