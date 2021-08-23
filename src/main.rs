fn main() {
    let tup:(i32,f64,i32)=(14,13.0,12);
    let (x, y, z)=tup;

    println!("x: {} y: {} z: {}", x, y, z);
    println!("z: {}", tup.2)
}
