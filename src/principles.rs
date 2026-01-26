// ownership
pub fn calling_function() {
    //stack_and_heap();
    //move_ownership();
    //copy_function();
    let s=String::from("hello");
    takes_ownership(s);
    let str1:String=give_ownership();
    print!("{:?}",str1);
}
fn stack_and_heap() {
    let var=1;// created on the stack
    let mut s="hello".to_string();//created on the heap
    s.push_str(", world");
}
fn move_ownership() {
    let x=vec!["hello".to_string()];
    let y=x;
    //println!("{:?}", x); this will give error because we moved the ownership to y
    println!("{:?}",y);

}
fn copy_function() {
    let x=1;
    let y=x;
    println!("x={},y={}",x,y);
}
fn takes_ownership(s: String) {
    let strin=s;
    println!("strin: {}",strin);
}
fn give_ownership() -> String {
    "given".to_string()
}