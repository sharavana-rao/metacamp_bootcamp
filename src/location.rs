//location.rs
use std::fmt;

#[derive(Debug)]
pub enum Country {

UnitedStates,
Canada,
UnitedKingdom,
Germany,
France, 
Japan,
Australia,
China, 
Brazil,
SouthKorea,
Ireland,
Spain,
India,
Switzerland

}

impl Country {
    pub fn country_to_continent(&self) -> Continent{
        match self{
            Country::UnitedStates => Continent::NorthAmerica,
            Country::Canada => Continent::NorthAmerica,
            Country::UnitedKingdom => Continent::Europe,
            Country::Germany => Continent::Europe,
            Country::France => Continent::Europe,
            Country::Japan => Continent::Asia,
            Country::Australia => Continent::Oceania,
            Country::China => Continent::Asia,
            Country::Brazil => Continent::SouthAmerica,
            Country::SouthKorea => Continent::Asia,
            Country::Ireland => Continent::Europe,
            Country::Spain => Continent::Europe,
            Country::India => Continent::Asia,
            Country::Switzerland => Continent::Europe,

        }
    }
}

impl std::str::FromStr for Country{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self,Self::Err>{
        match s{
             "UnitedStates" => Ok(Country::UnitedStates),
             "Canada" => Ok(Country::Canada),
             "UnitedKingdom" => Ok(Country::UnitedKingdom),
             "Germany" => Ok(Country::Germany),
             "France" => Ok(Country::France),
             "Japan" => Ok(Country::Japan),
             "Australia" => Ok(Country::Australia),
             "China" => Ok(Country::China),
             "Brazil" => Ok(Country::Brazil),
             "SouthKorea" => Ok(Country::SouthKorea),
             "Ireland" => Ok(Country::Ireland),
             "Spain" => Ok(Country::Spain),
             "India" => Ok(Country::India),
             "Switzerland" => Ok(Country::Switzerland),
             _ => Err("Invalid country")
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]

pub enum Continent {
NorthAmerica,
Europe,
Asia,
Oceania,
SouthAmerica
}

impl std::fmt::Display for Continent{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Continent::NorthAmerica => write!(f, "North America"),
            Continent::Europe => write!(f, "Europe"),
            Continent::Asia => write!(f, "Asia"),
            Continent::Oceania => write!(f, "Oceania"),
            Continent::SouthAmerica => write!(f, "South America"),
            _ => panic!()
        }
    }
}