
use utils::balances::*;
use utils::wallet::*;
mod utils;



fn main() {
    let mut balance1 = set_balance(String::from("terry"), 10000);

    let mut balance2 = set_balance(String::from("taek"), 10000);

    

    balance1.transfer(&mut balance2, 2000);

    balance1.get_balance();
    
    balance2.get_balance();

    balance1.transfer(&mut balance2, 10000);
    
    balance1.get_balance();
    
    balance2.get_balance();
    
    let mut wallet1 = set_wallet(String::from("terry"),Token::Bitcoin,100);
    let mut wallet2 = set_wallet(String::from("taek"),Token::Bitcoin,100);

   

    wallet1.transfer(&mut wallet2, 20);

    wallet1.get_wallet();
    
    wallet2.get_wallet();

    wallet1.transfer(&mut wallet2, 100); 
    
    wallet1.get_wallet();
    
    wallet2.get_wallet();
}
