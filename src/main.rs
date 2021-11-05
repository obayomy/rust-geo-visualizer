mod debug_visualizer;

fn main() {
    println!("Hello, world!");

    let mut arr = vec![1, 2, 3];
    let mut _s = debug_visualizer::show_arr(&arr);
    for _ in 0..5 {
        arr.swap(0, 2); // set a break-point here, to easily observe the change
        _s = debug_visualizer::show_arr(&arr);
    }
    dbg!(arr);
    println!("break-point here, too");
}
