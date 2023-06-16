#[allow(non_camel_case_types)]
type int = i32;
#[allow(non_camel_case_types)]
type unsigned = i32;
#[allow(non_camel_case_types)]
type array<t> = Vec<t>;
#[allow(non_camel_case_types)]
type long__long = i64;



static items_eng: [&str; 12] = [
	"",
	"Shining Starfish x1",
	"Pin Tuna x2",
	"Shining Starfish x2",
	"Sushifish x2",
	"Popfish x5",
	"Sushifish x3",
	"Whetfish x6",
	"Sleepyfish x2",
	"Popfish x4",
	"Whetfish x4",
	"Humspun Conch"
];



fn rnd(n: int) -> int {
	if n == 0 {return 176}

	let mut x: int = n * 16 - n * 4 - n;
	let mut y: long__long = (x * 16).into();
	let mut z: long__long = y * 11367737 / 4294967296;
	let mut m: long__long = y - ( (y-z) / 2 + z) / 32768 * 65363;
	(m & 0xFFFF) as int
}


fn get_fish_index(roll: unsigned, fish: &mut array<unsigned>) -> unsigned {
	let mut total: unsigned = fish[0];
	let mut count: unsigned = 0;
	while total <= roll {
		count += 1;total += fish[count as usize];
	}
	count
}


fn reward_func(mut num: unsigned, rep: unsigned) -> unsigned {
	let mut i = 0;while i < rep {
		num = (num * 176) % 65363;

		i += 1;}
	num
}


fn array_equals(a: &mut array<unsigned>, mut r#as: unsigned, b: &mut array<unsigned>, mut bs: unsigned, len: unsigned) -> bool {
	let mut i: unsigned = 0;while i < len {
		if a[(r#as + i) as usize] != b[(bs + i) as usize] {return false}

		i += 1;}
	true
}




fn clamp<T: Ord>(val: T, min_val: T, max_val: T) -> T {
	::std::cmp::max(::std::cmp::min(val, max_val), min_val)
}


fn search_seed(seed: unsigned, fish_table: &mut Vec<unsigned>, items: &mut Vec<unsigned>) -> bool {
	let mut qwe: Vec<unsigned> = Vec::with_capacity(13);
	let mut j: int = 0;while j < (qwe.len()).try_into().unwrap() {
		j+= 1;let roll: unsigned = reward_func(seed, j) % 100;
		qwe[(j-1) as usize] = get_fish_index(roll, fish_table);
	}
	let max_fishfinder: unsigned = (clamp(items.len(), 4, 7) - 4).try_into().unwrap();
	let mut i: unsigned = 0;while i <= max_fishfinder {
		if array_equals(&mut qwe, 0, items, 0, (items.len() - (i as usize)).try_into().unwrap())
			&& array_equals(&mut qwe, 10, items, (items.len() - (i as usize)).try_into().unwrap(), i) {
				return true;
			}
			i += 1;}
	false
}


fn check_seed(seed: int, fish_table: &mut array<unsigned>, items: &mut array<unsigned>) -> bool {
	let mut n: int = seed;
	loop {
		if search_seed(n, fish_table, items) {return true};

		n = rnd(n);
		if n != seed {break};
	}
	false
}


fn get_table_results(items: &mut array<unsigned>) -> unsigned {
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
		HumspunConch = 10,
	}

	let mut table_seeds: array<int> = vec!{1, 15, 5, 13, 4, 3, 9, 12, 26, 18, 163, 401, 6, 2, 489, 802, 1203};
	let mut fish: array<unsigned> = vec!{20, 15, 5, 5, 10, 15, 5, 10, 5, 5, 5};

	let mut res: unsigned = 0;
	let mut t: unsigned = 0;while t < 17 {
		if check_seed(table_seeds[t as usize], &mut fish, items) {
			res |= 1 << t;
		}
	t += 1;}
	res
}



fn main() {
    println!("Hello, world!");
}
