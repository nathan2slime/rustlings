// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.

#[allow(unused_variables, unused_assignments)]
#[allow(clippy::panicking_unwrap)]
#[allow(clippy::unnecessary_literal_unwrap)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        my_option.unwrap()
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    println!("This Vec is empty, see? {:?}", vec![1, 2, 3, 4, 5].resize(0, 5));

    let mut value_a = 45;
    let value_b = 66;
    // Let's swap these two!
    value_a = value_b;

    println!("value a: {}; value b: {}", value_a, value_b);
}
