//source from https://www.youtube.com/watch?v=lQ7XF-6HYGc&t=2046s

#[allow(unused_variables)]

fn main(){
    //STACK
    // - Fast memory creation and retrieval
    // Known Fixed size vriables Collections Cannot be stack without array

    // Strings are collection of u8's 

    let stack_i8: i8 = 10;
    let stack_f32: f32 = 20.0;
    let stack_bool : bool = true;

    //HEAP
    // -Flexibility
    // Memory that can grow in size( Vector, HashMap, String)
    // Runtime performance cost
    // Memory that can live beyond the scope that created it

    let heap_vector : Vec<i8> = Vec::new();
    let heap_string : String = String::from("Terry");
    let heap_i8 : Box<i8> = Box::new(30);
    let stack_i8_2 = stack_i8;
    println!("{}",stack_i8); //<-- possible because of Copy Trait
    println!("{}",stack_i8_2);

    let heap_i8_2 = heap_i8;
    //println!("{}",heap_i8); <-- cause error b/c ownership moved to _2
    println!("{}",heap_i8_2);


    let var_a = String::from("Howdy!");
    let var_b = &var_a;
    let var_c = &var_a;

    println!("{} {} {}", var_a, var_b, var_c);

    //Who's the ovwner of the data?
    //var_a throughout
    //why & works? compiler know var_a does not change downstream in the code
    //If change, compile error
}