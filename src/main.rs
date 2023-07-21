use std::error::Error;
use std::fs::File;
use csv::WriterBuilder;
use reqwest;
use chrono::Local;

mod model;

fn main() -> Result<(), Box<dyn Error>> {    
    // Lire les cookies à partir du fichier TOML
    let cookies_str = std::fs::read_to_string("cookies.toml")?;
    let cookies: toml::Value = toml::from_str(&cookies_str)?;

    // Créer la chaîne de cookie pour le header de la requête
    let cookie_value = cookies
        .get("cookies")
        .and_then(|c| c.as_table())
        .map(|cookies_table| {
            cookies_table
                .iter()
                .filter_map(|(key, value)| value.as_str().map(|v| format!("{}={};", key, v)))
                .collect::<String>()
        })
        .unwrap_or_default();

    // Créer un client HTTP avec les cookies dans les en-têtes
    let client = reqwest::blocking::Client::builder()
        .default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::COOKIE,
                reqwest::header::HeaderValue::from_str(&cookie_value).unwrap(),
            );
            headers
        })
        .build()?;

    let response = client
        .get("https://api.foodles.co/api/fridge/")
        .send()?;

    // S'assurer de la réussite de la réponse
    response.error_for_status_ref()?;

    // Read the JSON data from the response
    let data: model::Fridge = response.json()?;

    // Get the current date and time
    let current_date = Local::now();
    let formatted_date = current_date.format("%Y-%m-%d_%H-%M-%S").to_string();

    // Create the file name with the date
    let output_file_name = format!("stats/Frigo_{}.csv", formatted_date);

    // Créez le fichier CSV et écrivez les en-têtes
    let file = File::create(output_file_name)?;
    let mut writer = WriterBuilder::new()
                                                            .has_headers(true)
                                                            .delimiter(b';')
                                                            .from_writer(file);
    writer.write_record(&["ID", "Nom", "Réduction date courte", "Quantité", "Prix"])?;

    // Écrivez chaque produit dans le fichier CSV
    for category in data.categories {
        for product in category.products {
            let date_courte = if product.has_near_expiration_sale { "Vrais" } else { "Faux" };
            writer.write_record(&[
                product.id.to_string(),
                product.name,
                date_courte.to_string(),
                product.quantity.to_string(),
                format!("{:.2}", product.price.amount as f32 / 100.0),
            ])?;
        }
    }

    // Vérifiez que tout s'est bien passé et fermez le fichier
    writer.flush()?;

    // Write the CSV data in Windows ASCII encoding
    // let output_bytes = iconv::encode(&writer, "WINDOWS-1251").unwrap();
    // //let output_bytes = WINDOWS_1252.encode(&writer.into_inner().unwrap().into_inner().unwrap(), EncoderTrap::Strict).unwrap();
    // let mut file = File::create(output_file_name)?;
    // file.write_all(&output_bytes)?;

    println!("Données extraites et écrites dans 'output.csv' avec succès.");

    Ok(())
}
