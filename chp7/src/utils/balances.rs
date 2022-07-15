pub struct Balance{
    address: String,
    amount : isize,
}

pub fn set_balance(address: String, amount: isize) -> Balance{
        Balance{
            address:address,
            amount: amount,
        }
    }
impl Balance{
    pub fn get_balance(&self){
        println!("{}",&self.amount);
    }

    pub fn transfer(&mut self, _to: & mut Balance, _amount:isize){
        
        if self.amount < _amount {
            println!("not enough balance");
            return
        }
        self.amount -= _amount;

        _to.amount += _amount;


    }


    fn _transfer(&mut self, _to: & mut Balance, _amount:isize){
        self.amount -= _amount;

        _to.amount += _amount;

    }
}