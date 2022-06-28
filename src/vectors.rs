//vectors are resizable arrays

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", numbers);
    //add 
    numbers.push(7);
    numbers.push(8);
    println!("Adding two num {:?}", numbers);
    numbers.pop();
    println!("Pop remove one num {:?}", numbers);

    //loop through vector values
    for x in numbers.iter() {
        println!("Number {}", x);
    }

    //loop and mutate values
    //similar to map in js
    for x in numbers.iter_mut() {
        *x *= 2;

    }
    println!("Number {:?}", numbers);
}