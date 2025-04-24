use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const DECODER_OUTPUT_DIR: &str = "generated_decoders";
const IDL_INPUT_DIR: &str = "idl_inputs";

fn generate_decoder(idl_or_address: &str, decoder_name: &str) -> Result<(), String> {
    let output_dir = DECODER_OUTPUT_DIR;

    let mut cmd = Command::new("carbon-cli");
    cmd.arg("parse");

    if idl_or_address.ends_with(".json") {
        cmd.args(&["--idl", &format!("{}/{}", IDL_INPUT_DIR, idl_or_address)]);
    } else {
        cmd.args(&["--idl", idl_or_address, "-u", "mainnet-beta"]);
    }

    cmd.args(&["--output", output_dir]);

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to execute Carbon CLI: {}", e))?;

    if output.status.success() {
        println!("✅ Decoder successfully generated for `{}`", decoder_name);

        // Ensure the folder is renamed
        rename_decoder_folder(decoder_name)?;

        // Update mod.rs to reflect the new name
        update_mod_rs()?;
        Ok(())
    } else {
        Err(format!(
            "❌ Decoder generation failed for `{}`: {}",
            decoder_name,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// Renames the decoder folder to replace `-` with `_` and handle `-decoder` suffix
fn rename_decoder_folder(original_name: &str) -> Result<(), String> {
    let expected_generated_name = format!("{}-decoder", original_name); // Carbon adds `-decoder`
    let original_path = PathBuf::from(format!(
        "{}/{}",
        DECODER_OUTPUT_DIR, expected_generated_name
    ));
    let valid_name = original_name.replace("-", "_"); // Convert `-` to `_`
    let new_path = PathBuf::from(format!("{}/{}", DECODER_OUTPUT_DIR, valid_name));

    // Debug print to check if paths exist
    println!(
        "🔍 Checking folder rename: `{}` -> `{}`",
        original_path.display(),
        new_path.display()
    );

    // Ensure the generated folder exists before renaming
    if original_path.exists() && expected_generated_name != valid_name {
        println!(
            "🛠 Renaming `{}` to `{}`...",
            expected_generated_name, valid_name
        );

        // Try renaming
        match fs::rename(&original_path, &new_path) {
            Ok(_) => println!(
                "✅ Successfully renamed `{}` to `{}`",
                expected_generated_name, valid_name
            ),
            Err(e) => {
                return Err(format!(
                    "❌ Failed to rename `{}` to `{}`: {}",
                    expected_generated_name, valid_name, e
                ))
            }
        }
    } else {
        println!(
            "⚠️ Folder `{}` does not exist or name is already valid.",
            expected_generated_name
        );
    }

    Ok(())
}

/// Updates `mod.rs` to include all generated decoders
fn update_mod_rs() -> Result<(), String> {
    let mod_file_path = format!("{}/mod.rs", DECODER_OUTPUT_DIR);
    let mut mod_file =
        fs::File::create(&mod_file_path).map_err(|e| format!("Failed to create mod.rs: {}", e))?;

    let mut mod_content = String::new();
    let entries = fs::read_dir(DECODER_OUTPUT_DIR)
        .map_err(|e| format!("Failed to read decoder directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let decoder_name = entry.file_name().into_string().unwrap();

        if entry.path().is_dir() {
            let valid_name = decoder_name.replace("-", "_");
            mod_content.push_str(&format!("\n#[allow(clippy::all)]\npub mod {};", valid_name));
        }
    }

    mod_file
        .write_all(mod_content.as_bytes())
        .map_err(|e| format!("Failed to write to mod.rs: {}", e))?;
    println!("🔄 Updated mod.rs with available decoders.");
    Ok(())
}

fn main() {
    fs::create_dir_all(DECODER_OUTPUT_DIR).expect("Failed to create decoder output directory");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("❌ Error: You must provide an IDL filename or program address.");
        eprintln!(
            "Usage: cargo run --bin generate_decoders <idl_filename.json> OR <program_address>"
        );
        return;
    }

    let idl_or_address = &args[1];
    let decoder_name = idl_or_address
        .strip_suffix(".json")
        .unwrap_or(idl_or_address);

    // Ensure "-" is replaced with "_"
    let valid_name = decoder_name.replace("-", "_");

    // Ensure that the expected generated folder does not already exist
    let expected_generated_name = format!("{}-decoder", decoder_name);
    if Path::new(&format!("{}/{}", DECODER_OUTPUT_DIR, valid_name)).exists()
        || Path::new(&format!(
            "{}/{}",
            DECODER_OUTPUT_DIR, expected_generated_name
        ))
        .exists()
    {
        println!("⚠️  Decoder `{}` already exists. Skipping...", valid_name);
        return;
    }

    match generate_decoder(idl_or_address, decoder_name) {
        Ok(_) => println!("✅ Successfully generated decoder for `{}`", valid_name),
        Err(e) => eprintln!("{}", e),
    }
}
