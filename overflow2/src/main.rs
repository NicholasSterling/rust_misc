
pub const IMG_ROWS: usize = 360;
pub const IMG_COLS: usize = 640;
pub const IMG_PIXELS: usize = IMG_ROWS * IMG_COLS;
pub const IMG_BYTES_PER_PIXEL: usize = 4;
pub const IMG_BYTES: usize = IMG_PIXELS * IMG_BYTES_PER_PIXEL;

pub type Pixel = [u8; 4];  // RGBA
pub type ImageRow = [Pixel; IMG_COLS];
pub type Image = [ImageRow; IMG_ROWS];

fn main() {
    let mut img1 = [[[0u8; 4]; IMG_COLS]; IMG_ROWS];
    img1[1][2][3] = 73;
    let mut vec: Vec<Image> = Vec::new();
    vec.push(img1);
    let img2 = &mut vec[0];
    assert_eq!(img2[1][2][3], 73);
    img2[1][2][3] = 39;
    assert_ne!(img1[1][2][3], img2[1][2][3]);
    let mut img3 = Box::new(img1);
    img3[1][2][3] = 17;
    assert_ne!(img1[1][2][3], img3[1][2][3]);
    let mut vec2: Vec<ImageRow> = Vec::new();
    for _ in 0..IMG_ROWS {
        vec2.push(img1[0]);
    }
    // let img4: &Image = unsafe {
    //     use slice_as_array::*;
    //     std::mem::transmute(
    //         slice_as_array!(vec2.as_slice(), [ImageRow; IMG_ROWS]).unwrap()
    //     )
    // };
    use slice_as_array::*;
    let img4: &Image =
            slice_as_array!(vec2.as_slice(), [ImageRow; IMG_ROWS]).unwrap();
    let mut img5 = Box::new(*img4);
    img5[0][0][1] = 33;
    assert_ne!(img1[0][0][1], img5[0][0][1]);
    eprintln!("Hello, world!");
}
