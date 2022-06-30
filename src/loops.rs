//loops

pub fn run() {
    let mut count = 0;
    
    //infinite loo&&&&p
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 5 {
            break;
        }
    }

    let mut newcount = 0;
    //while loop (fizzBuzz)
    while newcount <= 15 {
        if newcount % 15 == 0 {
            println!("fizzbuzz");
        } else if newcount % 3 == 0 {
            println!("fizz");
        } else if newcount % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", newcount);
        }
        //inc
        newcount += 1;
    }

    //for range
    for x in 0..10 {
        if x % 2 == 0 {
            println!("Even : {}", x);
        } else {
            println!("Odd : {}", x);
        }
    }
}