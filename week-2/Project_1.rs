fn main(){
	let priciple : f64 = 52_000_000.00;
	let number_of_years: f64 = 5.0;
	let rate: f64 = 10.0/100.0; 
	
	let compound_interest = priciple * ( 1.0 + rate ).powf(number_of_years);
	println!("The compound interest for {} at the rate of 10% per annum is {}", number_of_years, compound_interest);
}