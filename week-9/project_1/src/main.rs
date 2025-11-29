use std::fs::File;
use std::io::Write;

fn main() {
    println!("-------- Welcome to Brewery ----------");

    let lager = vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];
    let stout = vec![
        "Legend",
        "Turbo King",
        "Williams",
    ];
    let non_alcoholic = vec![
        "Maltina",
        "Amstel Malta",
        "Malta Gold",
        "Fayrouz",
    ];

    // Create the file
    let mut file = File::create("Brewery.csv").expect("File failed");

    // Write header
    file.write_all("Lager,Stout,Non-Alcoholic\n".as_bytes()).expect("Failed to write");

    // Find the max number of rows
    let max_rows = lager.len().max(stout.len()).max(non_alcoholic.len());

    // Write row by row
    for i in 0..max_rows {
        let lager_file = lager.get(i).unwrap_or(&"");
        let sout_file = stout.get(i).unwrap_or(&"");
        let non_alc = non_alcoholic.get(i).unwrap_or(&"");

        file.write_all(format!("{},{},{}\n", lager_file, sout_file, non_alc).as_bytes())
            .expect("Failed to write");
    }

    println!("File created successfully in perfect table format!");
}
