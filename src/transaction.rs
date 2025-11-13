use crate::{location::{Continent, Country}, transaction};
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Transaction {
    transaction_id: u32,
    client_id: u32,
    asset_name: String,
    country: Country,
    pub continent: Continent,
    pub amount: f64,
    days_under_management: i64,
}

pub fn from_csv_line(line: &str) -> Result<Transaction,String>{
    let fields: Vec<&str> = line.split(",").collect();
    if fields.len()!= 7{
        return Err(String::from("Error"));
    }
    let transaction_id = fields[0].parse::<u32>().unwrap();
    let client_id = fields[1].parse::<u32>().unwrap();
    let asset_name = fields[2].to_uppercase();

    let transaction_start_date:NaiveDate = NaiveDate::parse_from_str(fields[3], "%Y-%m-%d").unwrap();
    let transaction_end_date:NaiveDate  = NaiveDate::parse_from_str(fields[4], "%Y-%m-%d").unwrap();
    let country= fields[5].parse::<Country>()?;
    let amount = fields[6].parse::<f64>().unwrap();

    let days_under_management= (transaction_start_date - transaction_end_date).num_days();
    let continent = country.country_to_continent();

    let transaction= Transaction{
        transaction_id,
        client_id,
        asset_name,
        country,
        continent,
        amount,
        days_under_management
    };

    Ok((transaction))

}


