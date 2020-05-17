fn main() {
    let w = 30;
    let h = 50;

    println!("This area of the rectangle is {} square pixels.", area(w, h))
}

fn area(w: u32, h: u32) -> u32 {
    w * h
}
