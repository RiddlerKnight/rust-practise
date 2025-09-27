mod working_with_string;

fn main() {
    let foo = 11;
    if foo <= 5 {
        println!("Binggo!");
    } else {
        println!("Nope!");
    }

   let _bar = if foo >= 10 {
        "High"
    } else if foo <= 3 {
        "Low"
    } else {
        "Medium"
    };

    println!("bar is: {}", _bar);

    let _baz = match foo {
        1 => "One",
        2 | 3 => "Two or Three",
        4..=10 => "Between Four and Ten",
        _ => "Greater than Ten",
    };
    println!("baz is: {}", _baz);

    // loops
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            continue;

        }
        println!("Count is: {}", count);
        if count >= 5 {
            break;
        }
    }

    let (mut x, mut y, mut z) = (0, 0, 0);
    'top_loop: loop {
        loop {
            loop {
                println!("x: {}, y: {}, z: {}", x, y, z);
                if x == 2 && y == 2 && z == 2 {
                    break 'top_loop; // exit all loops with label
                }
                x += 1;
                y += 1;
                z += 1;
            }
        }
    }

    // loop with range
    for i in 1..5 {
        println!("i is: {}", i);
    }
    for g in 1..=5 {
        println!("g is: {}", g);
    }

    // loop through array
    for item in ["apple", "banana", "cherry"].iter() {
        println!("item: {}", item);
    }

    // loop with index
    for (index, value) in [10, 20, 30].iter().enumerate() {
        println!("index: {}, value: {}", index, value);
    }

    // nested loop through 2D array
    let array = [[100, 200], [300, 400], [500, 600]];
    for row in array.iter() {
        for col in row.iter() {
            println!("col: {}", col);
        }
    }

    working_with_string::run();
}
