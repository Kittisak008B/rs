trait Move {
    fn use_move(&self);
}
struct Thunderbolt;
struct Flamethrower;
struct WaterGun;
impl Move for Thunderbolt {
    fn use_move(&self) {
        println!("Thunderbolt! booom!");
    }
}
impl Move for Flamethrower {
    fn use_move(&self) {
        println!("Flamethrower! A burst of flames!");
    }
}
impl Move for WaterGun {
    fn use_move(&self) {
        println!("Water Gun! A splash of water!");
    }
}
struct Pikachu {
    health: u8,
    attack: u8,
    speed: u8,
    move_set: Box<dyn Move>, //Box<dyn Move> ensures that move_set holds a heap-allocated instance of Move, avoiding the "unknown size" issue.
}
struct Charizard {
    health: u8,
    attack: u8,
    speed: u8,
    move_set: Box<dyn Move>,
}
struct Squirtle {
    health: u8,
    attack: u8,
    speed: u8,
    move_set: Box<dyn Move>,
}
impl Pikachu {
    fn new()-> Self {
        Self {
            health: 100,
            attack: 12,
            speed: 20,
            move_set: Box::new(Thunderbolt),
        }
    }
    fn heal(&mut self , value:u8){
        self.health = (self.health + value).min(100);
    }
    fn take_damage(&mut self , value:u8){
        self.health = self.health.saturating_sub(value);
    }
}
impl Charizard {
    fn new()-> Self {
        Self {
            health: 200,
            attack: 40,
            speed: 15,
            move_set: Box::new(Flamethrower),
        }
    }
    fn heal(&mut self , value:u8){
        self.health = (self.health + value).min(200);
    }
    fn take_damage(&mut self , value:u8){
        self.health = self.health.saturating_sub(value);
    }
}
impl Squirtle {
    fn new()-> Self {
        Self {
            health: 120,
            attack: 20,
            speed: 12,
            move_set: Box::new(WaterGun),
        }
    }
    fn heal(&mut self , value:u8){
        self.health = (self.health + value).min(120);
    }
    fn take_damage(&mut self , value:u8){
        self.health = self.health.saturating_sub(value);
    }
}

fn perform_move(pokemon_move: &dyn Move) {  // &dyn Move Reference to a trait object (avoids ownership moves)
    pokemon_move.use_move();
}

fn main() {
    let mut pikachu = Pikachu::new();
    let mut charizard = Charizard::new();
    let mut squirtle = Squirtle::new();

    pikachu.take_damage(10);
    charizard.take_damage(10);
    squirtle.take_damage(10);

    println!("Pikachu health: {}", pikachu.health);
    println!("Charizard health: {}", charizard.health);
    println!("Squirtle health: {}", squirtle.health);

    pikachu.heal(10);
    charizard.heal(10);
    squirtle.heal(10);

    println!("Pikachu health: {}", pikachu.health);
    println!("Charizard health: {}", charizard.health);
    println!("Squirtle health: {}", squirtle.health);

    perform_move(&*pikachu.move_set);
    perform_move(&*charizard.move_set);
    perform_move(&*squirtle.move_set); 
    // squirtle.move_set is a Box<dyn Move>, a heap-allocated trait object. Since Box is a smart pointer, it dereferences to the underlying Move implementation. 
    // *squirtle.move_set gives dyn Move, but trait objects must be used via references. &(*squirtle.move_set) first dereferences the Box, then borrows it as &dyn Move for perform_move.
}
