fn main(){
	let price : f64 = 510_000.00;
	let dp_rate : f64 = 5.0;
	let num_years : f64 = 3.0;
	let depreciation :f64 = price * (1.0 - (dp_rate / 100.0)).powf(num_years);
    println!("The depreciation of {} at the rate of {}% is : {:.4} ", price, dp_rate, depreciation);
}