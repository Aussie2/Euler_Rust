// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.

use std::time::Instant;

fn main() {
	let mut now = Instant::now();
	let mut result = 0;
	let mut mult_3 = 0;
	let mut mult_5 = 0;
	let inc_3 = 3;
	let inc_5 = 5;
	let limit = 1000;
	
	// println!("Adding the 3s");
	
	while mult_3+inc_3 < limit {
		
		mult_3 += inc_3;
		if mult_3 < limit {
		result  += mult_3;// + mult_5;
		}
		// println!("mult3: {} result: {}", mult_3, result);
	}
	
	// println!("Adding the 5s");
	
	while mult_5+inc_5 < limit {
		
		mult_5 += inc_5;
		if mult_5 < limit {
		result  += mult_5;
		}
		// println!("mult5: {} result: {}", mult_5, result);
		
	}

   println!("The end result is: {}", result);
   println!("elapsed time: {:?}", now.elapsed());
   
   now = Instant::now();
   let mut answer = 0;
 
    for x in 0..1000 {
        if x % 3 == 0 {
            answer += x;
        }
        else if x % 5 == 0 {
            answer += x;
        }
    }
 
    println!("{}", answer);
	println!("elapsed time: {:?}", now.elapsed());
	
	// let	now = Instant::now();

	// let result:u64 = (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
	// println!("elapsed time: {:?}", now.elapsed());
    // println!("Hello, world! {}", result);
}
