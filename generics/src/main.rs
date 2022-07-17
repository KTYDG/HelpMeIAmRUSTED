struct Point<T, K> {
    x: T,
    y: K,
}
fn main() {
    test_non_generic();
    test_generic();
    struct_generic();
}

fn struct_generic() {
    let f = Point { x: 1, y: 2 };
    let s = Point { x: 1.1, y: 2.1 };
    let t = Point { x: "x: 1", y: "y: 2" };
    let n = Point {x: 1.1, y: "y: 2.0"};
    println!("F: {} {}\nS: {} {}\nT: {} {}\nN: {} {}", f.x, f.y, s.x, s.y, t.x, t.y, n.x, n.y);
}

fn test_generic() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn test_non_generic() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
