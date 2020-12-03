mod map;

fn main() {
    println!("Hello, world!");
    let mut map = map::load();
    let mut trees = 0;
    loop {
        map.move_3_1();
        if !map.is_map_end() {
            if map.is_tree_at_curr_pos() {
                println!("found tree: {}", trees);
                trees += 1;
            }
        } else {
            println!("Number of trees is: {}", trees);
            break;
        }
    }
}
