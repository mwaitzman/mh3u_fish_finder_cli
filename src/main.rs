//#![warn(clippy::pedantic)]
use std::{str::FromStr, fmt::Display};

#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type unsigned = u32;
#[allow(non_camel_case_types)]
type array<t> = Vec<t>;
#[allow(non_camel_case_types)]
type long__long = i64;


fn rnd(n: int) -> int {
	if n == 0 { return 176; }

	let x: int = n * 16 - n * 4 - n;
	let y: long__long = (x * 16).into();
	let z: long__long = y * 11367737 / 4294967296;
	let m: long__long = y - ( (y-z) / 2 + z) / 32768 * 65363;
	(m & 0xFFFF) as int
}

fn get_fish_index(roll: unsigned, fish: &array<unsigned>) -> unsigned {
	let mut total: unsigned = fish[0];
	let mut count: unsigned = 0;
	while total <= roll {
		count += 1;
		total += fish[count as usize];
	}
	count
}

fn reward_func(mut num: unsigned, rep: unsigned) -> unsigned {
	for _ in 0..rep {
		num = (num * 176) % 65363;
	}
	num
}

fn array_equals(a: &array<unsigned>, r#as: unsigned, b: &array<unsigned>, bs: unsigned, len: unsigned) -> bool {
	(0..len).all(|i| a[(r#as + i) as usize] == b[(bs + i) as usize])
}

fn search_seed(seed: unsigned, fish_table: &array<unsigned>, items: &array<unsigned>) -> bool {
	//irre: I have no idea what qwe is supposed to mean. I'm assuming AthenaADP was just being a lazy QWERTY user :|
	let qwe: array<unsigned> = (1..14).map(|j| {
		let roll: unsigned = reward_func(seed, j.try_into().unwrap()) % 100;
		get_fish_index(roll, fish_table)
	}).collect();

	let max_fishfinder: unsigned = (std::cmp::Ord::clamp(items.len(), 4, 7) - 4).try_into().unwrap();
	(0..=max_fishfinder).any(|i: unsigned| {
		array_equals(&qwe, 0, items, 0, (items.len() - (i as usize)).try_into().unwrap())
			&& array_equals(&qwe, 10, items, (items.len() - (i as usize)).try_into().unwrap(), i)
	})
}

fn check_seed(seed: int, fish_table: &array<unsigned>, items: &array<unsigned>) -> bool {
	let mut n: int = seed;
	loop {
		if search_seed(n.try_into().unwrap(), fish_table, items) { return true; };

		n = rnd(n);

		if n == seed { return false; }
	}
}

fn get_table_results(items: &array<unsigned>) -> unsigned {

	let table_seeds: array<int> = vec![1, 15, 5, 13, 4, 3, 9, 12, 26, 18, 163, 401, 6, 2, 489, 802, 1203];
	let fish: array<unsigned> = vec![20, 15, 5, 5, 10, 15, 5, 10, 5, 5, 5];

	(0..17)
		.filter(|t| check_seed(table_seeds[*t as usize], &fish, items))
		.fold(0, |acc, e| acc | 1 << e)
}


#[derive(Debug,PartialEq)]
enum FishRewards {
	ShiningStarfishx1 = 0,
	PinTunax2 = 1,
	ShiningStarfishx2 = 2,
	Sushifishx2 = 3,
	Popfishx5 = 4,
	Sushifishx3 = 5,
	Whetfishx6 = 6,
	Sleepyfishx2 = 7,
	Popfishx4 = 8,
	Whetfishx4 = 9,
	HumspunConchx1 = 10,
}
impl FromStr for FishRewards {
	type Err = String;
	fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
		use FishRewards::*;
		match input {
			"shiningstarfishx1" | "shiningstarfish" => Ok(ShiningStarfishx1),
			"pintunax2" => Ok(PinTunax2),
			"shiningstarfishx2" => Ok(ShiningStarfishx2),
			"sushifishx2" => Ok(Sushifishx2),
			"popfishx5" => Ok(Popfishx5),
			"sushifishx3" => Ok(Sushifishx3),
			"whetfishx6" => Ok(Whetfishx6),
			"sleepyfishx2" => Ok(Sleepyfishx2),
			"popfishx4" => Ok(Popfishx4),
			"whetfishx4" => Ok(Whetfishx4),
			"humspunconchx1" | "humspunconch" => Ok(HumspunConchx1),
			_ => Err(format!("Couldn't parse {input} into a FishRewards"))
		}
	}
}
impl Display for FishRewards {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		use FishRewards::*;
		write!(
			f,
			"{}",
			match self {
				ShiningStarfishx1 => "Shining Starfish x1",
				PinTunax2 => "Pin Tuna x2",
				ShiningStarfishx2 => "Shining Starfish x2",
				Sushifishx2 => "Sushifish x2",
				Popfishx5 => "Popfish x5",
				Sushifishx3 => "Sushifish x3",
				Whetfishx6 => "Whetfish x6",
				Sleepyfishx2 => "Sleepyfish x2",
				Popfishx4 => "Popfish x4",
				Whetfishx4 => "Whetfish x4",
				HumspunConchx1 => "Humspun Conch x1",
			}
		)
	}
}

fn normalize_row_input(row: &str) -> Vec<FishRewards> {
	row
		.split(',')
		.map(|e| e.split_whitespace().collect::<String>())
		.filter(|e| !e.is_empty())
		.map(|e| e.to_lowercase())
		.map(|e| e.parse().unwrap_or_else(|err| panic!("{err}")))
		.collect()
}

fn main() {
	let mut line = String::new();
	const ALL_TABLES_POSSIBLE: unsigned = u32::MAX >> (u32::BITS - 17);
	let mut possible_tables = ALL_TABLES_POSSIBLE;
	while 0 != std::io::stdin().read_line(&mut line).unwrap() {
		if line == "\n" {
			if possible_tables != ALL_TABLES_POSSIBLE {
				println!("\n\npossible tables reset");
			} else { possible_tables = ALL_TABLES_POSSIBLE; }
			continue;
		}

		let my_items: array<unsigned> = normalize_row_input(&line)
			.into_iter()
			.map(|e| e as unsigned)
			.collect();
		let packed_result = get_table_results(&my_items);
		println!("possible tables from this row: {}",
				 (0..17)
				 .filter(|i| packed_result & (1 << i) == 1 << i)
				 .fold(String::new(), |acc, e| format!("{acc} {}", e + 1))
		);
		possible_tables &= packed_result;
		let mut output = String::from("possible tables: {");
		for i in 0..17 {
			if (possible_tables & (1 << i)) == (1 << i) {
				output += &format!("\n\tT{},", i + 1);
			}
		}
		output.pop();
		output += "\n}\n";
		println!("{output}");

		line.clear();
	}
}


#[cfg(test)]
mod tests {
    use crate::{normalize_row_input, get_table_results};

    #[test]
    fn test_normalize_row_input() {
		let test_input = ",,,Pin Tuna\t  x2, Sleepyfish x2,   SushiFISHx3,,Humspun Conch,";
		assert_eq!(
			crate::normalize_row_input(test_input),
			{use crate::FishRewards::*;
			vec![
				PinTunax2,
				Sleepyfishx2,
				Sushifishx3,
				HumspunConchx1
			]}
		);
    }

	#[test]
	fn input_should_be_table_1() {
		let test_input = "popfishx4, popfishx4, sushifishx3, pintunax2, shiningstarfish";
		let normalized_input = normalize_row_input(test_input)
			.into_iter()
			.map(|e| e as u32)
			.collect();
		assert_eq!(
			get_table_results(&normalized_input),
			1
		);
	}
	#[test]
	fn should_be_table_1_or_table_10() {
		let input = "sleepyfishx2, pintuna x2, popfish x5, shining starfish x2, shining starfish";
		assert_eq!(
			get_table_results(&normalize_row_input(input)
							  .into_iter()
							  .map(|e| e as u32)
							  .collect()
			),
			1 | (1 << 9)
		);
	}
}
