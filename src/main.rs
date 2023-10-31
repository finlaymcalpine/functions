fn main() {
    another_function(5);
    print_labeled_measurement(5, 'm');
    
    let val = adder(3, 8);
    println!("The sum is {val}");
}

// We can define a function (called from main) either above or below main(). Just has to be in scope.

fn another_function(x: i32) {
    println!("The value of x is: {x}."); // We can add a semicolon here because this function only prints a line out.
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurment is: {value}{unit_label}");
}

// Statements are instructions that do not return a value: e.g. let x = 6 is a statement. Because we are creating a variable
// and assigning a value to that variable. A function definition is a statement.
// Expressions are instructions that evaluate to a value and return that value. E.g. 5 + 6 evaluates to 11. Calling a function 
// is an expression. Expressions do not have an ending semi-colon. If we do, it becomes a statement and won't return a value.

// Functions can be set to return a value when called, rather than just prining arguments as with the functions above.
// This is done by declaring the return type after the argument brackets, as below.

fn adder(x: i32, y: i32) -> i32 {
    x + y // No semicolon because we want this line to return a value.
}