//! File containing the data definitions for better readability.

use rand_distr::{Distribution, Normal};

pub(crate) struct Station {
    pub(crate) name: &'static str,
    pub(crate) temperature: f64,
}

impl Station {
    pub(crate) fn generate_measurement(&self, rng: &mut rand::prelude::StdRng) -> f64 {
        let dist = Normal::new(self.temperature, 10.0).unwrap();
        dist.sample(rng)
    }
}

/// data from https://en.wikipedia.org/wiki/List_of_cities_by_average_temperature;
/// converted using https://wikitable2csv.ggor.de/
///
/// ```sql
/// D copy (
///     select City, regexp_extract(Year,'(.*)\n.*', 1) as AverageTemp
///     from (
///         select City,Year
///         from read_csv_auto('List_of_cities_by_average_temperature_1.csv', header = true)
///         union
///         select City,Year
///         from read_csv_auto('List_of_cities_by_average_temperature_2.csv', header = true)
///         union
///         select City,Year
///         from read_csv_auto('List_of_cities_by_average_temperature_3.csv', header = true)
///         union
///         select City,Year
///         from read_csv_auto('List_of_cities_by_average_temperature_4.csv', header = true)
///         union
///         select City,Year
///         from read_csv_auto('List_of_cities_by_average_temperature_5.csv', header = true)
///         )
/// ) TO 'output.csv' (HEADER, DELIMITER ',');
/// ```
pub(crate) const STATIONS: &[Station] = &[
    Station {
        name: "Abha",
        temperature: 18.0,
    },
    Station {
        name: "Abidjan",
        temperature: 26.0,
    },
    Station {
        name: "Abéché",
        temperature: 29.4,
    },
    Station {
        name: "Accra",
        temperature: 26.4,
    },
    Station {
        name: "Addis Ababa",
        temperature: 16.0,
    },
    Station {
        name: "Adelaide",
        temperature: 17.3,
    },
    Station {
        name: "Aden",
        temperature: 29.1,
    },
    Station {
        name: "Ahvaz",
        temperature: 25.4,
    },
    Station {
        name: "Albuquerque",
        temperature: 14.0,
    },
    Station {
        name: "Alexandra",
        temperature: 11.0,
    },
    Station {
        name: "Alexandria",
        temperature: 20.0,
    },
    Station {
        name: "Algiers",
        temperature: 18.2,
    },
    Station {
        name: "Alice Springs",
        temperature: 21.0,
    },
    Station {
        name: "Almaty",
        temperature: 10.0,
    },
    Station {
        name: "Amsterdam",
        temperature: 10.2,
    },
    Station {
        name: "Anadyr",
        temperature: -6.9,
    },
    Station {
        name: "Anchorage",
        temperature: 2.8,
    },
    Station {
        name: "Andorra la Vella",
        temperature: 9.8,
    },
    Station {
        name: "Ankara",
        temperature: 12.0,
    },
    Station {
        name: "Antananarivo",
        temperature: 17.9,
    },
    Station {
        name: "Antsiranana",
        temperature: 25.2,
    },
    Station {
        name: "Arkhangelsk",
        temperature: 1.3,
    },
    Station {
        name: "Ashgabat",
        temperature: 17.1,
    },
    Station {
        name: "Asmara",
        temperature: 15.6,
    },
    Station {
        name: "Assab",
        temperature: 30.5,
    },
    Station {
        name: "Astana",
        temperature: 3.5,
    },
    Station {
        name: "Athens",
        temperature: 19.2,
    },
    Station {
        name: "Atlanta",
        temperature: 17.0,
    },
    Station {
        name: "Auckland",
        temperature: 15.2,
    },
    Station {
        name: "Austin",
        temperature: 20.7,
    },
    Station {
        name: "Baghdad",
        temperature: 22.77,
    },
    Station {
        name: "Baguio",
        temperature: 19.5,
    },
    Station {
        name: "Baku",
        temperature: 15.1,
    },
    Station {
        name: "Baltimore",
        temperature: 13.1,
    },
    Station {
        name: "Bamako",
        temperature: 27.8,
    },
    Station {
        name: "Bangkok",
        temperature: 28.6,
    },
    Station {
        name: "Bangui",
        temperature: 26.0,
    },
    Station {
        name: "Banjul",
        temperature: 26.0,
    },
    Station {
        name: "Barcelona",
        temperature: 18.2,
    },
    Station {
        name: "Bata",
        temperature: 25.1,
    },
    Station {
        name: "Batumi",
        temperature: 14.0,
    },
    Station {
        name: "Beijing",
        temperature: 12.9,
    },
    Station {
        name: "Beirut",
        temperature: 20.9,
    },
    Station {
        name: "Belgrade",
        temperature: 12.5,
    },
    Station {
        name: "Belize City",
        temperature: 26.7,
    },
    Station {
        name: "Benghazi",
        temperature: 19.9,
    },
    Station {
        name: "Bergen",
        temperature: 7.7,
    },
    Station {
        name: "Berlin",
        temperature: 10.3,
    },
    Station {
        name: "Bilbao",
        temperature: 14.7,
    },
    Station {
        name: "Birao",
        temperature: 26.5,
    },
    Station {
        name: "Bishkek",
        temperature: 11.3,
    },
    Station {
        name: "Bissau",
        temperature: 27.0,
    },
    Station {
        name: "Blantyre",
        temperature: 22.2,
    },
    Station {
        name: "Bloemfontein",
        temperature: 15.6,
    },
    Station {
        name: "Boise",
        temperature: 11.4,
    },
    Station {
        name: "Bordeaux",
        temperature: 14.2,
    },
    Station {
        name: "Bosaso",
        temperature: 30.0,
    },
    Station {
        name: "Boston",
        temperature: 10.9,
    },
    Station {
        name: "Bouaké",
        temperature: 26.0,
    },
    Station {
        name: "Bratislava",
        temperature: 10.5,
    },
    Station {
        name: "Brazzaville",
        temperature: 25.0,
    },
    Station {
        name: "Bridgetown",
        temperature: 27.0,
    },
    Station {
        name: "Brisbane",
        temperature: 21.4,
    },
    Station {
        name: "Brussels",
        temperature: 10.5,
    },
    Station {
        name: "Bucharest",
        temperature: 10.8,
    },
    Station {
        name: "Budapest",
        temperature: 11.3,
    },
    Station {
        name: "Bujumbura",
        temperature: 23.8,
    },
    Station {
        name: "Bulawayo",
        temperature: 18.9,
    },
    Station {
        name: "Burnie",
        temperature: 13.1,
    },
    Station {
        name: "Busan",
        temperature: 15.0,
    },
    Station {
        name: "Cabo San Lucas",
        temperature: 23.9,
    },
    Station {
        name: "Cairns",
        temperature: 25.0,
    },
    Station {
        name: "Cairo",
        temperature: 21.4,
    },
    Station {
        name: "Calgary",
        temperature: 4.4,
    },
    Station {
        name: "Canberra",
        temperature: 13.1,
    },
    Station {
        name: "Cape Town",
        temperature: 16.2,
    },
    Station {
        name: "Changsha",
        temperature: 17.4,
    },
    Station {
        name: "Charlotte",
        temperature: 16.1,
    },
    Station {
        name: "Chiang Mai",
        temperature: 25.8,
    },
    Station {
        name: "Chicago",
        temperature: 9.8,
    },
    Station {
        name: "Chihuahua",
        temperature: 18.6,
    },
    Station {
        name: "Chișinău",
        temperature: 10.2,
    },
    Station {
        name: "Chittagong",
        temperature: 25.9,
    },
    Station {
        name: "Chongqing",
        temperature: 18.6,
    },
    Station {
        name: "Christchurch",
        temperature: 12.2,
    },
    Station {
        name: "City of San Marino",
        temperature: 11.8,
    },
    Station {
        name: "Colombo",
        temperature: 27.4,
    },
    Station {
        name: "Columbus",
        temperature: 11.7,
    },
    Station {
        name: "Conakry",
        temperature: 26.4,
    },
    Station {
        name: "Copenhagen",
        temperature: 9.1,
    },
    Station {
        name: "Cotonou",
        temperature: 27.2,
    },
    Station {
        name: "Cracow",
        temperature: 9.3,
    },
    Station {
        name: "Da Lat",
        temperature: 17.9,
    },
    Station {
        name: "Da Nang",
        temperature: 25.8,
    },
    Station {
        name: "Dakar",
        temperature: 24.0,
    },
    Station {
        name: "Dallas",
        temperature: 19.0,
    },
    Station {
        name: "Damascus",
        temperature: 17.0,
    },
    Station {
        name: "Dampier",
        temperature: 26.4,
    },
    Station {
        name: "Dar es Salaam",
        temperature: 25.8,
    },
    Station {
        name: "Darwin",
        temperature: 27.6,
    },
    Station {
        name: "Denpasar",
        temperature: 23.7,
    },
    Station {
        name: "Denver",
        temperature: 10.4,
    },
    Station {
        name: "Detroit",
        temperature: 10.0,
    },
    Station {
        name: "Dhaka",
        temperature: 25.9,
    },
    Station {
        name: "Dikson",
        temperature: -11.1,
    },
    Station {
        name: "Dili",
        temperature: 26.6,
    },
    Station {
        name: "Djibouti",
        temperature: 29.9,
    },
    Station {
        name: "Dodoma",
        temperature: 22.7,
    },
    Station {
        name: "Dolisie",
        temperature: 24.0,
    },
    Station {
        name: "Douala",
        temperature: 26.7,
    },
    Station {
        name: "Dubai",
        temperature: 26.9,
    },
    Station {
        name: "Dublin",
        temperature: 9.8,
    },
    Station {
        name: "Dunedin",
        temperature: 11.1,
    },
    Station {
        name: "Durban",
        temperature: 20.6,
    },
    Station {
        name: "Dushanbe",
        temperature: 14.7,
    },
    Station {
        name: "Edinburgh",
        temperature: 9.3,
    },
    Station {
        name: "Edmonton",
        temperature: 4.2,
    },
    Station {
        name: "El Paso",
        temperature: 18.1,
    },
    Station {
        name: "Entebbe",
        temperature: 21.0,
    },
    Station {
        name: "Erbil",
        temperature: 19.5,
    },
    Station {
        name: "Erzurum",
        temperature: 5.1,
    },
    Station {
        name: "Fairbanks",
        temperature: -2.3,
    },
    Station {
        name: "Fianarantsoa",
        temperature: 17.9,
    },
    Station {
        name: "Flores,  Petén",
        temperature: 26.4,
    },
    Station {
        name: "Frankfurt",
        temperature: 10.6,
    },
    Station {
        name: "Fresno",
        temperature: 17.9,
    },
    Station {
        name: "Fukuoka",
        temperature: 17.0,
    },
    Station {
        name: "Gabès",
        temperature: 19.5,
    },
    Station {
        name: "Gaborone",
        temperature: 21.0,
    },
    Station {
        name: "Gagnoa",
        temperature: 26.0,
    },
    Station {
        name: "Gangtok",
        temperature: 15.2,
    },
    Station {
        name: "Garissa",
        temperature: 29.3,
    },
    Station {
        name: "Garoua",
        temperature: 28.3,
    },
    Station {
        name: "George Town",
        temperature: 27.9,
    },
    Station {
        name: "Ghanzi",
        temperature: 21.4,
    },
    Station {
        name: "Gjoa Haven",
        temperature: -14.4,
    },
    Station {
        name: "Guadalajara",
        temperature: 20.9,
    },
    Station {
        name: "Guangzhou",
        temperature: 22.4,
    },
    Station {
        name: "Guatemala City",
        temperature: 20.4,
    },
    Station {
        name: "Halifax",
        temperature: 7.5,
    },
    Station {
        name: "Hamburg",
        temperature: 9.7,
    },
    Station {
        name: "Hamilton",
        temperature: 13.8,
    },
    Station {
        name: "Hanga Roa",
        temperature: 20.5,
    },
    Station {
        name: "Hanoi",
        temperature: 23.6,
    },
    Station {
        name: "Harare",
        temperature: 18.4,
    },
    Station {
        name: "Harbin",
        temperature: 5.0,
    },
    Station {
        name: "Hargeisa",
        temperature: 21.7,
    },
    Station {
        name: "Hat Yai",
        temperature: 27.0,
    },
    Station {
        name: "Havana",
        temperature: 25.2,
    },
    Station {
        name: "Helsinki",
        temperature: 5.9,
    },
    Station {
        name: "Heraklion",
        temperature: 18.9,
    },
    Station {
        name: "Hiroshima",
        temperature: 16.3,
    },
    Station {
        name: "Ho Chi Minh City",
        temperature: 27.4,
    },
    Station {
        name: "Hobart",
        temperature: 12.7,
    },
    Station {
        name: "Hong Kong",
        temperature: 23.3,
    },
    Station {
        name: "Honiara",
        temperature: 26.5,
    },
    Station {
        name: "Honolulu",
        temperature: 25.4,
    },
    Station {
        name: "Houston",
        temperature: 20.8,
    },
    Station {
        name: "Ifrane",
        temperature: 11.4,
    },
    Station {
        name: "Indianapolis",
        temperature: 11.8,
    },
    Station {
        name: "Iqaluit",
        temperature: -9.3,
    },
    Station {
        name: "Irkutsk",
        temperature: 1.0,
    },
    Station {
        name: "Istanbul",
        temperature: 13.9,
    },
    Station {
        name: "İzmir",
        temperature: 17.9,
    },
    Station {
        name: "Jacksonville",
        temperature: 20.3,
    },
    Station {
        name: "Jakarta",
        temperature: 26.7,
    },
    Station {
        name: "Jayapura",
        temperature: 27.0,
    },
    Station {
        name: "Jerusalem",
        temperature: 18.3,
    },
    Station {
        name: "Johannesburg",
        temperature: 15.5,
    },
    Station {
        name: "Jos",
        temperature: 22.8,
    },
    Station {
        name: "Juba",
        temperature: 27.8,
    },
    Station {
        name: "Kabul",
        temperature: 12.1,
    },
    Station {
        name: "Kampala",
        temperature: 20.0,
    },
    Station {
        name: "Kandi",
        temperature: 27.7,
    },
    Station {
        name: "Kankan",
        temperature: 26.5,
    },
    Station {
        name: "Kano",
        temperature: 26.4,
    },
    Station {
        name: "Kansas City",
        temperature: 12.5,
    },
    Station {
        name: "Karachi",
        temperature: 26.0,
    },
    Station {
        name: "Karonga",
        temperature: 24.4,
    },
    Station {
        name: "Kathmandu",
        temperature: 18.3,
    },
    Station {
        name: "Khartoum",
        temperature: 29.9,
    },
    Station {
        name: "Kingston",
        temperature: 27.4,
    },
    Station {
        name: "Kinshasa",
        temperature: 25.3,
    },
    Station {
        name: "Kolkata",
        temperature: 26.7,
    },
    Station {
        name: "Kuala Lumpur",
        temperature: 27.3,
    },
    Station {
        name: "Kumasi",
        temperature: 26.0,
    },
    Station {
        name: "Kunming",
        temperature: 15.7,
    },
    Station {
        name: "Kuopio",
        temperature: 3.4,
    },
    Station {
        name: "Kuwait City",
        temperature: 25.7,
    },
    Station {
        name: "Kyiv",
        temperature: 8.4,
    },
    Station {
        name: "Kyoto",
        temperature: 15.8,
    },
    Station {
        name: "La Ceiba",
        temperature: 26.2,
    },
    Station {
        name: "La Paz",
        temperature: 23.7,
    },
    Station {
        name: "Lagos",
        temperature: 26.8,
    },
    Station {
        name: "Lahore",
        temperature: 24.3,
    },
    Station {
        name: "Lake Havasu City",
        temperature: 23.7,
    },
    Station {
        name: "Lake Tekapo",
        temperature: 8.7,
    },
    Station {
        name: "Las Palmas de Gran Canaria",
        temperature: 21.2,
    },
    Station {
        name: "Las Vegas",
        temperature: 20.3,
    },
    Station {
        name: "Launceston",
        temperature: 13.1,
    },
    Station {
        name: "Lhasa",
        temperature: 7.6,
    },
    Station {
        name: "Libreville",
        temperature: 25.9,
    },
    Station {
        name: "Lisbon",
        temperature: 17.5,
    },
    Station {
        name: "Livingstone",
        temperature: 21.8,
    },
    Station {
        name: "Ljubljana",
        temperature: 10.9,
    },
    Station {
        name: "Lodwar",
        temperature: 29.3,
    },
    Station {
        name: "Lomé",
        temperature: 26.9,
    },
    Station {
        name: "London",
        temperature: 11.3,
    },
    Station {
        name: "Los Angeles",
        temperature: 18.6,
    },
    Station {
        name: "Louisville",
        temperature: 13.9,
    },
    Station {
        name: "Luanda",
        temperature: 25.8,
    },
    Station {
        name: "Lubumbashi",
        temperature: 20.8,
    },
    Station {
        name: "Lusaka",
        temperature: 19.9,
    },
    Station {
        name: "Luxembourg City",
        temperature: 9.3,
    },
    Station {
        name: "Lviv",
        temperature: 7.8,
    },
    Station {
        name: "Lyon",
        temperature: 12.5,
    },
    Station {
        name: "Madrid",
        temperature: 15.0,
    },
    Station {
        name: "Mahajanga",
        temperature: 26.3,
    },
    Station {
        name: "Makassar",
        temperature: 26.7,
    },
    Station {
        name: "Makurdi",
        temperature: 26.0,
    },
    Station {
        name: "Malabo",
        temperature: 26.3,
    },
    Station {
        name: "Malé",
        temperature: 28.0,
    },
    Station {
        name: "Managua",
        temperature: 27.3,
    },
    Station {
        name: "Manama",
        temperature: 26.5,
    },
    Station {
        name: "Mandalay",
        temperature: 28.0,
    },
    Station {
        name: "Mango",
        temperature: 28.1,
    },
    Station {
        name: "Manila",
        temperature: 28.4,
    },
    Station {
        name: "Maputo",
        temperature: 22.8,
    },
    Station {
        name: "Marrakesh",
        temperature: 19.6,
    },
    Station {
        name: "Marseille",
        temperature: 15.8,
    },
    Station {
        name: "Maun",
        temperature: 22.4,
    },
    Station {
        name: "Medan",
        temperature: 26.5,
    },
    Station {
        name: "Mek'ele",
        temperature: 22.7,
    },
    Station {
        name: "Melbourne",
        temperature: 15.1,
    },
    Station {
        name: "Memphis",
        temperature: 17.2,
    },
    Station {
        name: "Mexicali",
        temperature: 23.1,
    },
    Station {
        name: "Mexico City",
        temperature: 17.5,
    },
    Station {
        name: "Miami",
        temperature: 24.9,
    },
    Station {
        name: "Milan",
        temperature: 13.0,
    },
    Station {
        name: "Milwaukee",
        temperature: 8.9,
    },
    Station {
        name: "Minneapolis",
        temperature: 7.8,
    },
    Station {
        name: "Minsk",
        temperature: 6.7,
    },
    Station {
        name: "Mogadishu",
        temperature: 27.1,
    },
    Station {
        name: "Mombasa",
        temperature: 26.3,
    },
    Station {
        name: "Monaco",
        temperature: 16.4,
    },
    Station {
        name: "Moncton",
        temperature: 6.1,
    },
    Station {
        name: "Monterrey",
        temperature: 22.3,
    },
    Station {
        name: "Montreal",
        temperature: 6.8,
    },
    Station {
        name: "Moscow",
        temperature: 5.8,
    },
    Station {
        name: "Mumbai",
        temperature: 27.1,
    },
    Station {
        name: "Murmansk",
        temperature: 0.6,
    },
    Station {
        name: "Muscat",
        temperature: 28.0,
    },
    Station {
        name: "Mzuzu",
        temperature: 17.7,
    },
    Station {
        name: "N'Djamena",
        temperature: 28.3,
    },
    Station {
        name: "Naha",
        temperature: 23.1,
    },
    Station {
        name: "Nairobi",
        temperature: 17.8,
    },
    Station {
        name: "Nakhon Ratchasima",
        temperature: 27.3,
    },
    Station {
        name: "Napier",
        temperature: 14.6,
    },
    Station {
        name: "Napoli",
        temperature: 15.9,
    },
    Station {
        name: "Nashville",
        temperature: 15.4,
    },
    Station {
        name: "Nassau",
        temperature: 24.6,
    },
    Station {
        name: "Ndola",
        temperature: 20.3,
    },
    Station {
        name: "New Delhi",
        temperature: 25.0,
    },
    Station {
        name: "New Orleans",
        temperature: 20.7,
    },
    Station {
        name: "New York City",
        temperature: 12.9,
    },
    Station {
        name: "Ngaoundéré",
        temperature: 22.0,
    },
    Station {
        name: "Niamey",
        temperature: 29.3,
    },
    Station {
        name: "Nicosia",
        temperature: 19.7,
    },
    Station {
        name: "Niigata",
        temperature: 13.9,
    },
    Station {
        name: "Nouadhibou",
        temperature: 21.3,
    },
    Station {
        name: "Nouakchott",
        temperature: 25.7,
    },
    Station {
        name: "Novosibirsk",
        temperature: 1.7,
    },
    Station {
        name: "Nuuk",
        temperature: -1.4,
    },
    Station {
        name: "Odesa",
        temperature: 10.7,
    },
    Station {
        name: "Odienné",
        temperature: 26.0,
    },
    Station {
        name: "Oklahoma City",
        temperature: 15.9,
    },
    Station {
        name: "Omaha",
        temperature: 10.6,
    },
    Station {
        name: "Oranjestad",
        temperature: 28.1,
    },
    Station {
        name: "Oslo",
        temperature: 5.7,
    },
    Station {
        name: "Ottawa",
        temperature: 6.6,
    },
    Station {
        name: "Ouagadougou",
        temperature: 28.3,
    },
    Station {
        name: "Ouahigouya",
        temperature: 28.6,
    },
    Station {
        name: "Ouarzazate",
        temperature: 18.9,
    },
    Station {
        name: "Oulu",
        temperature: 2.7,
    },
    Station {
        name: "Palembang",
        temperature: 27.3,
    },
    Station {
        name: "Palermo",
        temperature: 18.5,
    },
    Station {
        name: "Palm Springs",
        temperature: 24.5,
    },
    Station {
        name: "Palmerston North",
        temperature: 13.2,
    },
    Station {
        name: "Panama City",
        temperature: 28.0,
    },
    Station {
        name: "Parakou",
        temperature: 26.8,
    },
    Station {
        name: "Paris",
        temperature: 12.3,
    },
    Station {
        name: "Perth",
        temperature: 18.7,
    },
    Station {
        name: "Petropavlovsk-Kamchatsky",
        temperature: 1.9,
    },
    Station {
        name: "Philadelphia",
        temperature: 13.2,
    },
    Station {
        name: "Phnom Penh",
        temperature: 28.3,
    },
    Station {
        name: "Phoenix",
        temperature: 23.9,
    },
    Station {
        name: "Pittsburgh",
        temperature: 10.8,
    },
    Station {
        name: "Podgorica",
        temperature: 15.3,
    },
    Station {
        name: "Pointe-Noire",
        temperature: 26.1,
    },
    Station {
        name: "Pontianak",
        temperature: 27.7,
    },
    Station {
        name: "Port Moresby",
        temperature: 26.9,
    },
    Station {
        name: "Port Sudan",
        temperature: 28.4,
    },
    Station {
        name: "Port Vila",
        temperature: 24.3,
    },
    Station {
        name: "Port-Gentil",
        temperature: 26.0,
    },
    Station {
        name: "Portland (OR)",
        temperature: 12.4,
    },
    Station {
        name: "Porto",
        temperature: 15.7,
    },
    Station {
        name: "Prague",
        temperature: 8.4,
    },
    Station {
        name: "Praia",
        temperature: 24.4,
    },
    Station {
        name: "Pretoria",
        temperature: 18.2,
    },
    Station {
        name: "Pyongyang",
        temperature: 10.8,
    },
    Station {
        name: "Rabat",
        temperature: 17.2,
    },
    Station {
        name: "Rangpur",
        temperature: 24.4,
    },
    Station {
        name: "Reggane",
        temperature: 28.3,
    },
    Station {
        name: "Reykjavík",
        temperature: 4.3,
    },
    Station {
        name: "Riga",
        temperature: 6.2,
    },
    Station {
        name: "Riyadh",
        temperature: 26.0,
    },
    Station {
        name: "Rome",
        temperature: 15.2,
    },
    Station {
        name: "Roseau",
        temperature: 26.2,
    },
    Station {
        name: "Rostov-on-Don",
        temperature: 9.9,
    },
    Station {
        name: "Sacramento",
        temperature: 16.3,
    },
    Station {
        name: "Saint Petersburg",
        temperature: 5.8,
    },
    Station {
        name: "Saint-Pierre",
        temperature: 5.7,
    },
    Station {
        name: "Salt Lake City",
        temperature: 11.6,
    },
    Station {
        name: "San Antonio",
        temperature: 20.8,
    },
    Station {
        name: "San Diego",
        temperature: 17.8,
    },
    Station {
        name: "San Francisco",
        temperature: 14.6,
    },
    Station {
        name: "San Jose",
        temperature: 16.4,
    },
    Station {
        name: "San José",
        temperature: 22.6,
    },
    Station {
        name: "San Juan",
        temperature: 27.2,
    },
    Station {
        name: "San Salvador",
        temperature: 23.1,
    },
    Station {
        name: "Sana'a",
        temperature: 20.0,
    },
    Station {
        name: "Santo Domingo",
        temperature: 25.9,
    },
    Station {
        name: "Sapporo",
        temperature: 8.9,
    },
    Station {
        name: "Sarajevo",
        temperature: 10.1,
    },
    Station {
        name: "Saskatoon",
        temperature: 3.3,
    },
    Station {
        name: "Seattle",
        temperature: 11.3,
    },
    Station {
        name: "Ségou",
        temperature: 28.0,
    },
    Station {
        name: "Seoul",
        temperature: 12.5,
    },
    Station {
        name: "Seville",
        temperature: 19.2,
    },
    Station {
        name: "Shanghai",
        temperature: 16.7,
    },
    Station {
        name: "Singapore",
        temperature: 27.0,
    },
    Station {
        name: "Skopje",
        temperature: 12.4,
    },
    Station {
        name: "Sochi",
        temperature: 14.2,
    },
    Station {
        name: "Sofia",
        temperature: 10.6,
    },
    Station {
        name: "Sokoto",
        temperature: 28.0,
    },
    Station {
        name: "Split",
        temperature: 16.1,
    },
    Station {
        name: "St. John's",
        temperature: 5.0,
    },
    Station {
        name: "St. Louis",
        temperature: 13.9,
    },
    Station {
        name: "Stockholm",
        temperature: 6.6,
    },
    Station {
        name: "Surabaya",
        temperature: 27.1,
    },
    Station {
        name: "Suva",
        temperature: 25.6,
    },
    Station {
        name: "Suwałki",
        temperature: 7.2,
    },
    Station {
        name: "Sydney",
        temperature: 17.7,
    },
    Station {
        name: "Tabora",
        temperature: 23.0,
    },
    Station {
        name: "Tabriz",
        temperature: 12.6,
    },
    Station {
        name: "Taipei",
        temperature: 23.0,
    },
    Station {
        name: "Tallinn",
        temperature: 6.4,
    },
    Station {
        name: "Tamale",
        temperature: 27.9,
    },
    Station {
        name: "Tamanrasset",
        temperature: 21.7,
    },
    Station {
        name: "Tampa",
        temperature: 22.9,
    },
    Station {
        name: "Tashkent",
        temperature: 14.8,
    },
    Station {
        name: "Tauranga",
        temperature: 14.8,
    },
    Station {
        name: "Tbilisi",
        temperature: 12.9,
    },
    Station {
        name: "Tegucigalpa",
        temperature: 21.7,
    },
    Station {
        name: "Tehran",
        temperature: 17.0,
    },
    Station {
        name: "Tel Aviv",
        temperature: 20.0,
    },
    Station {
        name: "Thessaloniki",
        temperature: 16.0,
    },
    Station {
        name: "Thiès",
        temperature: 24.0,
    },
    Station {
        name: "Tijuana",
        temperature: 17.8,
    },
    Station {
        name: "Timbuktu",
        temperature: 28.0,
    },
    Station {
        name: "Tirana",
        temperature: 15.2,
    },
    Station {
        name: "Toamasina",
        temperature: 23.4,
    },
    Station {
        name: "Tokyo",
        temperature: 15.4,
    },
    Station {
        name: "Toliara",
        temperature: 24.1,
    },
    Station {
        name: "Toluca",
        temperature: 12.4,
    },
    Station {
        name: "Toronto",
        temperature: 9.4,
    },
    Station {
        name: "Tripoli",
        temperature: 20.0,
    },
    Station {
        name: "Tromsø",
        temperature: 2.9,
    },
    Station {
        name: "Tucson",
        temperature: 20.9,
    },
    Station {
        name: "Tunis",
        temperature: 18.4,
    },
    Station {
        name: "Ulaanbaatar",
        temperature: -0.4,
    },
    Station {
        name: "Upington",
        temperature: 20.4,
    },
    Station {
        name: "Ürümqi",
        temperature: 7.4,
    },
    Station {
        name: "Vaduz",
        temperature: 10.1,
    },
    Station {
        name: "Valencia",
        temperature: 18.3,
    },
    Station {
        name: "Valletta",
        temperature: 18.8,
    },
    Station {
        name: "Vancouver",
        temperature: 10.4,
    },
    Station {
        name: "Veracruz",
        temperature: 25.4,
    },
    Station {
        name: "Vienna",
        temperature: 10.4,
    },
    Station {
        name: "Vientiane",
        temperature: 25.9,
    },
    Station {
        name: "Villahermosa",
        temperature: 27.1,
    },
    Station {
        name: "Vilnius",
        temperature: 6.0,
    },
    Station {
        name: "Virginia Beach",
        temperature: 15.8,
    },
    Station {
        name: "Vladivostok",
        temperature: 4.9,
    },
    Station {
        name: "Warsaw",
        temperature: 8.5,
    },
    Station {
        name: "Washington, D.C.",
        temperature: 14.6,
    },
    Station {
        name: "Wau",
        temperature: 27.8,
    },
    Station {
        name: "Wellington",
        temperature: 12.9,
    },
    Station {
        name: "Whitehorse",
        temperature: -0.1,
    },
    Station {
        name: "Wichita",
        temperature: 13.9,
    },
    Station {
        name: "Willemstad",
        temperature: 28.0,
    },
    Station {
        name: "Winnipeg",
        temperature: 3.0,
    },
    Station {
        name: "Wrocław",
        temperature: 9.6,
    },
    Station {
        name: "Xi'an",
        temperature: 14.1,
    },
    Station {
        name: "Yakutsk",
        temperature: -8.8,
    },
    Station {
        name: "Yangon",
        temperature: 27.5,
    },
    Station {
        name: "Yaoundé",
        temperature: 23.8,
    },
    Station {
        name: "Yellowknife",
        temperature: -4.3,
    },
    Station {
        name: "Yerevan",
        temperature: 12.4,
    },
    Station {
        name: "Yinchuan",
        temperature: 9.0,
    },
    Station {
        name: "Zagreb",
        temperature: 10.7,
    },
    Station {
        name: "Zanzibar City",
        temperature: 26.0,
    },
    Station {
        name: "Zürich",
        temperature: 9.3,
    },
];
