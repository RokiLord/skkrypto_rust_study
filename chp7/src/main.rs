use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::collections::HashMap;

use std::io;
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Token{
    Ethereum,
    Sol,
    Xrp,
 
}
#[derive(Hash, Eq, PartialEq, Debug)]
struct WalletValue{
    tokentype: Token,
    amount:isize,
}
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Wallet{
    address: String,
    value: WalletValue,
}

impl Wallet{
    fn make_address(account_id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.input_str(account_id);
        let hex = hasher.result_str();
        
        hex
    }
}

fn make_wallet(account_id: &str, tokentype: Token, amount:isize) -> Wallet{
    Wallet { 
        address: Wallet::make_address(account_id), 
        value: (WalletValue{tokentype,amount})
    }
}

pub fn start_wallet() ->(String,Token,isize){
    let mut username = String::new();
    let mut usertoken = Token::Ethereum;
    let mut _amount = 0;
    println!("Enter your name");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    
    println!("Chooose token to put into wallet");
    println!("1.Ethereum\n 2.Sol\n 3.Xrp");

    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    
    let input: isize = input_line.trim().parse().expect("Input not an integer");
    
    if input == 1{
        usertoken = Token::Ethereum;
    }else if input == 2{
        usertoken = Token::Sol;
    }else if input == 3{
        usertoken = Token::Xrp
    }else{
        println!("Enter right number");
    }
    

    println!("Enter the amount of token to put in the wallet");
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    
    let _amount: isize = input_line.trim().parse().expect("Input not an integer");
    

    (username,usertoken,_amount)
}
#[derive(Debug)]
pub struct Balances{
    balances:HashMap<String,Wallet>
}

impl Balances{
    pub fn new(balances: HashMap<String,Wallet>) -> Self { Self { balances } }

    fn add_wallet(&mut self,owner:&str, wallet: Wallet){
       self.balances.insert(owner.to_string(), wallet);
    }


    
}

fn main(){
    
    let mut balances= Balances::new(HashMap::new());

    balances.add_wallet("Sudo",make_wallet("", Token::Ethereum, 1));
    
    let (_owner, _token,_amount) = start_wallet();
    balances.add_wallet(&_owner, make_wallet(&_owner, _token, _amount));
    
    let (_owner, _token,_amount) = start_wallet();
    balances.add_wallet(&_owner, make_wallet(&_owner, _token, _amount));
    
    println!("{:#?}",balances)
}

