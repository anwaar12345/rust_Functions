/*/*
fn main(){
let v = ret(-4);
if v > 0{
println!("positive{}",v);    
}else{
    println!("number is negative {}",v);
}

second(1,1.4,'A',true);

let per =result();

if per >= 80{

println!("You got {} % \n Grade is A+",per);

}else if per < 80 && per >= 70{

println!("You got {} % \n grade is A",per);
    
}else if per < 70 && per >= 60 {

println!("You got {} % \n grade is B",per);
    
}else if per < 60 && per >= 50 {

println!("You got {} % \n grade is C",per);
    
}else if per < 50 && per >= 40 {

println!("You got {} % \n grade is D",per);
    
}else{
    println!("failed");
}


}

//Assignment second question
fn second(first :u64, second :f64, third: char, fourth: bool){
    println!("The Value Of int  is :{} \nthe value of float is:{} \n the value of char is :{} \n the value of Bool is {}",first,second,third,fourth);

}

//Assignment 1st Question
fn ret( number:i64) -> i64{
let n = number;
n

}
//Assignment 4th Question

fn result() -> u64{
    let maths:u64 = 100;
    let computer:u64 = 100;
    let mathmarks =70;
    let computermarks = 94;
println!("Each subject has Total of following  marks \n Maths : {} \n computer : {}",maths,computer);
    let _total: u64 = maths + computer;
    println!("Total Marks are = {}",_total);
    let _mytotal: u64 = mathmarks + computermarks;
    
    let per = _mytotal * 100 / _total; 
    per
}
*/
fn main(){
   let v = number(4);
let tup = (v);

println!("The Value Of number is: {}\n tup is :{}",v,tup * tup);
}

fn number(num:u64) -> u64{
num
} 
*/

//second Practice for RUST Functions

fn main(){
    println!("message from \n");
    cal();
}

//simple function Definition
fn cal(){
    println!("i am from called function");
}
