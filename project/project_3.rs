project_3.rs
fn main({
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;
	let c = 1.0 - (r / 100.0);
	let z= f64:: powf(c, t);

	let a = p * z;
	println!("Amount is {}",a);
    let d = p - a;
	println("Depreciation is {},d");
}
)