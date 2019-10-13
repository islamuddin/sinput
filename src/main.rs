mod lib;

fn main(){
    let input_f32=lib::as_f32("Enter your f32 input:");
    println!("f32 input + 1 = {}",input_f32+1.0);
}