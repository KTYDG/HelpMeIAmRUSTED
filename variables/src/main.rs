fn main() {
    {
        let x = 5;

        let x = x + 1;

        {
            let x = x * 2;
            {
                let x = x * 3;
                println!("The value of x in the inner-INNER scope is: {x}");
            }
            println!("The value of x in the inner scope is: {x}");
        }

        println!("The value of x is: {x}");
    }
    {
        let spaces = "   ";
        let spaces = spaces.len();
        println!("The amount of spaces: {spaces}");
    }
    {
        let tup = ("500", 6.4, '1');

        let (_a, _b, c) = tup;

        println!("The value of y is: {c}");
    }
    {
        let array = [1, 2 ,3 ,4, 5];
        let elem = array[1];
        println!("array[2] = {elem}");
    }
}
