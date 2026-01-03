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
