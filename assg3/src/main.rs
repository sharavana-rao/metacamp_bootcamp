fn main() {

    let user_account = UserAccount{
        name: String::from("John"),
        age: None
    };

    match increase_balance(&user_account, 10){
        Ok(x)=> println!("UserAccount balance increased to {}", x),
        Err(e)=> println!("{}", e),
    }

    if let Some(age) = user_account.age {
    println!("Age is: {}", age);
} else {
    println!("Age is not existent");
}
        
    }
    

struct UserAccount{
    name: String,
    age: Option<u32>
}

trait Balance{
    fn get_balance(&self) -> u32{
        10
    }
}

impl Balance for UserAccount{}

fn increase_balance<T: Balance>(item: &T, amount: u32) -> Result<u32, String>{
    if amount <= 10{
        return Ok(item.get_balance() + amount)
    }

    Err(String::from("Increase must be less than 10!"))
}