fn main(){
	let toshiba: f64 = 450_000.00;
	let mac :f64 = 1_500_000.00;
	let hp :f64 = 750_000_000.00;
	let dell :f64 = 2_850_000.00;
	let acer :f64 = 250_000.00;

	let total_toshiba : f64 = toshiba * 2.0; // total toshiba
	let total_hp : f64 = hp * 3.0; // total hp
	let total_dell : f64 = dell * 3.0;  // total dell 
	let sum: f64 = total_toshiba + total_hp + total_dell + mac + acer;  // summation of the sales 
	println!("The sum of the sales is: {}", sum); 
	let average : f64 = sum / 10.0;  // find average 
	println!("The average of sales is: {}", average); 
}