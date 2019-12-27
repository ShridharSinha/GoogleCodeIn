use std::io;
use std::convert::TryInto;
fn main() {
  let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("failed to read from stdin");

    let trimmed = inp.trim();
    let mut a: i32 = trimmed.parse().unwrap();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),

        Err(..) => println!("this was not an integer: {}", trimmed),
    };
println!("Hello World!");
let mut c:i32=a;
let mut count:i32=0;
while c>0
{
count=count+1;
c=(c/10) as i32;
}
c=a;
let mut sum:i32=0;
let mut i:i32=0;
let mut j:u32=count.try_into().unwrap();
while i<count
{
sum=sum+(c%10).pow(j);
c=c/10;
i+=1;
}
if sum==a
{
println!("Armstrong number");
}
else
{
println!("not an Armstrong number");
}
}
