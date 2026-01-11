
//variables
pub fn variables(){
    let x = 5;
    println!("The value of x is: {}", x);
    let mut y = 5;
    println!("The value of x is: {}", y);
    y=6;
    println!("The value of x is: {}", y);
    const SECONDS:i32 = 60;
    println!("The value of second is: {}", SECONDS);

}
//data types
//scalar data types
pub fn scalar_data_types() {

    //integers
    {
        let x:i8= -10;
        println!("The value of x is: {}", x);

        let y:u32 =8;
        println!("The value of y is: {}", y);
    }
    //integer literals
    {
        let decimal = 02_55;
        let hex = 0xff;
        let octal = 0o77;
        let binary = 0b1111_0000;
        let byte = b'A';
        println!("The value of decimal is: {}", decimal);
        println!("The value of hex is: {}", hex);
        println!("The value of octal is: {}", octal);
        println!("The value of binary is: {}", binary);
        println!("The value of byte is: {}", byte);
    }
    //floating points
    {
        let x:f64 = 5.5;
        println!("The value of x is: {}", x);
    }
    //booleans
    {
        let x=true;
        println!("The value of x is: {}", x);
    }
    //characters
    {
        let cha:char='a';
        println!("The value of cha is: {}", cha);
    }
}
// compound data types
pub fn compound_data_types(){
    //tuples
    let tup=(12,"hello",false);
    let (x,y,z)=tup;
    println!("The value of tup is: {}", tup.0);
    println!("The value of y is: {}", y);

    //arrays
    let ar=[1,2,3,4,5];
    println!("The value of ar is: {}", ar[0]);
    let mut ar2:[i8; 5] = [1,2,3,4,5];
    println!("The value of ar2 is: {}", ar2[3]);
    ar2[3] = 1;
    println!("The value of ar2 is: {}", ar2[3]);
}
//vectors
pub fn vector_data_types(){
    //vectors are resizable arrays of elements allocated on the heap
    //the easiest way to use vectors is to use the VEC macro
    let mut nums=vec![1, 2, 3, 4, 5];
    println!("The value of nums is: {:?}", nums);
    nums.push(10);
    println!("The value of nums is: {:?}", nums);
    nums.pop();
    println!("The value of nums is: {:?}", nums);

    let mut vec=Vec::new();
    vec.push("hello");
    vec.push("world");
    println!("The value of vec is: {:?}", vec);
    vec.reverse();
    println!("The value of vec is: {:?}", vec);

    let mut vec2=Vec::<i32>::with_capacity(2);
    println!("The value of vec2 is: {}", vec2.capacity());
    vec2.push(1);
    vec2.push(2);
    vec2.push(3);
    println!("The value of vec2 is: {:?}", vec2);
    println!("The value of vec2 is: {}", vec2.capacity());
}
//Slices
pub fn slices_example(){
    //a slice is a region of an array or vector that could be any link
    //slices can not be stored directly into variable or passed as function argument

    let v:Vec<i32>=(0..5).collect();
    println!("The value of v is: {:?}", v);
    let sv:&[i32]=&v[2..4];
    println!("The value of sv is: {:?}", sv);
}
//Strings and &str
pub fn string_data_types(){
    let mut s = String::from("hello");
    let name=String::from("kirolos");
    let last_name="roman".to_string();
    let new_name=name.replace("kirolos","ki");
    println!("The value of new name is: {}", new_name);

    //string literals
    let rust="\x52\x65";
    println!("The value of new name is: {}", rust);

}
// functions
pub fn function_call(){
    print_phrase();
}
fn print_phrase(){
    println!("hello there");
    print_phrase_string_slice("hello there");
    println!("{}",gcd(20,5))
}
fn print_phrase_string_slice(phrase:&str){
    println!("{}", phrase);
}
fn gcd(mut a:u64,mut b:u64)->u64{
    while a!=0{
        if a<b{
            let c=a;
            a=b;
            b=c;
        }
        a=a%b;
    }
    b
}
