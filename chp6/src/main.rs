enum Token {
    Bitcoin,
    Ether,
}



fn main() {
   
       fn Swap(token : Token){
        match token {
            Token :: Bitcoin => {
                println!("Current Bitcoin price is {}$",20000);
            },
            Token :: Ether => {
                println!("Current Ether price is {}$",1000);
            }
        }
    }


    Swap(Token:: Bitcoin);
    Swap(Token:: Ether);
}
