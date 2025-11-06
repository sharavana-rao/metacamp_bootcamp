fn main() {

    let buyer_1 = Buyer{
        name: String::from("John"),
        payment_type: PaymentType::DigitalToken,
        balance: 100.00
    };

    let buyer_2 = Buyer{
        name: String::from("Sally"),
        payment_type: PaymentType::Cash,
        balance: 100.00
    };

    let mut new_buyer_group = BuyerGroup{
        members: Vec::new()
    };

    new_buyer_group.add_member(buyer_1);
    new_buyer_group.add_member(buyer_2);

    // println!("{:?}", new_buyer_group)

    let mut seller_1 = Seller{
    payment_type: PaymentType::Cash,
    price: 10.00,
    balance: 0.00
};

let i = new_buyer_group.find_buyer(PaymentType::Cash);
new_buyer_group.buy(i as usize, &mut seller_1);

println!("{}", seller_1.balance);

}

#[derive(PartialEq)]
#[derive(Debug)]
enum PaymentType{
    DigitalToken,
    Cash
}

struct Seller{
    payment_type: PaymentType,
    price: f32,
    balance: f32
}

#[derive(Debug)]
struct Buyer{
    name: String,
    payment_type: PaymentType,
    balance: f32
}   

#[derive(Debug)]
struct BuyerGroup{
    members: Vec<Buyer>
}

impl BuyerGroup{
    fn add_member(&mut self, buyer: Buyer){
        self.members.push(buyer )
    }

    fn find_buyer(&self, payment_type_to_find: PaymentType) -> isize {
        for (index, buyer) in self.members.iter().enumerate(){
            if buyer.payment_type == payment_type_to_find{
                return index as isize//had to typecast this because index was usize return type but function needs to be able to return negative too
            }
        }
        -1
    }
//Implement method buy on BuyerGroup which accepts a buyer index, reference to a seller, 
//and keeps transferring value of seller.price from buyer to seller, until the buyer's balance is insufficient.

    fn buy(&mut self, buyer_index: usize, seller: &mut Seller){

    while self.members[buyer_index].balance >= seller.price{
        self.members[buyer_index].balance = self.members[buyer_index].balance - seller.price;
        seller.balance = seller.balance + seller.price;
    }

    }
}
