use std::fs;

    	let element_data_string = fs::read_to_string("element_data.csv").unwrap();
    	//let mut split_lines = element_data_string.lines().map(|line| line.split(",").collect::<Vec<_>>());

    	let mut csv_body = String::new(); 

    	let mut csv_heading = Vec::new();

	let mut split_lines = element_data_string.lines().map(|line| line.split(",").collect::<Vec<_>>());

	let mut accepted_indices = Vec::new();

	for (index, heading) in split_lines.next().unwrap().into_iter().enumerate() {
		match heading {
			"AtomicNumber" => csv_heading.push(String::from("atomic_number")), 
			"Element" => csv_heading.push(String::from("name")), 
			"Symbol" => csv_heading.push(String::from("symbol")), 
			"AtomicMass" => csv_heading.push(String::from("atomic_mass")), 
			"Electronegativity" => csv_heading.push(String::from("electronegativity")), 
			_ => continue,
		}

		accepted_indices.push(index);
	}

	println!("{:?}", csv_heading.join(", "));

	for line in split_lines.map(|line| line.into_iter().enumerate().filter(|(index, entry)| accepted_indices.contains(index)).map(|(_, entry)| if entry == "" { "0.0" } else { entry.trim() })) { 
		let line: Vec<_> = line.collect();
		let line = line.join(", ");
		println!("{:?}", line);
	} 
			

