mod location;
mod transaction;
use std::fs::File;
use std::io::{BufReader, BufRead};
use transaction::{Transaction,from_csv_line};
use std::collections::HashMap;

use location::Continent;

fn main() {
    println!("Hello, world!");

    let file = File::open("./transactions.csv").unwrap();
    let reader = BufReader::new(file);
    let mut transactions: Vec<Transaction> = Vec::new();
    let mut skipped_lines = Vec::new();

    for (idx,line) in reader.lines().enumerate(){
        if idx == 0{
            continue;//skips this 0 condition
        }
        let line_str = line.unwrap();
        let parsed_transaction = from_csv_line(line_str.as_str());

        match parsed_transaction{
            Ok(t) => transactions.push(t),
            Err(e) => skipped_lines.push((idx,e,line_str))
        }
    }

    for transaction in transactions.iter(){
        println!("{:?}", transaction);
    }

    for lines in skipped_lines.iter(){
        println!("{:?}", lines);
    }
//Utilize HashMap to keep track of the total invested amount per continent
//and print the result out for each continent
//Hint: You would need to convert the continent to String to store as keys

//create a new empty hashmap for continent name and amount
let mut continent_totals: HashMap<String, f64> = HashMap::new();

    // Loop through the transactions.
    for t in transactions.iter() {
        // Get the continent name as a string, since the display Trait is implemented alr.
        let continent_name = t.continent.to_string();

        // Use the entry HashMap method to get the value for this continent, or insert 0.0 if this is the first time we've seen it.
        let current_total = continent_totals.entry(continent_name).or_insert(0.0);

        // Add the transaction's amount to that value.
        // Because or_insert method returns a mutable reference, I need to dereference the mutable reference and update the actual value.
        *current_total += t.amount;
    }

    //Print the total invested per continent
    println!("Total invested per continent:{:?}", continent_totals);

//Calling the european_companies function from below
let europe = Continent::Europe;
european_companies(&transactions, &europe);

}

//Create a function that takes in a reference slice of transactions and a
//reference of Continent, and filters rows by the Continent. Print only
//transactions with European companies
//Hint: You would need to utilise iterators, and filter function

fn european_companies(transactions: &[Transaction], continent_criteria: &Continent) {
let filtered_transactions = transactions.iter().filter(|t| {
        // Compare the transaction's continent with the required continent.
        // Compare both references through borrowing, as partial EQ is applied
        &t.continent == continent_criteria
    });

    //Loop and printing the transactions needed
    println!("Transactions in {:?}:", continent_criteria);
    for t in filtered_transactions {
        println!("{:?}", t);
    }

}


