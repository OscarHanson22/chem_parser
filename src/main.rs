use chemical::{Element, Elements};
use std::collections::HashMap;

#[derive(Debug)]
struct MolecularUnit {
	element: Element, 
	quantity: usize, 
}

impl MolecularUnit {
	fn new(element: Element, quantity: usize) -> Self {
		Self {
			element, 
			quantity, 
		}
	}

	fn multiply(&mut self, by_factor: usize) {
		self.quantity *= by_factor;
	}
}

#[derive(Debug)]
struct ParentheticalUnit {
	molecular_units: Vec<MolecularUnit>,
	multiplier: usize,  
	children: Vec<ParentheticalUnit>, 
}

use std::iter::Peekable;
use std::str::Chars;

struct MoleculeReader<'a> {
	chars: Peekable<Chars<'a>>,
	element_hashmap: HashMap<String, fn() -> Element>,
}

impl<'a> MoleculeReader<'a> {
	fn new(text: &'a str) -> Self {
		Self {
			chars: text.chars().peekable(), 
			element_hashmap: Elements::as_hashmap(),
		}
	}

	fn is_done(&mut self) -> bool {
		self.chars.peek().is_none()
	}

	fn peek(&mut self) -> Option<char> {
		self.chars.peek().copied()
	}

	fn read_quantity(&mut self) -> usize {
		let mut quantity_string = String::new();

		while let Some(c) = self.chars.next_if(|&c| c.is_numeric()) {
			quantity_string.push(c);
		}

		let quantity: usize = quantity_string.parse()
			.unwrap_or(1);

		quantity
	}

	fn read_element(&mut self) -> Option<Element> {
		let mut element_string = String::new();

		if let Some(c) = self.chars.peek() {
			if c.is_uppercase() {
				element_string.push(*c);
				self.chars.next();
			} else {
				return None;
			}
		}

		while let Some(c) = self.chars.next_if(|&c| c.is_lowercase()) {
			element_string.push(c);
		}

		let element = self.element_hashmap.get(&element_string).expect("LMAO for NOW");
		let element = element();

		Some(element)
	}

	fn capture_parentheses(&mut self) -> Option<String> {
		if let Some(c) = self.chars.peek() {
			if *c != '(' {
				return None;
			}
		}

		self.chars.next();

		let mut balancing_sum = 0;
		let mut captured_string = String::new();

		while let Some(c) = self.chars.next() {
			if balancing_sum == 0 && c == ')' {
				return Some(captured_string);
			}

			if c == '(' {
				balancing_sum += 1;
			} else if c == ')' {
				balancing_sum -= 1;
			}

			captured_string.push(c);
		}

		Some(captured_string)
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseMoleculeError;

impl ParentheticalUnit {
	fn build(molecule_string: &str, multiplier: usize) -> Result<Self, ParseMoleculeError> {
		let mut parenthetical_unit = Self {
			molecular_units: Vec::new(), 
			multiplier, 
			children: Vec::new(), 
		};

		let mut molecule_reader = MoleculeReader::new(molecule_string);

		while !molecule_reader.is_done() {
			if molecule_reader.peek() == Some('(') {
				let captured_molecule_string = molecule_reader.capture_parentheses().ok_or(ParseMoleculeError)?;
				let multiplier = molecule_reader.read_quantity();
				parenthetical_unit.children.push(Self::build(&captured_molecule_string, multiplier)?);
			} else {
				let element = molecule_reader.read_element().ok_or(ParseMoleculeError)?;
				let quantity = molecule_reader.read_quantity();
				parenthetical_unit.molecular_units.push(MolecularUnit::new(element, quantity));
			}
		}

		Ok(parenthetical_unit)
	}

	fn multiply(&mut self, by_factor: usize) {
		self.multiplier *= by_factor;
	}

	fn absorb(&mut self, other: Self) {
		for mut molecular_unit in other.molecular_units {
			molecular_unit.multiply(other.multiplier);

			self.molecular_units.push(molecular_unit);
		}

		for mut parenthetical_unit in other.children {
			parenthetical_unit.multiply(other.multiplier);

			self.children.push(parenthetical_unit);
		}
	}

	fn combine(&mut self) {
		while let Some(child) = self.children.pop() { 
			self.absorb(child);
		}
	}
}

#[derive(Debug)]
pub struct Molecule {
	formula: String,
	elements: HashMap<Element, usize>, 
}

impl Molecule {
	pub fn new(formula: &str) -> Self {
		Molecule {
			formula: formula.to_string(),
			elements: HashMap::new(), 
		}
	}

	pub fn add_element(&mut self, element: Element, quantity: usize) {
		*self.elements.entry(element).or_insert(0) += quantity;
	}

	pub fn molar_mass(&self) -> f64 {
		self.elements.iter()
			.map(|(element, quantity)| element.atomic_mass() * *quantity as f64)
			.sum::<f64>()
	}
}

// pub fn new(components: &[(fn() -> Element, usize)]) -> Self {
// 	for (element, quantity) in components {
		

struct Property<S, T: Copy> {
	value: Option<T>, 
	production_function: fn(&S) -> T, 
}

impl<S, T: Copy> Property<S, T> {
	fn new(production_function: fn(&S) -> T) -> Self {
		Self {
			value: None, 
			production_function, 
		}
	}

	fn get(&mut self, owner: &S) -> T {
		*self.value.get_or_insert((self.production_function)(owner))
	}
}

// struct MoleculeBuilder {
// 	name: Option<String>,
// 	elements: Vec<(fn() -> Element, usize)>,
// }

// impl MoleculeBuilder {
// 	pub fn add(&mut self, element: Element 

impl std::str::FromStr for Molecule {
	type Err = ParseMoleculeError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let parenthetical_unit = {
			let mut parenthetical_unit = ParentheticalUnit::build(s, 1)?;
			parenthetical_unit.combine();

			parenthetical_unit
		};

		let molecule = {
			let mut molecule = Molecule::new(s);

			for molecular_unit in parenthetical_unit.molecular_units {
				molecule.add_element(molecular_unit.element, molecular_unit.quantity);
			}

			molecule
		};

		Ok(molecule)
	}
}

struct Ion {
	elements: Molecule, 
	charge: isize,
}

struct Reaction {
	reactants: Vec<(usize, Molecule)>, 
	products: Vec<(usize, Molecule)>, 
}

impl Reaction {
	// fn new(reactants: Vec<Molecule>, products: Vec<Molecule>) -> Self {
	// 	Self {
	// 		reactants, 
	// 		products, 
	// 	}
	// }

	// fn balance(&mut self) {

}

// impl std::str::FromStr for Reaction {
// 	type Err = ParseMoleculeError;

// 	fn from_str(s: &str) -> Result<Self, Self::Err> {

// 	}
// }

fn main() {
	// let mut molecule: Molecule = "Na(SO4(HMg(S2))2)3".parse().unwrap();

	// println!("{:#?}", molecule.elements);

	// let mut molar_mass_property = Property::new(Molecule::molar_mass);
	// println!("{:#?}", &molar_mass_property.value);
	// println!("{:#?}", molar_mass_property.get(&molecule));
	// println!("{:#?}", &molar_mass_property.value);

	loop {
		println!("Enter formula:");

		let formula = {
			let mut formula_string = String::new();

    		std::io::stdin()
    			.read_line(&mut formula_string)
    			.expect("Error reading line");

    		String::from(formula_string.trim())
		};

		let molecule: Molecule = match formula.parse() {
			Ok(molecule) => molecule,
			Err(_) => {
				println!("Error parsing molecule");
				continue
			} 
		};

		println!("Molar mass: {} g/mol", molecule.molar_mass());
		println!();
	}
	

	//use std::str::FromStr;
	
	

	/*
	use std::fs;

	let element_data = fs::read_to_string("element_data.csv").unwrap();

	let mut element_data_iter = element_data.lines();

	let headings: Vec<_> = element_data_iter.next().unwrap().split(", ").collect();

	for element_data in element_data_iter.map(|line| line.split(", ").collect::<Vec<_>>()) {
		println!("pub fn {}() -> Element {{", element_data[1].to_lowercase());
		
		println!("      Element {{");

		for (heading, entry) in headings.iter().zip(element_data.iter()) {
			match *heading {
				"name" | "symbol" => println!("            {heading}: \"{entry}\".to_string(),"), 
				_ => println!("            {heading}: {entry},"),
			}
		}

		println!("      }}");

		println!("}}\n");
	}*/
}
