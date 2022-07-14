mod vector;
mod ray;
mod color;

fn main() {

    let mut r = ray::Ray::new();

    r.setOrigin(1.0 , 2.0, 3.0).setDest(4.0, 5.0, 6.0);

    println!("{}", r.origin.x);
}
