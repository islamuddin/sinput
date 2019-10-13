
#[doc="created function to reuse and reduce LOC for f32 input."]
#[warn(dead_code)]
pub fn as_f32(message:&str)->(f32){
println!("{}",message);
let mut input = String::new();
std::io::stdin().read_line(&mut input).expect("Input Failed");
let integer: f32 = input.trim().parse().unwrap();
integer
}
