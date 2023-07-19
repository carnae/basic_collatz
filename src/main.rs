use std::time::{Instant, Duration};
use std::str::FromStr;
//use std::fs::File;
//use std::io::Write;

fn main() {
    let low_bound = u128::from_str("2361183241434822606848").unwrap(); 
    let up_bound = u128::from_str("2361183241434823606848").unwrap();
    //let mut hailstones = File::create("HailstonesNormal.txt").expect("creation failed");

    let mut start = Instant::now();
    let mut duration = start.elapsed();
    for element in low_bound..up_bound+1{
        let mut x = element;
        //println!("{x}");
        start = Instant::now();
        while x != 1{
            //hailstones.write(x.to_string().as_bytes()).expect("write failed");
            //hailstones.write(" ".as_bytes()).expect("write failed");
            
            if x % 2 == 0{
                x = x / 2
            }
            else{
                x = (x*3)+1
            }
        }
        let duration = start.elapsed();
        println!("Time taken is: {:?}", duration); //timer print 
    }
        //hailstones.write("\n".as_bytes()).expect("write failed");
}
     //timer stop   
