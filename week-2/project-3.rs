fn main (){
	let p:f64 = 51000.00;
	let r:f64 = 5.00;
	let t:f64 = 3.00;

	let a =p*(1.00-(r/100.00)).powf(t);

	println!("Amount is{}", a);
	println!("Compound Interest is {}",a);
	}