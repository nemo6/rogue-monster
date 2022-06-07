/* use console::Term;

fn main() {
	
    let stdout = Term::buffered_stdout();
	
	let secret_number = 50;

    loop {
        if let Ok(character) = stdout.read_char() {
            match character {
                'z' => println!("Up"),
                'q' => println!("Left"),
                's' => println!("Down"),
                'd' => println!("Right"),
				'e' => println!("Attack!"),
                _ => println!("Not a move"), // ()
            }
        }
    }
} */

// use std::io;
// use std::io::Write;
use console::Term;
use rand::Rng;
use std::process;

fn main() {

	let seed = 4;

	let stdout = Term::buffered_stdout();

	let mut player_position = rand::thread_rng().gen_range(0,(seed*seed)+1);

	let monstr_position = rand::thread_rng().gen_range(0,(seed*seed)+1);

	let mut life_monster = 10;

	let mut find_monster: bool = false;
	
	/* let stdout = Term::buffered_stdout();
	
	print!("Please enter a number : ");
		
	io::stdout().flush().unwrap();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

	let mut number = match input_text.trim().parse::<i32>() {
		Ok(num) => num,
		Err(..) => {
		   println!("this was not an integer");
		   0
		}
    }; */
	
	/*fn generate (x:i32) -> Vec<i32> {
		// (0..=(x-1)).map( |x| (x%5)+1 ).collect()
		(0..=(x-1)).collect()
	}*/
	
	 fn generate_zero(size: i32) -> Vec<i32> {
		
		let mut zero_vec: Vec<i32> = Vec::with_capacity(size as usize);
		for _i in 0..size {
			zero_vec.push(0);
		}
		return zero_vec;
	} 
	
	// let my_vector = generate(5*5);
	
	let mut my_vector = generate_zero(seed*seed);
	
	// println!("{:?}\n",my_vector);
	
	/* for n in 0..my_vector.len() {
		
		my_vector[n] = (n as i32) + 2;
		println!("{}", my_vector[n] );
	} */
	
	/* for el in my_vector.iter() {
	  println!("{}", el);
	} */
	
	/* for n in 0..=5 {
	let sl2 = &my_vector[0..=5];
	println!("{:?}",sl2);
	} */
	
	/* let mut last = 0;
	for x in (4..=24).step_by(5) {
		let sl2 = &my_vector[last..=x];
        println!("{:?}", sl2 );
		last = x+1;
    } */

    fn attack(find_monster:bool,life_monster:&mut i32){

    	if !find_monster
    	{ println!("Vous n'avez pas encore trouvé le monstre.") }

    	if *life_monster > 0
    	{ *life_monster-=1 }
    	else {
    		println!("Vous avez gagné!");
    		process::exit(0);
    	}

    	println!("point de vie du monstre : {}",life_monster);

    	// number-=1; println!("Attack ( monster life : {} )", number );
    }

 	fn show(my_vector:&mut Vec<i32>,player_position:i32,monstr_position:i32,seed:i32){

		for n in 0..my_vector.len() {

			if n%(seed as usize) == 0
			{ print!("\n") }

			if n == (player_position as usize)
			{ print!( "{} ","P") }
			else if n == (monstr_position as usize)
			{ print!("{} ","M") }
			else
			{ print!( "{} ",  my_vector[n] ) }

			if n == my_vector.len()-1 { print!("\n\n") }
		}

	}

	fn change_position(direction:&str,player_position:&mut i32,monstr_position:i32,seed:i32,find_monster:&mut bool){
		
		// player_position = player_position - seed
		
		println!( "{} ", direction );

		let mut n = *player_position;

		if direction == "Up"
		{n-=seed}
		else if direction == "Left"
		{n-=1}
		else if direction == "Down"
		{n+=seed}
		else if direction == "Right"
		{n+=1}

		if n == monstr_position {
			println!("Vous avez trouvé le monstre!");
			*find_monster = true;
		}

		if n >= 0 && n < seed*seed 
		{*player_position = n}
	}

	 loop {
		
		if let Ok(character) = stdout.read_char() {
			match character {
				'z' => change_position("Up",    &mut player_position, monstr_position, seed , &mut find_monster ),
				'q' => change_position("Left",  &mut player_position, monstr_position, seed , &mut find_monster ),
				's' => change_position("Down",  &mut player_position, monstr_position, seed , &mut find_monster ),
				'd' => change_position("Right", &mut player_position, monstr_position, seed , &mut find_monster ),
				'e' => attack(find_monster,&mut life_monster),
				'f' => show( &mut my_vector,  player_position,monstr_position, seed ),
				_   => println!("Not a move"),
			}
		}
		
	} 

}
