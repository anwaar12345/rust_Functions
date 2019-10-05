fn main(){
let (num,sqn) = tupr(4);
println!("num is : \n {}",num);
println!("\n square is : \n {}",sqn);
}

fn tupr(num: u64) -> (u64,u64){

    let sqn = num *  num;
    (num,sqn)
}