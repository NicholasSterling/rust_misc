
/*
 * I'm trying to explore what causes a stack overflow by creating a
 * big array on the stack and then using various techniques to create
 * a boxed copy of it.
 */

const N: usize = 1_000_000;  // causes stack overflow on Mac OS
// by_clone1: stack overflow
// by_clone2: stack overflow
// by_vec: no stack overflow, but free of unallocated pointer

type BigArray = [i32; N];

fn main() {
    let mut array = [0; N];
    array[0] = 73;
    eprintln!("len = {}", array.len());
    // let it = _by_clone1(array);
    // let it = _by_clone2(array);
    let it = _by_copyless(array);
    // let it = _by_vec(array);
    // let it = _by_as_array(array);
    eprintln!("len = {}", it.len());
    // _by_as_ptr(array);
}

fn _by_clone1(it: BigArray) -> Box<BigArray> {
    eprintln!("by_clone1");
    let clone = it.clone();
    eprintln!("cloned");
    Box::new(clone)  // moves it to the heap, presumably
}

fn _by_clone2(it: BigArray) -> Box<BigArray> {
    eprintln!("by_clone2");
    Box::new(it.clone())  // moves it to the heap, presumably
}

// This doesn't seem to work!
fn _by_copyless(it: BigArray) -> Box<BigArray> {
    use copyless::BoxHelper;
    eprintln!("by_copyless");
    Box::alloc().init(it.clone())
}

// Here we create a Vec, which stores its data on the heap,
// and convert it into a boxed slice.  We transmute the addr
// of the first element into an array.  However, the slice
// gets dropped at the end, so the pointer is invalid.
// Stack overflow is avoided, but at the end of main() (after
// printing len, it dies with "ptr being freed was not allocated."
fn _by_vec(it: BigArray) -> Box<BigArray> {
    let vec = it.iter().cloned().collect::<Vec<i32>>();
    let slice = vec.into_boxed_slice();
    let first_element = &slice[0];
    eprintln!("first_element: {:p}", &first_element);
    let box_t = unsafe {
        std::mem::transmute(first_element)
    };
    eprintln!("transmuted: {:p}", &box_t);
    box_t
}

// This could work.  Build a Vec from the array, cloning one element at a time,
// and store that, transmuting it into an array when you want to use it.
// Vec uses box, but does not require the nightly compiler.
fn _by_as_ptr(it: BigArray) {
    let vec = it.iter().cloned().collect::<Vec<i32>>();
    let ptr = vec.as_ptr();
    let array: &[i32; N] = unsafe {
        std::mem::transmute(ptr)
    };
    assert_eq!(array[0], 73);
}

/*
// This is similar to by_vec, but we use the slice_as_array crate
// to convert the slice to the desired array.
fn _by_as_array(it: BigArray) -> BigArray {
    use slice_as_array::*;
    let vec = it.iter().cloned().collect::<Vec<i32>>();
    let slice = vec.into_boxed_slice();
    *slice_as_array!(slice, [i32; N]).expect("length was wrong")
}
fn _by_to_array_clone(it: BigArray) -> BigArray {
    use slice_as_array::*;
    let vec = it.iter().cloned().collect::<Vec<i32>>();
    let slice = vec.into_boxed_slice();
    slice_to_array_clone!(slice, [i32; N]).expect("length was wrong")
}
*/

/*
// Well, I was going to try new_uninit, but that's unstable too.
// Same with new_zeroed.
fn _by_uninit(it: BigArray) -> Box<BigArray> {
    let mut it = Box::<BigArray>::new_uninit();
}
 */
