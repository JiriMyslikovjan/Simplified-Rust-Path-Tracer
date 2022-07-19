mod vector;
mod ray;
mod color;
mod material;
mod object;

fn main() {

    //TODO: materials (diffuse, specular, refractive), image representation, tracing,
    //      writing to PPM, object representation and shapes, scene representation

    let mut r = ray::Ray::new();

    r.setOrigin(1.0 , 2.0, 3.0).setDest(4.0, 5.0, 6.0);

    println!("{}", r.dest.x);
}
