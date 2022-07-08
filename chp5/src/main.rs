#[derive(Debug, Clone)]
#[allow(unused_variables)]

struct Character {
    name : String,
    health : i32,
    alive : bool,
    
}
fn main(){
    let mut terry = Character{
        name : String::from("terry"),
        health: 100,
        alive : true,
    };

    let mut john = Character{
        name : String::from("john"),
        health: 100,
        alive : true,
    };

    
    println!("{}", terry.getName());
    println!("{}", john.getName());

}




impl Character{
    fn getName(&self) -> &str{
        &self.name
    }


}   