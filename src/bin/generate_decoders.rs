use std::env;
use std::fs;
use std::path::Path;
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

    cmd.args(&["--output", output_dir, "--as-crate"]);

    let output = cmd.output().map_err(|e| format!("Failed to execute Carbon CLI: {}", e))?;

    if output.status.success() {
        println!("✅ Decoder successfully generated for `{}`", decoder_name);
        Ok(())
    } else {
        Err(format!(
            "❌ Decoder generation failed for `{}`: {}",
            decoder_name,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

fn main() {
    fs::create_dir_all(DECODER_OUTPUT_DIR).expect("Failed to create decoder output directory");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("❌ Error: You must provide an IDL filename or program address.");
        eprintln!("Usage: cargo run --bin generate_decoders <idl_filename.json> OR <program_address>");
        return;
    }

    let idl_or_address = &args[1];
    let decoder_name = idl_or_address.strip_suffix(".json").unwrap_or(idl_or_address);

    // Ensure that the folder doesn't exist before generation
    if Path::new(&format!("{}/{}", DECODER_OUTPUT_DIR, decoder_name)).exists() {
        println!("⚠️  Decoder `{}` already exists. Skipping...", decoder_name);
        return;
    }

    match generate_decoder(idl_or_address, decoder_name) {
        Ok(_) => println!("✅ Successfully generated decoder for `{}`", decoder_name),
        Err(e) => eprintln!("{}", e),
    }
}
