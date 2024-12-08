//! # Rust Munich 2024 / 4 - Billion Row Challenge
//!
//! This playground is meant to give you a last chance to be part of the event in case your Rust
//! setup is completely broken and you cannot program locally. Proper I/O, large input data and
//! extensive benchmarking are not possible, but at least you get to develop an algorithmic idea and
//! dip your toe into Rust 🦀.
//!
//! ## Challenge
//!
//! Your mission, should you choose to accept it, is to write a program that retrieves temperature
//! measurement values from a text file and calculates the min, mean, and max temperature per weather
//! station.
//!
//! The input data has a simple structure with one measurement value per row (cmp. INPUT_DATA
//! constant below):
//!
//! ```txt
//! Hamburg;12.0
//! Bulawayo;8.9
//! Palembang;38.8
//! Hamburg;34.2
//! St. John's;15.2
//! Cracow;12.6
//! ... etc. ...
//! ```
//!
//! The program should print out the min, mean, and max values per station, alphabetically ordered. The
//! format that is expected varies slightly from language to language, but the following example shows
//! the expected output for the first three stations:
//!
//! ```txt
//! {
//!   Abha=5.0/18.0/27.4,
//!   Abidjan=15.7/26.0/34.1,
//!   Abéché=12.1/29.4/35.6,
//!   Accra=14.7/26.4/33.1,
//!   Addis Ababa=2.1/16.0/24.3,
//!   Adelaide=4.1/17.3/29.7,
//!   <...>
//! }
//! ```
//!
//! Don't let the setup struggles get you down, you can still learn something here! 💪

fn main() {
    // You can implement your solution here.

    for _line in INPUT_DATA.lines() {
        // TODO: Parse the input line
        // TODO: Aggregate the min/max/mean temperature by city
    }

    // TODO: Order the results by their city name alphabetically
    // TODO: Output your results here
    println!("Result: {}", "here");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        // TODO: You can add additional test cases here
        assert_eq!(1, 1);
    }
}

/// This input data matches the CSV format of the original challenge.
///
/// We put it at the end of the file since it is rather long.
const INPUT_DATA: &str = r#"
Da Nang;14.2
Wau;22.1
Benghazi;10.5
Wellington;23.5
Livingstone;28.9
Asmara;17.8
Tbilisi;8.6
Cape Town;7.8
Nuuk;-7.7
Tirana;13.6
Chicago;7.7
San Diego;5.7
Monaco;8.5
Rabat;11.5
Boise;10.6
Gagnoa;32.5
Omaha;14.3
Batumi;3.8
Belgrade;-12.1
Fresno;30.8
George Town;33.4
Garoua;25.2
Maputo;17.7
Niigata;10.3
Nuuk;5.4
Miami;33.7
Marseille;10.7
Douala;35.7
El Paso;19.3
Valletta;28.8
Novosibirsk;12.4
Portland (OR);19.7
Khartoum;24.3
Chihuahua;15.5
Lodwar;46.0
Louisville;16.5
Launceston;15.5
Pointe-Noire;18.6
Paris;-6.9
Christchurch;13.7
Port Sudan;37.3
Yaoundé;26.0
Palmerston North;17.1
Khartoum;11.4
Bangui;20.7
Tehran;7.9
Honiara;42.1
Ho Chi Minh City;22.3
Banjul;13.3
Kyiv;-3.2
Bissau;34.3
Los Angeles;21.8
Palermo;12.7
Sofia;8.3
Nouadhibou;12.3
Prague;5.9
Sokoto;26.6
Port Vila;28.2
Baltimore;8.7
Havana;27.6
Garissa;42.2
Bata;10.0
Novosibirsk;-1.6
Bangkok;41.8
Lodwar;24.1
Lake Tekapo;15.4
Flores,  Petén;23.1
Dhaka;27.4
Rostov-on-Don;23.9
Pittsburgh;19.1
Lhasa;28.0
Kunming;17.1
Hobart;42.3
Tampa;23.9
Fukuoka;10.0
Anadyr;-7.6
Baltimore;5.9
Alice Springs;30.2
Port Vila;36.2
Brisbane;26.5
Boise;-4.4
Hanga Roa;21.9
Kansas City;16.7
Banjul;17.4
Baltimore;16.0
Hong Kong;30.8
Lagos;20.0
Fianarantsoa;10.3
Hobart;17.7
Parakou;44.5
St. Louis;9.5
Odesa;-1.0
Zanzibar City;28.2
Sacramento;24.2
Baguio;20.0
Sacramento;14.4
Rostov-on-Don;6.1
Mexico City;19.4
St. John's;18.9
Pittsburgh;24.8
Moscow;20.3
Denpasar;25.8
Prague;7.5
Montreal;9.4
Phoenix;21.0
Split;-1.0
Andorra la Vella;-4.0
Skopje;14.5
Ljubljana;14.3
Adelaide;27.5
Mombasa;41.0
Pontianak;20.2
Brussels;10.9
Ndola;31.6
Khartoum;21.8
Las Palmas de Gran Canaria;19.9
Dolisie;24.4
Niigata;-4.7
Dolisie;20.5
Bucharest;6.0
Port Sudan;12.1
Lviv;19.4
Jacksonville;11.0
Rabat;15.2
Da Nang;31.6
Lyon;19.3
Suwałki;17.7
Las Vegas;2.4
Bosaso;38.0
Port Sudan;12.3
Colombo;20.9
Manama;17.3
Malé;38.3
Seville;31.4
Mogadishu;17.1
Lomé;17.0
Jacksonville;13.1
Montreal;11.0
Anadyr;13.8
Seville;14.0
Cape Town;6.5
Valencia;-7.8
Brazzaville;21.2
Suva;14.5
Kinshasa;37.3
Arkhangelsk;-15.5
Bangkok;41.7
Bucharest;12.8
Muscat;40.8
Zanzibar City;27.0
Tamale;41.3
Rome;-7.7
Canberra;5.6
Dikson;-15.3
Tauranga;8.2
Brussels;4.3
Guangzhou;28.5
Upington;21.8
Warsaw;13.0
Baltimore;7.7
Oulu;-5.5
Kyiv;22.6
Beirut;17.5
Ankara;8.3
Saint-Pierre;-8.2
Kumasi;16.7
Harare;8.2
Louisville;3.4
Irkutsk;11.7
Rangpur;6.7
Yangon;14.6
Pretoria;8.8
Tijuana;11.7
London;12.3
Irkutsk;4.6
La Ceiba;16.5
Guadalajara;18.2
Gagnoa;37.0
İzmir;12.9
Seattle;7.8
Wrocław;4.3
Belize City;26.4
Bloemfontein;21.5
Guangzhou;35.7
Entebbe;32.2
Milwaukee;12.6
Perth;7.3
Marrakesh;4.7
Birao;36.5
Tallinn;5.7
Bouaké;47.6
Algiers;13.9
Kyoto;18.3
Roseau;45.9
Mandalay;19.2
Malabo;20.1
Tokyo;14.7
Ürümqi;-4.6
Timbuktu;38.9
Tromsø;-11.3
Kyoto;7.0
Hiroshima;3.3
Bujumbura;27.0
Ashgabat;23.5
Charlotte;22.2
Brussels;-5.6
Kathmandu;20.5
Oranjestad;23.8
Luxembourg City;11.0
Kingston;18.8
Edinburgh;4.1
Lake Havasu City;35.9
San Juan;30.7
Managua;32.9
Detroit;8.9
Kankan;33.9
Las Palmas de Gran Canaria;13.1
Chicago;9.6
Sydney;27.6
Lahore;28.6
Budapest;24.5
George Town;47.1
Guadalajara;14.4
Tallinn;-10.4
Banjul;44.3
Tamale;42.7
Tegucigalpa;11.5
Saint-Pierre;3.2
Athens;11.5
Valletta;2.9
Blantyre;25.6
Skopje;5.2
Napier;5.5
Wichita;6.0
Xi'an;11.0
Kinshasa;34.7
Ségou;18.6
Andorra la Vella;7.3
Vilnius;-6.3
Vancouver;13.8
Oslo;-4.1
Napier;17.1
Cape Town;11.1
Vienna;-6.4
Heraklion;14.5
Toronto;1.2
Calgary;19.6
Belize City;35.2
Bangui;12.2
Kumasi;13.8
Boston;7.1
Detroit;23.6
Warsaw;15.0
Chihuahua;-6.9
Addis Ababa;13.3
Dallas;15.4
Barcelona;21.3
Algiers;15.6
Naha;19.6
Garissa;35.1
Pointe-Noire;30.5
Vientiane;33.8
Antsiranana;19.7
Denver;14.0
Tromsø;19.3
Lusaka;20.2
Managua;20.2
Tucson;21.8
Singapore;11.9
Budapest;46.9
Tauranga;1.3
Da Nang;15.0
Kinshasa;39.7
Ürümqi;-2.9
Mek'ele;24.9
Kandi;23.7
Los Angeles;13.1
Kyoto;6.5
Stockholm;8.7
Ottawa;8.6
San Jose;7.4
Ürümqi;-11.4
Bratislava;18.0
Malé;38.8
Harbin;12.4
Mango;45.8
Baku;-4.3
San Francisco;32.2
Harbin;-2.9
Busan;24.1
Jerusalem;5.2
Tehran;31.3
Sydney;18.9
Ankara;17.5
Phoenix;10.4
Bangkok;21.6
Iqaluit;-14.4
Lahore;14.0
Warsaw;4.4
San Francisco;13.7
Honolulu;24.0
Muscat;11.4
Pretoria;26.8
Fairbanks;-16.5
Sapporo;18.9
Oranjestad;23.5
Alice Springs;4.8
Garissa;34.7
Vilnius;0.1
Gjoa Haven;-6.8
Antananarivo;56.3
Garoua;25.8
Bata;12.2
Dakar;39.6
Napoli;5.8
Bangui;45.3
Khartoum;42.3
Flores,  Petén;25.5
Canberra;17.1
Brisbane;-7.6
Nassau;19.7
Dushanbe;27.2
Tbilisi;14.6
Kathmandu;4.5
Lahore;25.4
Hat Yai;32.3
Da Nang;21.9
Parakou;35.9
Nouakchott;23.5
Fresno;-3.4
Fresno;24.8
Mango;26.3
Tashkent;14.4
Erzurum;16.2
Ankara;10.5
Dili;13.5
Albuquerque;40.6
St. John's;-8.3
Rangpur;45.4
Athens;11.7
Petropavlovsk-Kamchatsky;-5.8
Lubumbashi;8.3
Odienné;33.3
Bouaké;29.6
Marrakesh;17.3
Detroit;12.4
Asmara;27.1
Lyon;8.9
Mek'ele;22.8
Gagnoa;36.2
Edmonton;-27.8
Hanga Roa;13.9
Edmonton;11.9
Chicago;2.4
Prague;-9.0
Vientiane;17.6
Lodwar;31.6
Tabriz;30.4
Napoli;2.7
Fairbanks;10.6
Budapest;11.3
Perth;18.1
Banjul;28.4
Phoenix;26.5
Lomé;30.0
Lahore;23.8
Lodwar;39.3
Yellowknife;-17.4
Almaty;9.5
Wau;21.9
Pontianak;16.6
Tauranga;-1.3
San Jose;16.1
Luanda;38.5
Dakar;29.7
Austin;7.7
Montreal;-12.5
Sokoto;19.6
Cairns;17.6
Oranjestad;25.7
Bosaso;27.8
Maun;15.2
Aden;25.1
Makassar;27.6
Sana'a;32.4
Saint Petersburg;1.3
St. John's;16.9
Mombasa;28.0
Boise;5.5
Antananarivo;22.8
Portland (OR);11.6
Makassar;24.7
Kuwait City;28.9
Charlotte;8.7
Niamey;31.1
Irkutsk;2.3
Oranjestad;41.2
Mahajanga;23.2
Luanda;30.1
Antsiranana;33.4
Winnipeg;2.4
Winnipeg;-10.7
Bangkok;24.5
Gangtok;39.8
Monaco;27.5
Christchurch;28.2
Bilbao;17.8
Seattle;15.1
Kuopio;3.0
Port Vila;26.6
Tamale;19.7
New Orleans;18.5
Tel Aviv;-0.9
Honiara;18.5
Roseau;34.0
Louisville;15.9
Livingstone;20.5
Pyongyang;19.5
Dunedin;6.4
Bulawayo;29.3
Ouahigouya;26.7
Dampier;28.9
Port Moresby;18.3
Tbilisi;5.1
Kunming;33.8
Mombasa;31.0
Ngaoundéré;33.8
Athens;39.4
La Ceiba;15.9
Sofia;17.6
Astana;-0.6
Tampa;10.4
Hat Yai;32.7
Jacksonville;13.5
Hiroshima;15.8
Toronto;19.7
Monterrey;33.2
Bata;31.8
Mogadishu;20.0
Kinshasa;25.8
Guangzhou;8.6
Memphis;17.7
Mexicali;23.3
Ürümqi;9.2
Paris;37.0
London;25.3
Villahermosa;37.0
Ürümqi;6.1
Sapporo;6.4
Bujumbura;12.4
Busan;8.5
Dunedin;28.4
New Orleans;15.3
Sydney;6.8
Edinburgh;23.4
Manama;36.4
Tbilisi;21.6
Kyoto;8.3
Tbilisi;6.2
Phnom Penh;29.8
Chiang Mai;16.7
El Paso;16.9
Chicago;0.2
Vaduz;19.6
San Antonio;41.2
Mexicali;17.5
Port Sudan;13.6
Lisbon;31.9
City of San Marino;16.4
Dampier;23.9
Malabo;25.8
Bordeaux;17.3
Kano;31.1
Harare;8.3
Dunedin;12.4
Juba;23.0
Albuquerque;12.5
Hong Kong;11.7
Chiang Mai;20.5
Fianarantsoa;29.9
Charlotte;28.6
Las Palmas de Gran Canaria;25.8
Bucharest;2.6
Saskatoon;10.2
Benghazi;12.3
Algiers;43.6
Saint Petersburg;-3.4
Palermo;14.1
Naha;27.7
Vientiane;31.0
Murmansk;6.4
Reggane;25.4
Calgary;16.7
Toliara;28.8
Honiara;33.2
Tauranga;-1.9
Irkutsk;-4.5
Pyongyang;23.7
Kuopio;1.5
Chiang Mai;37.0
Santo Domingo;30.4
Ashgabat;29.3
Niigata;19.6
Anchorage;-14.9
Dikson;-1.7
Lake Havasu City;25.6
Hobart;22.8
Assab;46.3
Istanbul;24.6
Valletta;1.2
City of San Marino;14.9
City of San Marino;17.0
Shanghai;16.4
Manila;29.0
Parakou;14.6
Garissa;37.1
Yaoundé;9.0
Lake Tekapo;18.9
Oslo;-6.6
Porto;9.2
Vilnius;16.8
Denver;9.4
Karonga;7.1
Bouaké;33.7
Ndola;13.1
Honolulu;17.4
Podgorica;27.7
Entebbe;8.3
Vladivostok;14.6
Kansas City;32.4
Montreal;9.3
Fresno;16.6
San José;36.6
Antsiranana;4.1
Amsterdam;18.0
Tucson;19.5
Makurdi;44.8
Brazzaville;11.3
Rangpur;24.3
Berlin;15.4
Malé;29.2
Cotonou;31.8
Tripoli;6.3
Medan;14.6
Petropavlovsk-Kamchatsky;0.4
Lomé;8.9
Da Nang;21.9
Antsiranana;11.2
Lusaka;17.6
Antsiranana;27.5
Washington, D.C.;15.0
Baku;22.9
Guatemala City;26.4
Pointe-Noire;22.4
Dallas;19.9
Lahore;21.2
Portland (OR);12.3
Riga;9.3
Kampala;18.8
Batumi;11.1
Ouagadougou;42.9
Ouahigouya;25.6
Bloemfontein;2.1
Nouadhibou;14.4
Auckland;19.1
Dushanbe;28.2
Riyadh;30.3
Marseille;21.7
Las Palmas de Gran Canaria;30.1
Jos;20.4
Beirut;6.1
San Francisco;3.1
Tabora;41.1
Mombasa;43.7
Darwin;28.8
Parakou;36.3
Riga;12.2
Sacramento;17.6
Port-Gentil;27.3
Tabriz;4.7
New York City;0.6
Dushanbe;21.5
Assab;34.8
Omaha;22.1
Muscat;21.7
Boise;12.1
Tunis;27.2
Guangzhou;14.3
Reggane;38.3
N'Djamena;22.2
Timbuktu;29.3
Sofia;20.8
Jerusalem;14.6
Helsinki;1.0
New Delhi;24.3
Yaoundé;17.3
London;-2.9
Flores,  Petén;24.3
Cairns;35.2
Tijuana;9.6
Austin;13.9
Chicago;15.4
Anchorage;22.3
Port Vila;48.4
Milwaukee;13.0
Colombo;38.3
Bamako;40.9
Riyadh;33.4
Novosibirsk;18.7
Barcelona;12.7
Fianarantsoa;11.4
Atlanta;18.0
Luanda;26.4
Lyon;8.0
Tauranga;14.0
Podgorica;21.2
Nuuk;13.1
Flores,  Petén;29.8
Burnie;7.9
Denpasar;33.3
La Ceiba;42.3
Albuquerque;11.0
Da Lat;10.5
Mombasa;36.4
Dhaka;24.5
Suwałki;20.0
Philadelphia;-3.6
Veracruz;33.6
Barcelona;29.8
Ngaoundéré;25.4
Oulu;3.1
Montreal;5.3
Dikson;-9.2
Adelaide;16.7
Chongqing;29.5
Petropavlovsk-Kamchatsky;23.6
Valencia;11.5
Port Vila;25.3
Tabriz;22.2
Zürich;4.3
Dallas;4.7
Moscow;3.8
Da Lat;20.1
Ahvaz;33.8
Tirana;18.1
Ürümqi;5.9
Budapest;16.6
Palm Springs;29.5
Kano;51.7
Conakry;23.4
Irkutsk;8.3
Kathmandu;18.8
Murmansk;-24.0
Maun;29.4
Seoul;13.6
Abéché;42.8
Lisbon;31.0
Ashgabat;24.6
Ouahigouya;44.7
Erzurum;24.6
Phoenix;21.8
Kumasi;32.4
Harbin;0.2
Istanbul;20.9
Whitehorse;-13.7
Wau;24.4
Abha;9.3
Baguio;24.9
Tehran;15.2
Sapporo;14.7
Khartoum;25.2
Yerevan;3.5
Kyoto;28.6
Podgorica;20.9
Tabora;20.6
Las Palmas de Gran Canaria;40.2
Bloemfontein;25.1
Libreville;12.8
Atlanta;2.8
Denver;14.3
Gangtok;22.4
Jayapura;46.3
Christchurch;17.4
Washington, D.C.;12.9
Nouadhibou;19.3
Riyadh;32.2
Yinchuan;-3.6
Dar es Salaam;22.5
Mexicali;11.5
Oklahoma City;10.0
Malé;37.4
Tamanrasset;28.8
Ljubljana;10.0
Ahvaz;21.1
Tunis;5.0
Erbil;15.2
Santo Domingo;20.4
Maputo;41.8
Dodoma;17.1
Da Lat;35.5
Lviv;13.8
Gangtok;9.8
Beirut;21.9
Mek'ele;35.9
Toamasina;24.3
Kumasi;30.9
Pyongyang;2.9
Bergen;-0.8
Chiang Mai;27.0
Milwaukee;2.7
Dubai;12.6
Bujumbura;25.8
Muscat;14.7
Sapporo;11.9
Sydney;7.2
Melbourne;3.6
Pretoria;19.5
Virginia Beach;24.2
San Jose;27.3
Douala;27.5
Tromsø;0.1
Yaoundé;20.7
Mogadishu;11.3
Napoli;10.3
Bilbao;14.6
Sapporo;19.3
Mandalay;36.6
Havana;32.1
Fresno;7.4
La Paz;35.8
Anadyr;-3.6
Kuwait City;27.8
Alexandria;27.4
Fresno;8.1
Pointe-Noire;21.2
London;7.8
Canberra;11.2
Dampier;3.7
Sana'a;18.5
Copenhagen;6.0
Cairns;25.4
Riga;-1.4
Burnie;7.3
Baku;26.9
Alexandra;14.3
Odienné;17.7
Karonga;15.9
Nairobi;16.9
Barcelona;5.8
Antananarivo;3.0
Rabat;20.7
Philadelphia;9.6
Kinshasa;23.6
Boston;20.0
Bujumbura;29.7
Karachi;26.3
Gaborone;7.9
Conakry;36.1
Tabriz;7.5
Malabo;33.6
Lviv;20.7
Pretoria;32.1
Algiers;1.1
Mumbai;16.7
Kathmandu;21.7
Conakry;36.3
Karonga;31.8
Willemstad;27.6
Hong Kong;23.4
Kumasi;25.8
Burnie;13.6
Nicosia;17.1
Seville;18.0
Jakarta;29.7
Cairns;21.6
Banjul;38.6
Saint Petersburg;6.7
Yellowknife;-13.0
Kolkata;31.6
Livingstone;7.4
Bloemfontein;0.3
Iqaluit;-16.1
Manila;37.7
Colombo;22.0
Odesa;8.3
Yangon;21.4
Port Vila;5.5
Entebbe;10.2
Anadyr;-20.6
Lahore;35.4
Bata;36.7
Monaco;20.8
Nicosia;22.6
Luxembourg City;22.2
Juba;8.9
Guatemala City;12.8
Chongqing;21.6
Riyadh;34.4
Chongqing;14.1
Mzuzu;6.0
Bishkek;-9.8
Zagreb;19.3
Phoenix;21.7
Abéché;56.8
Nairobi;15.1
Tehran;13.1
Managua;30.4
Vienna;26.7
Albuquerque;19.0
Hargeisa;16.1
Pittsburgh;10.0
Melbourne;12.9
Kansas City;30.8
St. John's;10.3
Stockholm;3.5
Kuala Lumpur;32.9
Tbilisi;14.0
Tegucigalpa;35.7
Seattle;0.5
Fresno;26.8
Erbil;9.4
Ngaoundéré;8.6
Zanzibar City;26.1
Athens;31.0
Amsterdam;13.2
Lomé;30.0
Gangtok;7.6
Makurdi;26.9
Ottawa;7.8
Dar es Salaam;18.7
Palermo;15.1
Cotonou;13.5
Ashgabat;29.9
Wau;32.0
Hong Kong;5.7
Perth;34.8
Adelaide;2.9
Tashkent;3.5
Philadelphia;8.5
Almaty;23.4
Birao;18.5
Bridgetown;28.5
San Jose;21.1
Rostov-on-Don;10.0
Roseau;31.1
Dushanbe;22.2
Bangkok;36.6
Mexicali;24.6
Lhasa;23.9
Chișinău;-10.6
Tirana;13.3
Honolulu;23.8
Tijuana;10.7
Edinburgh;12.2
Palermo;31.0
Ouarzazate;7.3
Heraklion;33.6
Taipei;4.0
Tromsø;3.4
Rangpur;14.0
Milwaukee;-10.0
Xi'an;19.4
Wau;38.3
Sapporo;9.1
Timbuktu;13.6
Auckland;27.0
Rostov-on-Don;13.9
Pittsburgh;18.1
Busan;11.3
Sacramento;15.4
Da Nang;33.2
Hiroshima;10.4
Libreville;13.8
Skopje;5.9
Columbus;4.2
İzmir;1.2
Dakar;26.3
Entebbe;22.6
Willemstad;32.3
Nashville;36.8
Kabul;9.7
Karonga;20.2
Omaha;8.9
Christchurch;3.1
Port Sudan;32.9
Ifrane;8.3
Tabora;25.3
Paris;31.9
Whitehorse;3.7
Cotonou;17.3
Mek'ele;30.8
Tel Aviv;11.7
Sana'a;14.0
Brazzaville;20.6
Marrakesh;23.7
Durban;5.8
Guangzhou;27.3
Saint-Pierre;0.5
Hat Yai;26.1
Arkhangelsk;-1.2
Milan;18.7
Lubumbashi;27.1
Sydney;20.3
Ouagadougou;35.0
Port Sudan;33.6
Pretoria;18.7
Hiroshima;15.9
Sydney;14.0
Mexico City;20.2
Dallas;19.1
Kumasi;36.3
Toamasina;33.6
Busan;13.9
Mahajanga;31.4
Kuwait City;30.5
Houston;25.1
Porto;11.4
Nakhon Ratchasima;34.4
Darwin;34.5
Irkutsk;-7.8
Dampier;35.7
Tokyo;10.7
Pontianak;46.0
Ürümqi;6.6
Dhaka;38.1
Willemstad;32.1
Moscow;16.4
Khartoum;21.3
Surabaya;44.5
Mexico City;16.4
Bratislava;4.8
Ouahigouya;32.3
Parakou;12.0
Marseille;31.9
Bosaso;15.2
Medan;40.3
Hargeisa;11.1
Milan;6.0
Salt Lake City;3.6
Blantyre;11.5
Calgary;6.6
Guadalajara;35.4
Dampier;14.1
Flores,  Petén;33.0
Anchorage;-5.3
Bangkok;36.3
Kuwait City;22.1
Benghazi;1.9
Yerevan;7.6
Albuquerque;9.8
City of San Marino;32.7
Makassar;32.1
George Town;6.4
Kunming;15.4
Kyiv;2.9
Conakry;39.7
Birao;21.5
Alice Springs;24.7
Kuwait City;18.9
Seoul;6.5
Warsaw;41.2
Nassau;31.0
Paris;13.5
Dar es Salaam;49.0
Wellington;-5.7
Oranjestad;12.2
Panama City;36.3
Dunedin;-3.4
Lake Tekapo;13.3
Harare;29.2
Tampa;11.7
Vienna;22.1
Melbourne;25.6
Ouahigouya;14.8
Upington;-2.3
Mexicali;24.4
Riyadh;6.8
Chihuahua;24.3
Abha;18.5
Ljubljana;30.0
Nouakchott;29.1
Yellowknife;-11.9
Almaty;23.7
Bujumbura;33.7
Abidjan;19.9
Makassar;34.0
Abidjan;42.9
Kyoto;15.6
Busan;15.6
Suva;32.3
Wellington;17.1
Praia;19.3
Reykjavík;-0.8
Hanga Roa;13.2
Mogadishu;24.7
Yinchuan;15.4
Ségou;18.1
Skopje;15.2
Chittagong;40.0
Tel Aviv;24.9
"#;
