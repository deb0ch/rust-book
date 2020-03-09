
fn largest_ref<T>(list: &[T]) -> &T
  where T : PartialOrd
{
    let mut largest = &list[0];

    for item in list.iter() {
        if item > &largest {
            // largest = item.clone();
            largest = item;
        }
    }

    &largest
}


pub fn largest_clone<T>(list: &[T]) -> T
  where T : PartialOrd + Clone
{
    let mut largest = list[0].clone();

    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }

    largest
}


fn largest<T>(list: &[T]) -> T
  where T : PartialOrd + Copy
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // {
    //     let r;

    //     {
    //         let x = 5;
    //         r = &x;
    //     }

    //     println!("r: {}", r);
    // }
}


fn test() -> i32 {
    if true {
        1
    } else {
        panic!()
    }
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
