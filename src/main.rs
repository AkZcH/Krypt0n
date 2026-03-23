use std::{
    env,
    fs::File,
    io::{self, BufReader, BufWriter, Write},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};

use krypton::stream::{decrypt::decrypt_stream, encrypt::encrypt_stream};

#[derive(Parser)]
#[command(name = "krypton")]
#[command(about = "Modern password-based encryption tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        input: PathBuf,

        #[arg(long)]
        output: Option<PathBuf>,

        #[arg(long)]
        password: Option<String>,

        #[arg(long = "password-env")]
        password_env: Option<String>,
    },

    Decrypt {
        input: PathBuf,

        #[arg(long)]
        output: Option<PathBuf>,

        #[arg(long)]
        password: Option<String>,

        #[arg(long = "password-env")]
        password_env: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt {
            input,
            output,
            password,
            password_env,
        } => {
            run_encrypt(input, output, password, password_env);
        }
        Commands::Decrypt {
            input,
            output,
            password,
            password_env,
        } => {
            run_decrypt(input, output, password, password_env);
        }
    }
}

fn resolve_password(password: Option<String>, password_env: Option<String>) -> Vec<u8> {
    if let Some(password) = password {
        return password.into_bytes();
    }

    if let Some(env_var) = password_env {
        return env::var(env_var)
            .expect("environment variable not set")
            .into_bytes();
    }

    print!("Enter password: ");
    io::stdout().flush().expect("failed to flush stdout");

    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("failed to read password");

    password.trim().as_bytes().to_vec()
}

fn default_encrypt_output(input: &Path) -> PathBuf {
    let mut output = input.to_path_buf();
    output.set_extension(format!(
        "{}.kry",
        input.extension().unwrap_or_default().to_string_lossy()
    ));
    output
}

fn default_decrypt_output(input: &Path) -> PathBuf {
    let mut output = input.to_path_buf();

    if let Some(ext) = output.extension() {
        if ext == "kry" {
            output.set_extension("");
        }
    }

    output
}

fn run_encrypt(
    input: PathBuf,
    output: Option<PathBuf>,
    password: Option<String>,
    password_env: Option<String>,
) {
    let input_file = File::open(&input).expect("failed to read input file");

    let password = resolve_password(password, password_env);

    let output = output.unwrap_or_else(|| default_encrypt_output(&input));
    let output_file = File::create(&output).expect("failed to write output file");

    encrypt_stream(
        BufReader::new(input_file),
        BufWriter::new(output_file),
        &password,
        b"krypton-cli:v1",
    )
    .expect("encryption failed");

    println!("Encrypted -> {}", output.display());
}

fn run_decrypt(
    input: PathBuf,
    output: Option<PathBuf>,
    password: Option<String>,
    password_env: Option<String>,
) {
    let input_file = File::open(&input).expect("failed to read input file");

    let password = resolve_password(password, password_env);

    let output = output.unwrap_or_else(|| default_decrypt_output(&input));
    let output_file = File::create(&output).expect("failed to write output file");

    decrypt_stream(
        BufReader::new(input_file),
        BufWriter::new(output_file),
        &password,
        b"krypton-cli:v1",
    )
    .expect("decryption failed");

    println!("Decrypted -> {}", output.display());
}
