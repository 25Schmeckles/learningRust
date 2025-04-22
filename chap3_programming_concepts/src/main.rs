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
}
//shows implicit return of an expression (no ;), type must be defined for return, 
// statements end with ; and do not return
fn five() -> i32 {
        5
    }
