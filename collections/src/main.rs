


fn main() {

    exercise1();
    exercise3();

}

use std::collections::HashMap;


// Given a list of integers, use a vector and return the 
//   median (when sorted, the value in the middle position) 
//   and mode (the value that occurs most often; a hash map will be helpful here) 
//   of the list.
fn exercise1() {

    let mut integers = [1, 1, 9, 7, 6, 3];
    println!("given list: {:?}", integers);

    integers.sort();
    // println!("sorted list: {:?}", integers);

    let i = integers.len() / 2;
    println!("median: {}", &integers[i]);

    let mut m = HashMap::new();
    for v in integers {
        let count = m.entry(v).or_insert(0);
        *count += 1;
    }
    // println!("{:?}", m);

    let mut most_often_key = -1;
    let mut most_often_count = 0;
    for (key, value) in m {
        if value > most_often_count {
            most_often_key = key;
            most_often_count = value;
        }
    }
    println!("most often {} occurred {} times", most_often_key, most_often_count);

}

// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
fn exercise3() {

    let commands = vec![
        "Add Sally to Engineering",     // <ignore> <name> <ignore> <department>
        "Add Amir to Sales",
        "Add Bob to Engineering",
        "Add Jack to Sales",
    ];

    let mut m: HashMap<String, Vec<String>> = HashMap::new();

    for c in &commands {
        let mut splitted = c.split_whitespace();

        match splitted.nth(1) {
            Some(name) => {
                match splitted.nth(1) {
                    Some(department) => {
                        let e = m.entry(String::from(department)).or_insert(Vec::new());
                        e.push(String::from(name));                
                    }
                    None => {
                        println!("unknown department, ignore");
                    }
                }
            }
            None => {
                println!("unknown name, ignore");
            }
        }
    }

    println!("{:?}", m);

    if let Some(v) = m.get(&String::from("Engineering")) {
        let mut v = v.clone();
        v.sort();

        println!("Engineering team: {:?}", v);
    }
}
