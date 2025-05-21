fn main() {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("The value of y is: {y}");
        
        let x: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;

        //array accessing
        let a = [1, 2, 3, 4, 5];

        let first = a[0];
        let second = a[1];

        //vector accessing
        let mut v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);

        let v = vec![1, 2, 3, 4, 5];

        println!("The value at 0 is: {}",v[0]);
        let x = five();

        println!("The value of x is: {x}");

        let condition = true;
        let number = if condition { 5 } else { 6 }; //ensure the types of each of these matches!

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }

        //loop types are: loop,while,for
        //break statements always return a value, continue does not but a break needs a ;
        //you can also label loops and specifically break/continue to that loop
        let mut count = 0;
        'counting_up: loop { //labeled loop
            println!("count = {count}");
            let mut remaining = 10;
    
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        }
        println!("End count = {count}");

        //while example
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
    
        while index < 5 {
            println!("the value is: {}", a[index]);
    
            index += 1;
        }

        //for example, safest and preferred loop type in rust
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }
        //another for loop with a range, range starts on 1 and ends on 3
        for number in (1..4).rev() {
                println!("{number}!");
            }
            println!("LIFTOFF!!!");


}
//shows implicit return of an expression (no ;), type must be defined for return, 
// statements end with ; and do not return
fn five() -> i32 {
        5
    }
