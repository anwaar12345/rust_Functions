/* fn main(){
let per :u64 = per(80,56);
if per > 33 {
println!("passed with \n {} %",per);    
}
}

fn per(maths:u64,computer:u64) -> u64{
    let total: u64 = 200;
    
    let marksobtain =maths + computer;
    let per = marksobtain * 100 / total;
    per 
}

*/


fn main(){
let per :f64 = per(80,56);
if per > 33.0 {
println!("passed with \n {} %",per);    
}
}

fn per(maths:u64,computer:u64) -> f64{
    let total: u64 = 200;
    
    let marksobtain =maths + computer;
    let per: f64 = (marksobtain * 100 / total) as f64;
    per 
}