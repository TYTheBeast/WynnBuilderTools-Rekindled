mod apparel;
mod atk_spd;
mod class;
mod comm_stat;
mod dam;
mod damages;
mod item;
mod point;
mod range;
mod weapon;

pub use apparel::*;
pub use atk_spd::*;
pub use class::*;
pub use comm_stat::*;
pub use dam::*;
pub use damages::*;
pub use item::*;
pub use point::*;
pub use range::*;
pub use weapon::*;

use crate::build_config::Api;
use crate::config::build_config::Config;
use std::io::Write;
use std::{fs::File, io::BufReader, path::Path};

/// Load items from a JSON file
///
/// # Arguments
///
/// - `path` - A path to the JSON file
///
/// # Returns
///
/// A tuple containing a list of gear and weapons
///
/// # Example
///
/// ```rust
/// use std::path::Path;
/// use items::load_from_json;
///
/// let (apparels, weapons) = load_from_json(Path::new("items.json"));
/// ```
///
/// # Panics
///
/// This function will panic if the file cannot be opened or if the JSON file is invalid
pub fn load_from_json<P>(path: P, config: &Config) -> ([Vec<Apparel>; 7], Vec<Weapon>)
where
    P: AsRef<Path>,
{
    let file = match File::open(&path) {
        Ok(ok) => ok,
        Err(_) => {
            let defaults = Api {
                url: "https://api.wynncraft.com".to_string(),
                version: "v3".to_string(),
                module: "item".to_string(),
                query: "database?fullResult".to_string(),
            };

            let request_url = format!(
                "{url}/{version}/{module}/{query}",
                url = config.api.as_ref().unwrap_or_else(|| { &defaults }).url,
                version = config.api.as_ref().unwrap_or_else(|| { &defaults }).version,
                module = config.api.as_ref().unwrap_or_else(|| { &defaults }).module,
                query = config.api.as_ref().unwrap_or_else(|| { &defaults }).query,

            );

            let response = reqwest::blocking::get(&request_url)
                .expect("reqwest should be able to contact the wynncraft api");
            let items: Items = response.json().expect("the response should be valid json");

            // Serialize items to JSON string
            let json_string =
                serde_json::to_string(&items).expect("Failed to serialize items to JSON");

            // Open file for writing
            let mut file = File::create(&path).expect("Failed to create file");

            // Write JSON string to file
            file.write_all(json_string.as_bytes())
                .expect("Failed to write JSON to file");

            file
        }
    };

    let reader = BufReader::new(file);

    let items: Items = serde_json::from_reader(reader).unwrap();
    let mut apparels: [Vec<Apparel>; 7] = Default::default();
    let mut weapons: Vec<Weapon> = Vec::new();
    items
        .items
        .iter()
        .for_each(|value| match value.r#type.as_str() {
            "helmet" => apparels[0].push(Apparel::try_from(value).unwrap()),
            "chestplate" => apparels[1].push(Apparel::try_from(value).unwrap()),
            "leggings" => apparels[2].push(Apparel::try_from(value).unwrap()),
            "boots" => apparels[3].push(Apparel::try_from(value).unwrap()),
            "ring" => apparels[4].push(Apparel::try_from(value).unwrap()),
            "bracelet" => apparels[5].push(Apparel::try_from(value).unwrap()),
            "necklace" => apparels[6].push(Apparel::try_from(value).unwrap()),
            "relik" | "bow" | "wand" | "dagger" | "spear" => {
                weapons.push(Weapon::try_from(value).unwrap())
            }
            _ => (),
        });

    (apparels, weapons)
}
