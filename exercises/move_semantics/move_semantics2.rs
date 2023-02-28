// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let mut vec0 = Vec::new();

    //let mut vec1 = fill_vec(vec0.clone()); // Method 1
    //let mut vec1 = fill_vec(&vec0); // Method 2
    fill_vec(&mut vec0); // Method 3

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    //vec1.push(88); // 1 & 2
    vec0.push(88); // 3

    //println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1); // 1 & 2
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0); // 3
}

// Method 1
//fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//let mut vec = vec;
 
// Method 2
//fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//let mut vec = vec.clone();

// Method 3
fn fill_vec(vec: &mut Vec<i32>) {

    vec.push(22);
    vec.push(44);
    vec.push(66);

    //vec // 1 & 2
}
