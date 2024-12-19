#[derive(Debug)]

//practice with creating structs, methods, and associated functions
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// multiple impl blocks for the same struct is valid syntax
impl Rectangle {
    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let my_rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 60,
        height: 45,
    };

    // associated functions do not take &self as the parameter, but return an instance of the struct
    let my_square = Rectangle::square(50);

    // practice with calling methods
    println!("can my_rect hold rect1? {}", my_rect.can_hold(&rect1));
    println!("can my_rect hold rect2? {}", my_rect.can_hold(&rect2));
    println!("the area of the pixels is {}.", my_rect.area());
    if my_rect.width() {
        println!("the rectangle has a width, and it is {}", my_rect.width);
    }
    println!("my square is a {my_square:#?}");
}

/*

// this isn't ideal because it's not clear where in the program the variables are related
 let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

// this isn't ideal because you have to reference the index of a tuple for width/height
    let rectangle = (30, 50);

    println!("the area is {}", area(rectangle));

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

// this uses a function call, but could be improved by using a method on the struct

    let scale = 2;
    let my_rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50
    };
   //println!("the area is {}", area(&my_rect));
    //println!("my_rect is {my_rect:#?}");
    dbg!(&my_rect);

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

 */
