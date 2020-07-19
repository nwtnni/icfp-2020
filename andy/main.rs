// use std::vec::Vec;
// use icfp::Value;
// use icfp::draw::multidraw;

// fn build_vec(mut vec: Vec<(i64, i64)>, acc: Value) -> Value {
//     if vec.is_empty() {
//         return acc
//     };
//     let (x, y) = vec.pop().expect("Empty vec?");
//     build_vec(
//         vec,
//         Value::Cons(
//             Box::new(
//                 Value::Cons(
//                     Box::new(Value::Int(x)),
//                     Box::new(Value::Int(y)),
//                 )
//             ),
//             Box::new(acc)
//         )
//     )
// }

// fn build_images_vec(mut vec: Vec<Vec<(i64, i64)>>, acc: Value) -> Value {
//     if vec.is_empty() {
//         return acc
//     };
//     let image: Vec<(i64, i64)> = vec.pop().expect("Empty list of images?");
//     build_images_vec(
//         vec,
//         Value::Cons(
//             Box::new(build_vec(image, Value::Nil)),
//             Box::new(acc)
//         )
//     )
// }

fn main() -> anyhow::Result<()> {
    env_logger::init();

//     // let _ = icfp::Client::new()?;
//     let mut image1: Vec<(i64, i64)> = Vec::new();
//     image1.push((2, 2));
//     image1.push((2, 3));
//     image1.push((3, 2));
//     image1.push((3, 3));

//     let mut image2: Vec<(i64, i64)> = Vec::new();
//     image2.push((6, 6));
//     image2.push((6, 7));
//     image2.push((7, 6));
//     image2.push((7, 7));

//     let mut image3: Vec<(i64, i64)> = Vec::new();
//     image3.push((10, 10));
//     image3.push((10, 11));
//     image3.push((11, 10));
//     image3.push((11, 11));

//     multidraw(&build_images_vec(vec![image1, image2, image3], Value::Nil));

    Ok(())
}
