use std::error::Error;
use std::fs;
use std::fs::{ File };
use std::path::Path;
use csv::WriterBuilder;
use foodles_api::fridge;
use chrono::Local;

/// Get the cookies from the TOML file
fn get_cookies() -> Result<String, Box<dyn Error>> {
    // Read the cookies from the TOML file
    let cookies_str = std::fs::read_to_string("cookies.toml")?;
    let cookies: toml::Value = toml::from_str(&cookies_str)?;

    // Create the cookie string for the request header
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

    Ok(cookie_value)
}

/// Get the fridge data and write it in a CSV file
fn get_fridge_data_and_write_in_csv(fridge_date: &fridge::model::Fridge) -> Result<String, Box<dyn Error>> {
    // Get the current date and time
    let current_date = Local::now();
    let formatted_date = current_date.format("%Y-%m-%d_%H-%M-%S").to_string();

    // Create the file name with the date
    let output_dir_name = "stats";
    let output_file_name = format!("Frigo_{}.csv", formatted_date);

    // Créez le fichier CSV et écrivez les en-têtes
    fs::create_dir_all(output_dir_name)?;
    let file = File::create((Path::new(".")).join(output_dir_name).join(&output_file_name))?;
    let mut writer = WriterBuilder::new()
                                   .has_headers(true)
                                   .delimiter(b';')
                                   .from_writer(file);
    writer.write_record(&["ID", "Réduction date courte", "Quantité", "Prix"])?;

    // Écrivez chaque produit dans le fichier CSV
    for category in &fridge_date.categories {
        for product in &category.products {
            let date_courte = if product.has_near_expiration_sale { "Vrais" } else { "Faux" };
            writer.write_record(&[
                product.id.to_string(),
                date_courte.to_string(),
                product.quantity.to_string(),
                format!("{:.2}", product.price.amount as f32 / 100.0),
            ])?;
        }
    }

    // Vérifiez que tout s'est bien passé et fermez le fichier
    writer.flush()?;

    Ok(output_file_name)
}

/// Update the index file for new products
fn update_index_file(fridge_date: &fridge::model::Fridge) -> Result<String, Box<dyn Error>> {
    // Create the file name with the date
    let output_dir_name = "stats";
    let output_file_name = "Fridge_index.csv";

    // Créez le fichier CSV et écrivez les en-têtes
    fs::create_dir_all(output_dir_name)?;
    let file = File::create((Path::new(".")).join(output_dir_name).join(&output_file_name))?;
    let mut writer = WriterBuilder::new()
                                   .has_headers(true)
                                   .delimiter(b';')
                                   .from_writer(file);
                          // id, name, description, image, icons, ingredients, trace_allergens, allergens, weight, energy_kcal, fat_content, fat_content_ags, glucides, glucides_sugar, fibres, proteins, salt, nutriscore
    writer.write_record(&["ID", "Nom", "Description", "Image", "Icons", "Ingrédients", "Traces d'allergènes", "Allergènes", "Poids", "Énergie (kCal) pour 100g", "Lipides", "Dont acides gras saturés", "Glucides", "Dont sucres", "Fibres", "Protéines", "Sel", "NUTRI-SCORE" ])?;

    // Écrivez chaque produit dans le fichier CSV
    for category in &fridge_date.categories {
        for product in &category.products {
            //let icons = product.icons.iter().map(|icon| icon.name.to_string()).collect::<Vec<String>>().join(", ");
            //let trace_allergens = product.trace_allergens.iter().map(|allergen| allergen.to_string()).collect::<Vec<String>>().join(", ");
            writer.write_record(&[
                product.id.to_string(),
                product.name.to_string(),
                product.description.to_string(),
                "".to_string(),//product.image.to_string(),
                "test".to_string(),//icons
                product.ingredients.to_string(),
                "test".to_string(),//trace_allergens
                product.allergens.to_string(),
                product.weight.to_string(),
                product.energy_kcal.to_string(),
                product.fat_content.to_string(),
                product.fat_content_ags.to_string(),
                product.glucides.to_string(),
                product.glucides_sugar.to_string(),
                product.fibres.to_string(),
                product.proteins.to_string(),
                product.salt.to_string(),
                "".to_string()//product.nutriscore.unwrap_or("".to_string()).to_string(),
            ])?;
        }
    }

    // Vérifiez que tout s'est bien passé et fermez le fichier
    writer.flush()?;

    Ok(output_file_name.to_string())
}

fn main() -> Result<(), Box<dyn Error>> {    
    let cookie_value = get_cookies();
    let data = fridge::get(&cookie_value?)?;

    let csv_path = get_fridge_data_and_write_in_csv(&data)?;
    println!("Données extraites et écrites dans '{}' avec succès.", csv_path);

    let index_path = update_index_file(&data)?;
    println!("Index mis à jour avec succès.");

    Ok(())
}
