
use utils::balances::*;

mod utils;


fn main() {
    let mut balance1 = set_balance(String::from("terry"), 10000);

    let mut balance2 = set_balance(String::from("taek"), 10000);

    balance1.get_balance();
    
    balance2.get_balance();

    balance1.transfer(&mut balance2, 2000);

    balance1.get_balance();
    
    balance2.get_balance();

}
