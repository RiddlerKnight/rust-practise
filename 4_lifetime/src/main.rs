// fn longest_map_nongeneric(map1: &str, map2: &str) -> &str {
//     if map1.len() > map2.len() {
//         map1
//     } else {
//         map2
//     }
// }

fn longest_map_generic<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}

fn main() {
    let map1: &str = "Ancient Egypt";
    let map2: &str = "Ancient Greece";

    // let longest: &str = longest_map_nongeneric(map1, map2); // error: lifetime may not live long enough
    // println!("The longest map is: {}", longest);

    let longest: &str = longest_map_generic(map1, map2); // this works because the lifetimes of map1 and map2 are the same
    println!("The longest map is: {}", longest);
}
