#[derive(Debug)]
pub enum Token{
    Bitcoin,
    Ethereum,

}

pub struct Wallet{
    address: String,
    tokentype: Token,
    amount : isize,
}



pub fn set_wallet(address: String, tokentype: Token, amount: isize) -> Wallet{
        Wallet{
            address:address,
            tokentype: tokentype,
            amount: amount,
        }
    }
impl Wallet{
    
    
    pub fn get_wallet(&self){
        println!("{:?} {} ",&self.tokentype, &self.amount);
    }

    pub fn transfer(&mut self, _to: & mut Wallet, _amount:isize){
        
        if self.amount < _amount {
            println!("not enough token");
            return
        }
        self.amount -= _amount;

        _to.amount += _amount;


    }


    fn _transfer(&mut self, _to: & mut Wallet, _amount:isize){
        self.amount -= _amount;

        _to.amount += _amount;

    }
}