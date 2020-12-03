mod map;

fn main() {
    let mut map = map::load();
    let mut trees = 0;
    let mut results: Vec<usize> = Vec::with_capacity(5);
    loop {
        map.move_3_1();
        if !map.is_map_end() {
            if map.is_tree_at_curr_pos() {
                trees += 1;
            }
        } else {
            results.push(trees);
            break;
        }
    }

    map.reset();
    trees = 0;
    loop {
        map.move_by(1, 1);
        if !map.is_map_end() {
            if map.is_tree_at_curr_pos() {
                trees += 1;
            }
        } else {
            results.push(trees);
            break;
        }
    }

    map.reset();
    trees = 0;
    loop {
        map.move_by(5, 1);
        if !map.is_map_end() {
            if map.is_tree_at_curr_pos() {
                trees += 1;
            }
        } else {
            results.push(trees);
            break;
        }
    }

    map.reset();
    trees = 0;
    loop {
        map.move_by(7, 1);
        if !map.is_map_end() {
            if map.is_tree_at_curr_pos() {
                trees += 1;
            }
        } else {
            results.push(trees);
            break;
        }
    }

    map.reset();
    trees = 0;
    loop {
        map.move_by(1, 2);
        if !map.is_map_end() {
            if map.is_tree_at_curr_pos() {
                trees += 1;
            }
        } else {
            results.push(trees);
            break;
        }
    }

    println!("1. solution: {}", results[0]);
    println!("2. solution: {}", results.iter().fold(1, |acc, &x| acc * x));
}
