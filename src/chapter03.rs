/*
 ##############
 # chapter 03 #
 ##############
*/
// chapter 3.1 variables and mutability
pub fn errormut(){
    let mut x=1;
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");
}
// shadowing variables
pub fn shadowing(){
    let x=5;
    let x=x+1;
    //Block Expression
    {
        let x=x*x;
        println!("the value of x is {x} in this scope");
    }
    println!("the value of x is {x} after getting out of the scope");
}
//changing variable data type using shadowing
pub fn data_type_change(){
    let letters="hello";
    println!("letters is {letters}");
    let letters = letters.len();
    println!("length of letters is {letters}");
}
//#############################################################
//3.2 Data Types
//tuples
pub fn tup_example(){
    let tup:(i32,f64,u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("{} {} {}", x, y, z);
}
// 3.3 functions
pub fn function_example(x:i32){
    println!("the value of x is {}", x);
}
// Statements and Expressions
pub fn expression_statement(){
    let y={
        let x = 3;
        x + 5
    };
    println!("the value of y is {}", y);
}
// 3.5 control flow
pub fn if_statement(){
    let x=5;
    if x < 7 {
        println!("condition was true");
    }else{
        println!("condition was false");
    }
}
pub fn if_statement2(){
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
pub fn loop_loo(){
    let mut counter=0;
    let result = loop{
        counter+=1;
        if counter==5{
            break counter*2;
        }
    };
    println!("The result is {}", result);
}
//Disambiguating with Loop Labels
pub fn disambiguator_loops() {
    let mut count = 0;
    'counting_up: loop {
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
}
pub fn loop_reverse(){
    for number in (1..5).rev() {
        println!("{}!", number);
    }
    let x=[1,2,3,4,5];
    for element in x {
        println!("The value is {}", element);
    }
}
