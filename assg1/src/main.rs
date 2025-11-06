fn main() {

    let mut user1 = User{
        name: String::from("Tommy"),
        balance: (200.00, String::from("SGD"))
        
    };

    //have to use &mut user1 here if not ownership gets moved and can't do multiple function calls to compound. 
    accrue_interest(&mut user1, 10.00);
    accrue_interest(&mut user1, 10.00);
    accrue_interest(&mut user1, 10.00);
    accrue_interest(&mut user1, 10.00);
}


struct User{
    name: String,
    balance: (f64, String)
}

impl User{
    fn print_user_detail (&self) {
        println!("name: {:?}, balance: {:?}, currency: {:?}", self.name, self.balance.0, self.balance.1 );
    }
}

fn accrue_interest(user: &mut User, interest: f64) {
    user.balance.0 = user.balance.0 + ((interest/100.0) * user.balance.0);
    user.print_user_detail();
}