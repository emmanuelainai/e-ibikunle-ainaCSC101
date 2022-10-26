project_2.rs
// representing the laptop brands by its first letter e.g h for hp and q for quantity and a for amount
fn main({
	let q1 = 2;
	let q2 = 1;
	let q3 = 3;
	let q4 = 3;
	let q5 = 1;

    let a1 = 450000;
    let a2= 1500000;
    let a3 = 750000;
    let a4 = 2850000;
	let a5 = 250000;
	
    let t = a1 * q1;
    let m = a2 * q2;
    let h = a3 * q3;
    let d = a4 * q4;
    let a = a5 * q5;

// s is the sum of the product of q and a while v is the average of the sum
	let s = t + m + h + d + a;
	println!("Sum of the record is {}",s);
    let v = s / 5;
	println("Average of the record is {},v");
}
)