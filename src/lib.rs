#[derive(Debug, PartialEq)]
pub struct Element {
	atomic_number: u8, 
	name: String, 
	symbol: String, 
	atomic_mass: f64, 
	electronegativity: f64, 
}

impl Element {
	pub fn atomic_number(&self) -> u8 {
		self.atomic_number
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn symbol(&self) -> &str {
		&self.symbol
	}

	pub fn atomic_mass(&self) -> f64 {
		self.atomic_mass
	}

	pub fn electronegativity(&self) -> f64 {
		self.electronegativity
	}
}

impl std::cmp::Eq for Element { }

impl std::hash::Hash for Element {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

// 1	, , , 	Group 1A and ammonium compounds are soluble.	–
// 2	, 	Acetates and nitrates are soluble.	–
// 3	, , 	Most chlorides, bromides, and iodides are soluble.	
// , , , , , , , , , , 
// 4		Most sulfates are soluble.	
// , , , , , 
// 5		Most carbonates are insoluble.	Group 1A carbonates, 
// 6		Most phosphates are insoluble.	Group 1A phosphates, 
// 7		Most sulfides are insoluble.	Group 1A sulfides, 
// 8		Most hydroxides are insoluble.	Group 1A hydroxides, , , 


// pub struct Ion {
// 	elements: Molecule, 
// 	charge: isize, 
// }

// impl Ion {
// 	pub fn new(element: Molecule, charge: isize) -> Self {

// struct Ions;

// impl Ions {
// 	pub fn ammonium() -> Ion {
		

// pub struct IonicCompound {
// 	cation: Ion, 
// 	anion: Ion, 
// }

// impl IonicCompound {
// 	pub fn is_soluble_in_water(&self) -> bool {
// 		match self {
// 			IonicCompound { Ion { elements: cation, charge }, anion } if cation == 


use std::collections::HashMap;

//pub const SYMBOL_TO_CONSTRUCTOR: HashMap<&str, fn() -> Element> = [
	
//].iter().cloned().collect();

use std::iter::Iterator;

pub struct Elements;

impl Elements {
	pub fn elements() -> impl Iterator<Item = fn() -> Element> {
		let elements = [
			Self::hydrogen,
			Self::helium,
			Self::lithium,
			Self::beryllium,
			Self::boron,
			Self::carbon,
			Self::nitrogen,
			Self::oxygen,
			Self::fluorine,
			Self::neon,
			Self::sodium,
			Self::magnesium,
			Self::aluminum,
			Self::silicon,
			Self::phosphorus,
			Self::sulfur,
			Self::chlorine,
			Self::argon,
			Self::potassium,
			Self::calcium,
			Self::scandium,
			Self::titanium,
			Self::vanadium,
			Self::chromium,
			Self::manganese,
			Self::iron,
			Self::cobalt,
			Self::nickel,
			Self::copper,
			Self::zinc,
			Self::gallium,
			Self::germanium,
			Self::arsenic,
			Self::selenium,
			Self::bromine,
			Self::krypton,
			Self::rubidium,
			Self::strontium,
			Self::yttrium,
			Self::zirconium,
			Self::niobium,
			Self::molybdenum,
			Self::technetium,
			Self::ruthenium,
			Self::rhodium,
			Self::palladium,
			Self::silver,
			Self::cadmium,
			Self::indium,
			Self::tin,
			Self::antimony,
			Self::tellurium,
			Self::iodine,
			Self::xenon,
			Self::cesium,
			Self::barium,
			Self::lanthanum,
			Self::cerium,
			Self::praseodymium,
			Self::neodymium,
			Self::promethium,
			Self::samarium,
			Self::europium,
			Self::gadolinium,
			Self::terbium,
			Self::dysprosium,
			Self::holmium,
			Self::erbium,
			Self::thulium,
			Self::ytterbium,
			Self::lutetium,
			Self::hafnium,
			Self::tantalum,
			Self::wolfram,
			Self::rhenium,
			Self::osmium,
			Self::iridium,
			Self::platinum,
			Self::gold,
			Self::mercury,
			Self::thallium,
			Self::lead,
			Self::bismuth,
			Self::polonium,
			Self::astatine,
			Self::radon,
			Self::francium,
			Self::radium,
			Self::actinium,
			Self::thorium,
			Self::protactinium,
			Self::uranium,
			Self::neptunium,
			Self::plutonium,
			Self::americium,
			Self::curium,
			Self::berkelium,
			Self::californium,
			Self::einsteinium,
			Self::fermium,
			Self::mendelevium,
			Self::nobelium,
			Self::lawrencium,
			Self::rutherfordium,
			Self::dubnium,
			Self::seaborgium,
			Self::bohrium,
			Self::hassium,
			Self::meitnerium,
			Self::darmstadtium,
			Self::roentgenium,
			Self::copernicium,
			Self::nihonium,
			Self::flerovium,
			Self::moscovium,
			Self::livermorium,
			Self::tennessine,
			Self::oganesson,
		];

		elements.into_iter()
	}

	pub fn as_hashmap() -> HashMap<String, fn() -> Element> {
		let mut elements = HashMap::new();

		for element_constructor in Self::elements() {
			let element = element_constructor();
			elements.insert(String::from(element.symbol()), element_constructor);
		}

		elements
	}

	pub fn hydrogen() -> Element {
	      Element {
	            atomic_number: 1,
	            name: "Hydrogen".to_string(),
	            symbol: "H".to_string(),
	            atomic_mass: 1.007,
	            electronegativity: 2.2,
	      }
	}

	pub fn helium() -> Element {
	      Element {
	            atomic_number: 2,
	            name: "Helium".to_string(),
	            symbol: "He".to_string(),
	            atomic_mass: 4.002,
	            electronegativity: 0.0,
	      }
	}

	pub fn lithium() -> Element {
	      Element {
	            atomic_number: 3,
	            name: "Lithium".to_string(),
	            symbol: "Li".to_string(),
	            atomic_mass: 6.941,
	            electronegativity: 0.98,
	      }
	}

	pub fn beryllium() -> Element {
	      Element {
	            atomic_number: 4,
	            name: "Beryllium".to_string(),
	            symbol: "Be".to_string(),
	            atomic_mass: 9.012,
	            electronegativity: 1.57,
	      }
	}

	pub fn boron() -> Element {
	      Element {
	            atomic_number: 5,
	            name: "Boron".to_string(),
	            symbol: "B".to_string(),
	            atomic_mass: 10.811,
	            electronegativity: 2.04,
	      }
	}

	pub fn carbon() -> Element {
	      Element {
	            atomic_number: 6,
	            name: "Carbon".to_string(),
	            symbol: "C".to_string(),
	            atomic_mass: 12.011,
	            electronegativity: 2.55,
	      }
	}

	pub fn nitrogen() -> Element {
	      Element {
	            atomic_number: 7,
	            name: "Nitrogen".to_string(),
	            symbol: "N".to_string(),
	            atomic_mass: 14.007,
	            electronegativity: 3.04,
	      }
	}

	pub fn oxygen() -> Element {
	      Element {
	            atomic_number: 8,
	            name: "Oxygen".to_string(),
	            symbol: "O".to_string(),
	            atomic_mass: 15.999,
	            electronegativity: 3.44,
	      }
	}

	pub fn fluorine() -> Element {
	      Element {
	            atomic_number: 9,
	            name: "Fluorine".to_string(),
	            symbol: "F".to_string(),
	            atomic_mass: 18.998,
	            electronegativity: 3.98,
	      }
	}

	pub fn neon() -> Element {
	      Element {
	            atomic_number: 10,
	            name: "Neon".to_string(),
	            symbol: "Ne".to_string(),
	            atomic_mass: 20.18,
	            electronegativity: 0.0,
	      }
	}

	pub fn sodium() -> Element {
	      Element {
	            atomic_number: 11,
	            name: "Sodium".to_string(),
	            symbol: "Na".to_string(),
	            atomic_mass: 22.99,
	            electronegativity: 0.93,
	      }
	}

	pub fn magnesium() -> Element {
	      Element {
	            atomic_number: 12,
	            name: "Magnesium".to_string(),
	            symbol: "Mg".to_string(),
	            atomic_mass: 24.305,
	            electronegativity: 1.31,
	      }
	}

	pub fn aluminum() -> Element {
	      Element {
	            atomic_number: 13,
	            name: "Aluminum".to_string(),
	            symbol: "Al".to_string(),
	            atomic_mass: 26.982,
	            electronegativity: 1.61,
	      }
	}

	pub fn silicon() -> Element {
	      Element {
	            atomic_number: 14,
	            name: "Silicon".to_string(),
	            symbol: "Si".to_string(),
	            atomic_mass: 28.086,
	            electronegativity: 1.9,
	      }
	}

	pub fn phosphorus() -> Element {
	      Element {
	            atomic_number: 15,
	            name: "Phosphorus".to_string(),
	            symbol: "P".to_string(),
	            atomic_mass: 30.974,
	            electronegativity: 2.19,
	      }
	}

	pub fn sulfur() -> Element {
	      Element {
	            atomic_number: 16,
	            name: "Sulfur".to_string(),
	            symbol: "S".to_string(),
	            atomic_mass: 32.065,
	            electronegativity: 2.58,
	      }
	}

	pub fn chlorine() -> Element {
	      Element {
	            atomic_number: 17,
	            name: "Chlorine".to_string(),
	            symbol: "Cl".to_string(),
	            atomic_mass: 35.453,
	            electronegativity: 3.16,
	      }
	}

	pub fn argon() -> Element {
	      Element {
	            atomic_number: 18,
	            name: "Argon".to_string(),
	            symbol: "Ar".to_string(),
	            atomic_mass: 39.948,
	            electronegativity: 0.0,
	      }
	}

	pub fn potassium() -> Element {
	      Element {
	            atomic_number: 19,
	            name: "Potassium".to_string(),
	            symbol: "K".to_string(),
	            atomic_mass: 39.098,
	            electronegativity: 0.82,
	      }
	}

	pub fn calcium() -> Element {
	      Element {
	            atomic_number: 20,
	            name: "Calcium".to_string(),
	            symbol: "Ca".to_string(),
	            atomic_mass: 40.078,
	            electronegativity: 1.0,
	      }
	}

	pub fn scandium() -> Element {
	      Element {
	            atomic_number: 21,
	            name: "Scandium".to_string(),
	            symbol: "Sc".to_string(),
	            atomic_mass: 44.956,
	            electronegativity: 1.36,
	      }
	}

	pub fn titanium() -> Element {
	      Element {
	            atomic_number: 22,
	            name: "Titanium".to_string(),
	            symbol: "Ti".to_string(),
	            atomic_mass: 47.867,
	            electronegativity: 1.54,
	      }
	}

	pub fn vanadium() -> Element {
	      Element {
	            atomic_number: 23,
	            name: "Vanadium".to_string(),
	            symbol: "V".to_string(),
	            atomic_mass: 50.942,
	            electronegativity: 1.63,
	      }
	}

	pub fn chromium() -> Element {
	      Element {
	            atomic_number: 24,
	            name: "Chromium".to_string(),
	            symbol: "Cr".to_string(),
	            atomic_mass: 51.996,
	            electronegativity: 1.66,
	      }
	}

	pub fn manganese() -> Element {
	      Element {
	            atomic_number: 25,
	            name: "Manganese".to_string(),
	            symbol: "Mn".to_string(),
	            atomic_mass: 54.938,
	            electronegativity: 1.55,
	      }
	}

	pub fn iron() -> Element {
	      Element {
	            atomic_number: 26,
	            name: "Iron".to_string(),
	            symbol: "Fe".to_string(),
	            atomic_mass: 55.845,
	            electronegativity: 1.83,
	      }
	}

	pub fn cobalt() -> Element {
	      Element {
	            atomic_number: 27,
	            name: "Cobalt".to_string(),
	            symbol: "Co".to_string(),
	            atomic_mass: 58.933,
	            electronegativity: 1.88,
	      }
	}

	pub fn nickel() -> Element {
	      Element {
	            atomic_number: 28,
	            name: "Nickel".to_string(),
	            symbol: "Ni".to_string(),
	            atomic_mass: 58.693,
	            electronegativity: 1.91,
	      }
	}

	pub fn copper() -> Element {
	      Element {
	            atomic_number: 29,
	            name: "Copper".to_string(),
	            symbol: "Cu".to_string(),
	            atomic_mass: 63.546,
	            electronegativity: 1.9,
	      }
	}

	pub fn zinc() -> Element {
	      Element {
	            atomic_number: 30,
	            name: "Zinc".to_string(),
	            symbol: "Zn".to_string(),
	            atomic_mass: 65.38,
	            electronegativity: 1.65,
	      }
	}

	pub fn gallium() -> Element {
	      Element {
	            atomic_number: 31,
	            name: "Gallium".to_string(),
	            symbol: "Ga".to_string(),
	            atomic_mass: 69.723,
	            electronegativity: 1.81,
	      }
	}

	pub fn germanium() -> Element {
	      Element {
	            atomic_number: 32,
	            name: "Germanium".to_string(),
	            symbol: "Ge".to_string(),
	            atomic_mass: 72.64,
	            electronegativity: 2.01,
	      }
	}

	pub fn arsenic() -> Element {
	      Element {
	            atomic_number: 33,
	            name: "Arsenic".to_string(),
	            symbol: "As".to_string(),
	            atomic_mass: 74.922,
	            electronegativity: 2.18,
	      }
	}

	pub fn selenium() -> Element {
	      Element {
	            atomic_number: 34,
	            name: "Selenium".to_string(),
	            symbol: "Se".to_string(),
	            atomic_mass: 78.96,
	            electronegativity: 2.55,
	      }
	}

	pub fn bromine() -> Element {
	      Element {
	            atomic_number: 35,
	            name: "Bromine".to_string(),
	            symbol: "Br".to_string(),
	            atomic_mass: 79.904,
	            electronegativity: 2.96,
	      }
	}

	pub fn krypton() -> Element {
	      Element {
	            atomic_number: 36,
	            name: "Krypton".to_string(),
	            symbol: "Kr".to_string(),
	            atomic_mass: 83.798,
	            electronegativity: 0.0,
	      }
	}

	pub fn rubidium() -> Element {
	      Element {
	            atomic_number: 37,
	            name: "Rubidium".to_string(),
	            symbol: "Rb".to_string(),
	            atomic_mass: 85.468,
	            electronegativity: 0.82,
	      }
	}

	pub fn strontium() -> Element {
	      Element {
	            atomic_number: 38,
	            name: "Strontium".to_string(),
	            symbol: "Sr".to_string(),
	            atomic_mass: 87.62,
	            electronegativity: 0.95,
	      }
	}

	pub fn yttrium() -> Element {
	      Element {
	            atomic_number: 39,
	            name: "Yttrium".to_string(),
	            symbol: "Y".to_string(),
	            atomic_mass: 88.906,
	            electronegativity: 1.22,
	      }
	}

	pub fn zirconium() -> Element {
	      Element {
	            atomic_number: 40,
	            name: "Zirconium".to_string(),
	            symbol: "Zr".to_string(),
	            atomic_mass: 91.224,
	            electronegativity: 1.33,
	      }
	}

	pub fn niobium() -> Element {
	      Element {
	            atomic_number: 41,
	            name: "Niobium".to_string(),
	            symbol: "Nb".to_string(),
	            atomic_mass: 92.906,
	            electronegativity: 1.6,
	      }
	}

	pub fn molybdenum() -> Element {
	      Element {
	            atomic_number: 42,
	            name: "Molybdenum".to_string(),
	            symbol: "Mo".to_string(),
	            atomic_mass: 95.96,
	            electronegativity: 2.16,
	      }
	}

	pub fn technetium() -> Element {
	      Element {
	            atomic_number: 43,
	            name: "Technetium".to_string(),
	            symbol: "Tc".to_string(),
	            atomic_mass: 98.0,
	            electronegativity: 1.9,
	      }
	}

	pub fn ruthenium() -> Element {
	      Element {
	            atomic_number: 44,
	            name: "Ruthenium".to_string(),
	            symbol: "Ru".to_string(),
	            atomic_mass: 101.07,
	            electronegativity: 2.2,
	      }
	}

	pub fn rhodium() -> Element {
	      Element {
	            atomic_number: 45,
	            name: "Rhodium".to_string(),
	            symbol: "Rh".to_string(),
	            atomic_mass: 102.906,
	            electronegativity: 2.28,
	      }
	}

	pub fn palladium() -> Element {
	      Element {
	            atomic_number: 46,
	            name: "Palladium".to_string(),
	            symbol: "Pd".to_string(),
	            atomic_mass: 106.42,
	            electronegativity: 2.2,
	      }
	}

	pub fn silver() -> Element {
	      Element {
	            atomic_number: 47,
	            name: "Silver".to_string(),
	            symbol: "Ag".to_string(),
	            atomic_mass: 107.868,
	            electronegativity: 1.93,
	      }
	}

	pub fn cadmium() -> Element {
	      Element {
	            atomic_number: 48,
	            name: "Cadmium".to_string(),
	            symbol: "Cd".to_string(),
	            atomic_mass: 112.411,
	            electronegativity: 1.69,
	      }
	}

	pub fn indium() -> Element {
	      Element {
	            atomic_number: 49,
	            name: "Indium".to_string(),
	            symbol: "In".to_string(),
	            atomic_mass: 114.818,
	            electronegativity: 1.78,
	      }
	}

	pub fn tin() -> Element {
	      Element {
	            atomic_number: 50,
	            name: "Tin".to_string(),
	            symbol: "Sn".to_string(),
	            atomic_mass: 118.71,
	            electronegativity: 1.96,
	      }
	}

	pub fn antimony() -> Element {
	      Element {
	            atomic_number: 51,
	            name: "Antimony".to_string(),
	            symbol: "Sb".to_string(),
	            atomic_mass: 121.76,
	            electronegativity: 2.05,
	      }
	}

	pub fn tellurium() -> Element {
	      Element {
	            atomic_number: 52,
	            name: "Tellurium".to_string(),
	            symbol: "Te".to_string(),
	            atomic_mass: 127.6,
	            electronegativity: 2.1,
	      }
	}

	pub fn iodine() -> Element {
	      Element {
	            atomic_number: 53,
	            name: "Iodine".to_string(),
	            symbol: "I".to_string(),
	            atomic_mass: 126.904,
	            electronegativity: 2.66,
	      }
	}

	pub fn xenon() -> Element {
	      Element {
	            atomic_number: 54,
	            name: "Xenon".to_string(),
	            symbol: "Xe".to_string(),
	            atomic_mass: 131.293,
	            electronegativity: 0.0,
	      }
	}

	pub fn cesium() -> Element {
	      Element {
	            atomic_number: 55,
	            name: "Cesium".to_string(),
	            symbol: "Cs".to_string(),
	            atomic_mass: 132.905,
	            electronegativity: 0.79,
	      }
	}

	pub fn barium() -> Element {
	      Element {
	            atomic_number: 56,
	            name: "Barium".to_string(),
	            symbol: "Ba".to_string(),
	            atomic_mass: 137.327,
	            electronegativity: 0.89,
	      }
	}

	pub fn lanthanum() -> Element {
	      Element {
	            atomic_number: 57,
	            name: "Lanthanum".to_string(),
	            symbol: "La".to_string(),
	            atomic_mass: 138.905,
	            electronegativity: 1.1,
	      }
	}

	pub fn cerium() -> Element {
	      Element {
	            atomic_number: 58,
	            name: "Cerium".to_string(),
	            symbol: "Ce".to_string(),
	            atomic_mass: 140.116,
	            electronegativity: 1.12,
	      }
	}

	pub fn praseodymium() -> Element {
	      Element {
	            atomic_number: 59,
	            name: "Praseodymium".to_string(),
	            symbol: "Pr".to_string(),
	            atomic_mass: 140.908,
	            electronegativity: 1.13,
	      }
	}

	pub fn neodymium() -> Element {
	      Element {
	            atomic_number: 60,
	            name: "Neodymium".to_string(),
	            symbol: "Nd".to_string(),
	            atomic_mass: 144.242,
	            electronegativity: 1.14,
	      }
	}

	pub fn promethium() -> Element {
	      Element {
	            atomic_number: 61,
	            name: "Promethium".to_string(),
	            symbol: "Pm".to_string(),
	            atomic_mass: 145.0,
	            electronegativity: 1.13,
	      }
	}

	pub fn samarium() -> Element {
	      Element {
	            atomic_number: 62,
	            name: "Samarium".to_string(),
	            symbol: "Sm".to_string(),
	            atomic_mass: 150.36,
	            electronegativity: 1.17,
	      }
	}

	pub fn europium() -> Element {
	      Element {
	            atomic_number: 63,
	            name: "Europium".to_string(),
	            symbol: "Eu".to_string(),
	            atomic_mass: 151.964,
	            electronegativity: 1.2,
	      }
	}

	pub fn gadolinium() -> Element {
	      Element {
	            atomic_number: 64,
	            name: "Gadolinium".to_string(),
	            symbol: "Gd".to_string(),
	            atomic_mass: 157.25,
	            electronegativity: 1.2,
	      }
	}

	pub fn terbium() -> Element {
	      Element {
	            atomic_number: 65,
	            name: "Terbium".to_string(),
	            symbol: "Tb".to_string(),
	            atomic_mass: 158.925,
	            electronegativity: 1.2,
	      }
	}

	pub fn dysprosium() -> Element {
	      Element {
	            atomic_number: 66,
	            name: "Dysprosium".to_string(),
	            symbol: "Dy".to_string(),
	            atomic_mass: 162.5,
	            electronegativity: 1.22,
	      }
	}

	pub fn holmium() -> Element {
	      Element {
	            atomic_number: 67,
	            name: "Holmium".to_string(),
	            symbol: "Ho".to_string(),
	            atomic_mass: 164.93,
	            electronegativity: 1.23,
	      }
	}

	pub fn erbium() -> Element {
	      Element {
	            atomic_number: 68,
	            name: "Erbium".to_string(),
	            symbol: "Er".to_string(),
	            atomic_mass: 167.259,
	            electronegativity: 1.24,
	      }
	}

	pub fn thulium() -> Element {
	      Element {
	            atomic_number: 69,
	            name: "Thulium".to_string(),
	            symbol: "Tm".to_string(),
	            atomic_mass: 168.934,
	            electronegativity: 1.25,
	      }
	}

	pub fn ytterbium() -> Element {
	      Element {
	            atomic_number: 70,
	            name: "Ytterbium".to_string(),
	            symbol: "Yb".to_string(),
	            atomic_mass: 173.054,
	            electronegativity: 1.1,
	      }
	}

	pub fn lutetium() -> Element {
	      Element {
	            atomic_number: 71,
	            name: "Lutetium".to_string(),
	            symbol: "Lu".to_string(),
	            atomic_mass: 174.967,
	            electronegativity: 1.27,
	      }
	}

	pub fn hafnium() -> Element {
	      Element {
	            atomic_number: 72,
	            name: "Hafnium".to_string(),
	            symbol: "Hf".to_string(),
	            atomic_mass: 178.49,
	            electronegativity: 1.3,
	      }
	}

	pub fn tantalum() -> Element {
	      Element {
	            atomic_number: 73,
	            name: "Tantalum".to_string(),
	            symbol: "Ta".to_string(),
	            atomic_mass: 180.948,
	            electronegativity: 1.5,
	      }
	}

	pub fn wolfram() -> Element {
	      Element {
	            atomic_number: 74,
	            name: "Wolfram".to_string(),
	            symbol: "W".to_string(),
	            atomic_mass: 183.84,
	            electronegativity: 2.36,
	      }
	}

	pub fn rhenium() -> Element {
	      Element {
	            atomic_number: 75,
	            name: "Rhenium".to_string(),
	            symbol: "Re".to_string(),
	            atomic_mass: 186.207,
	            electronegativity: 1.9,
	      }
	}

	pub fn osmium() -> Element {
	      Element {
	            atomic_number: 76,
	            name: "Osmium".to_string(),
	            symbol: "Os".to_string(),
	            atomic_mass: 190.23,
	            electronegativity: 2.2,
	      }
	}

	pub fn iridium() -> Element {
	      Element {
	            atomic_number: 77,
	            name: "Iridium".to_string(),
	            symbol: "Ir".to_string(),
	            atomic_mass: 192.217,
	            electronegativity: 2.2,
	      }
	}

	pub fn platinum() -> Element {
	      Element {
	            atomic_number: 78,
	            name: "Platinum".to_string(),
	            symbol: "Pt".to_string(),
	            atomic_mass: 195.084,
	            electronegativity: 2.28,
	      }
	}

	pub fn gold() -> Element {
	      Element {
	            atomic_number: 79,
	            name: "Gold".to_string(),
	            symbol: "Au".to_string(),
	            atomic_mass: 196.967,
	            electronegativity: 2.54,
	      }
	}

	pub fn mercury() -> Element {
	      Element {
	            atomic_number: 80,
	            name: "Mercury".to_string(),
	            symbol: "Hg".to_string(),
	            atomic_mass: 200.59,
	            electronegativity: 2.0,
	      }
	}

	pub fn thallium() -> Element {
	      Element {
	            atomic_number: 81,
	            name: "Thallium".to_string(),
	            symbol: "Tl".to_string(),
	            atomic_mass: 204.383,
	            electronegativity: 2.04,
	      }
	}

	pub fn lead() -> Element {
	      Element {
	            atomic_number: 82,
	            name: "Lead".to_string(),
	            symbol: "Pb".to_string(),
	            atomic_mass: 207.2,
	            electronegativity: 2.33,
	      }
	}

	pub fn bismuth() -> Element {
	      Element {
	            atomic_number: 83,
	            name: "Bismuth".to_string(),
	            symbol: "Bi".to_string(),
	            atomic_mass: 208.98,
	            electronegativity: 2.02,
	      }
	}

	pub fn polonium() -> Element {
	      Element {
	            atomic_number: 84,
	            name: "Polonium".to_string(),
	            symbol: "Po".to_string(),
	            atomic_mass: 210.0,
	            electronegativity: 2.0,
	      }
	}

	pub fn astatine() -> Element {
	      Element {
	            atomic_number: 85,
	            name: "Astatine".to_string(),
	            symbol: "At".to_string(),
	            atomic_mass: 210.0,
	            electronegativity: 2.2,
	      }
	}

	pub fn radon() -> Element {
	      Element {
	            atomic_number: 86,
	            name: "Radon".to_string(),
	            symbol: "Rn".to_string(),
	            atomic_mass: 222.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn francium() -> Element {
	      Element {
	            atomic_number: 87,
	            name: "Francium".to_string(),
	            symbol: "Fr".to_string(),
	            atomic_mass: 223.0,
	            electronegativity: 0.7,
	      }
	}

	pub fn radium() -> Element {
	      Element {
	            atomic_number: 88,
	            name: "Radium".to_string(),
	            symbol: "Ra".to_string(),
	            atomic_mass: 226.0,
	            electronegativity: 0.9,
	      }
	}

	pub fn actinium() -> Element {
	      Element {
	            atomic_number: 89,
	            name: "Actinium".to_string(),
	            symbol: "Ac".to_string(),
	            atomic_mass: 227.0,
	            electronegativity: 1.1,
	      }
	}

	pub fn thorium() -> Element {
	      Element {
	            atomic_number: 90,
	            name: "Thorium".to_string(),
	            symbol: "Th".to_string(),
	            atomic_mass: 232.038,
	            electronegativity: 1.3,
	      }
	}

	pub fn protactinium() -> Element {
	      Element {
	            atomic_number: 91,
	            name: "Protactinium".to_string(),
	            symbol: "Pa".to_string(),
	            atomic_mass: 231.036,
	            electronegativity: 1.5,
	      }
	}

	pub fn uranium() -> Element {
	      Element {
	            atomic_number: 92,
	            name: "Uranium".to_string(),
	            symbol: "U".to_string(),
	            atomic_mass: 238.029,
	            electronegativity: 1.38,
	      }
	}

	pub fn neptunium() -> Element {
	      Element {
	            atomic_number: 93,
	            name: "Neptunium".to_string(),
	            symbol: "Np".to_string(),
	            atomic_mass: 237.0,
	            electronegativity: 1.36,
	      }
	}

	pub fn plutonium() -> Element {
	      Element {
	            atomic_number: 94,
	            name: "Plutonium".to_string(),
	            symbol: "Pu".to_string(),
	            atomic_mass: 244.0,
	            electronegativity: 1.28,
	      }
	}

	pub fn americium() -> Element {
	      Element {
	            atomic_number: 95,
	            name: "Americium".to_string(),
	            symbol: "Am".to_string(),
	            atomic_mass: 243.0,
	            electronegativity: 1.3,
	      }
	}

	pub fn curium() -> Element {
	      Element {
	            atomic_number: 96,
	            name: "Curium".to_string(),
	            symbol: "Cm".to_string(),
	            atomic_mass: 247.0,
	            electronegativity: 1.3,
	      }
	}

	pub fn berkelium() -> Element {
	      Element {
	            atomic_number: 97,
	            name: "Berkelium".to_string(),
	            symbol: "Bk".to_string(),
	            atomic_mass: 247.0,
	            electronegativity: 1.3,
	      }
	}

	pub fn californium() -> Element {
	      Element {
	            atomic_number: 98,
	            name: "Californium".to_string(),
	            symbol: "Cf".to_string(),
	            atomic_mass: 251.0,
	            electronegativity: 1.3,
	      }
	}

	pub fn einsteinium() -> Element {
	      Element {
	            atomic_number: 99,
	            name: "Einsteinium".to_string(),
	            symbol: "Es".to_string(),
	            atomic_mass: 252.0,
	            electronegativity: 1.3,
	      }
	}

	pub fn fermium() -> Element {
	      Element {
	            atomic_number: 100,
	            name: "Fermium".to_string(),
	            symbol: "Fm".to_string(),
	            atomic_mass: 257.0,
	            electronegativity: 1.3,
	      }
	}

	pub fn mendelevium() -> Element {
	      Element {
	            atomic_number: 101,
	            name: "Mendelevium".to_string(),
	            symbol: "Md".to_string(),
	            atomic_mass: 258.0,
	            electronegativity: 1.3,
	      }
	}

	pub fn nobelium() -> Element {
	      Element {
	            atomic_number: 102,
	            name: "Nobelium".to_string(),
	            symbol: "No".to_string(),
	            atomic_mass: 259.0,
	            electronegativity: 1.3,
	      }
	}

	pub fn lawrencium() -> Element {
	      Element {
	            atomic_number: 103,
	            name: "Lawrencium".to_string(),
	            symbol: "Lr".to_string(),
	            atomic_mass: 262.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn rutherfordium() -> Element {
	      Element {
	            atomic_number: 104,
	            name: "Rutherfordium".to_string(),
	            symbol: "Rf".to_string(),
	            atomic_mass: 261.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn dubnium() -> Element {
	      Element {
	            atomic_number: 105,
	            name: "Dubnium".to_string(),
	            symbol: "Db".to_string(),
	            atomic_mass: 262.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn seaborgium() -> Element {
	      Element {
	            atomic_number: 106,
	            name: "Seaborgium".to_string(),
	            symbol: "Sg".to_string(),
	            atomic_mass: 266.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn bohrium() -> Element {
	      Element {
	            atomic_number: 107,
	            name: "Bohrium".to_string(),
	            symbol: "Bh".to_string(),
	            atomic_mass: 264.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn hassium() -> Element {
	      Element {
	            atomic_number: 108,
	            name: "Hassium".to_string(),
	            symbol: "Hs".to_string(),
	            atomic_mass: 267.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn meitnerium() -> Element {
	      Element {
	            atomic_number: 109,
	            name: "Meitnerium".to_string(),
	            symbol: "Mt".to_string(),
	            atomic_mass: 268.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn darmstadtium() -> Element {
	      Element {
	            atomic_number: 110,
	            name: "Darmstadtium".to_string(),
	            symbol: "Ds".to_string(),
	            atomic_mass: 271.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn roentgenium() -> Element {
	      Element {
	            atomic_number: 111,
	            name: "Roentgenium".to_string(),
	            symbol: "Rg".to_string(),
	            atomic_mass: 272.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn copernicium() -> Element {
	      Element {
	            atomic_number: 112,
	            name: "Copernicium".to_string(),
	            symbol: "Cn".to_string(),
	            atomic_mass: 285.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn nihonium() -> Element {
	      Element {
	            atomic_number: 113,
	            name: "Nihonium".to_string(),
	            symbol: "Nh".to_string(),
	            atomic_mass: 284.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn flerovium() -> Element {
	      Element {
	            atomic_number: 114,
	            name: "Flerovium".to_string(),
	            symbol: "Fl".to_string(),
	            atomic_mass: 289.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn moscovium() -> Element {
	      Element {
	            atomic_number: 115,
	            name: "Moscovium".to_string(),
	            symbol: "Mc".to_string(),
	            atomic_mass: 288.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn livermorium() -> Element {
	      Element {
	            atomic_number: 116,
	            name: "Livermorium".to_string(),
	            symbol: "Lv".to_string(),
	            atomic_mass: 292.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn tennessine() -> Element {
	      Element {
	            atomic_number: 117,
	            name: "Tennessine".to_string(),
	            symbol: "Ts".to_string(),
	            atomic_mass: 295.0,
	            electronegativity: 0.0,
	      }
	}

	pub fn oganesson() -> Element {
	      Element {
	            atomic_number: 118,
	            name: "Oganesson".to_string(),
	            symbol: "Og".to_string(),
	            atomic_mass: 294.0,
	            electronegativity: 0.0,
	      }
	}
}
