#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let my_rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("the area of the pixels is {}.", my_rect.area());
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
