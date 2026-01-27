# netrunner-v2-decoders

Generates type-safe Rust decoders for Solana programs using [Carbon](https://github.com/sevenlabs-hq/carbon). Created to support [**Netrunner**](https://github.com/phaselabscrypto/netrunner-v2-api/) in decoding transactions and account data.

## Description

`netrunner-v2-decoders` is a CLI utility that:
1.  Takes a Solana program address or a local IDL file as input.
2.  Uses [`carbon-cli`](https://github.com/sevenlabs-hq/carbon) to generate Rust decoders.
3.  Organizes generated code into a structured `generated_decoders` directory.
4.  Renames folders to follow Rust naming conventions.
5.  Updates `mod.rs` to automatically expose new decoders.

## Pre-requisites

To run this application, you need the following installed:

1.  **Rust & Cargo**: Ensure you have Rust installed. You can check with:
    ```sh
    cargo --version
    ```
2.  **Carbon CLI**: This tool wraps `carbon-cli`. Install it using:
    ```sh
    cargo install carbon-cli
    ```

## Usage

You can use the tool in two modes: using a local JSON IDL file or fetching it directly from the Solana mainnet.

### Mode 1: Generate from Local IDL

Place your JSON IDL file in the `src/idl_inputs/` directory (or anywhere else). Run the command pointing to the file:

```sh
cargo run --bin generate_decoders src/idl_inputs/<filename.json>
```

**Example:**
```sh
cargo run --bin generate_decoders src/idl_inputs/amm.json
```

### Mode 2: Generate from Program Address

If you want to fetch the IDL from the blockchain using a program ID:

```sh
cargo run --bin generate_decoders <program_address>
```

**Example:**
```sh
cargo run --bin generate_decoders whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc
```

### Output

The generated code will be placed in `generated_decoders/<decoder_name>`. The tool assumes a standard project structure where `src/bin/generate_decoders.rs` is executed from the project root.