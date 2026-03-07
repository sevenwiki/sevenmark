use serde_json::{Value, json};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use std::thread::sleep;
use std::time::Duration;

const CRATES: &[&str] = &[
    "sevenmark_ast",
    "sevenmark_parser",
    "sevenmark_utils",
    "sevenmark_html",
    "sevenmark_formatter",
];

const DEFAULT_WASM_CRATE_NAME: &str = "sevenmark_wasm";
const DEFAULT_WASM_NPM_OUT_DIR: &str = "pkg-npm";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("publish") => publish(false),
        Some("publish-dry") => publish(true),
        Some("wasm-npm-pack") => {
            if args[2..]
                .iter()
                .any(|argument| argument == "--help" || argument == "-h")
            {
                println!("{}", wasm_npm_usage("wasm-npm-pack"));
                return;
            }
            let options = WasmNpmOptions::from_args(&args[2..]).unwrap_or_else(|message| {
                eprintln!("{message}");
                exit(1);
            });
            wasm_npm_pack(&options).unwrap_or_else(|error| {
                eprintln!("Failed to build wasm npm package: {error}");
                exit(1);
            });
        }
        Some("wasm-npm-publish") => {
            if args[2..]
                .iter()
                .any(|argument| argument == "--help" || argument == "-h")
            {
                println!("{} [--dry-run]", wasm_npm_usage("wasm-npm-publish"));
                return;
            }
            let options = WasmNpmPublishOptions::from_args(&args[2..]).unwrap_or_else(|message| {
                eprintln!("{message}");
                exit(1);
            });
            wasm_npm_publish(&options).unwrap_or_else(|error| {
                eprintln!("Failed to publish wasm npm package: {error}");
                exit(1);
            });
        }
        _ => {
            eprintln!("Usage: cargo xtask <command>");
            eprintln!();
            eprintln!("Commands:");
            eprintln!("  publish      Publish crates to crates.io");
            eprintln!("  publish-dry  Dry run publish");
            eprintln!("  wasm-npm-pack     Build the bundler-target npm package for a wasm crate");
            eprintln!("  wasm-npm-publish  Build and publish the bundler-target npm package");
            exit(1);
        }
    }
}

fn publish(dry_run: bool) {
    println!("Publishing SevenMark crates...\n");

    for (i, crate_name) in CRATES.iter().enumerate() {
        println!("Publishing {}...", crate_name);

        let mut cmd = Command::new("cargo");
        cmd.arg("publish").arg("-p").arg(crate_name);

        if dry_run {
            cmd.arg("--dry-run");
        }

        let status = cmd.status().expect("Failed to execute cargo publish");

        if !status.success() {
            eprintln!("Failed to publish {}", crate_name);
            exit(1);
        }

        println!("{} published successfully\n", crate_name);

        // Wait for crates.io index sync (except for last crate)
        if !dry_run && i < CRATES.len() - 1 {
            println!("Waiting 15s for crates.io index sync...");
            sleep(Duration::from_secs(15));
        }
    }

    println!("All crates published!");
}

#[derive(Debug, Clone)]
struct WasmNpmOptions {
    crate_name: String,
    scope: Option<String>,
    package_name: String,
    out_dir: PathBuf,
    release: bool,
}

#[derive(Debug, Clone)]
struct WasmNpmPublishOptions {
    build: WasmNpmOptions,
    dry_run: bool,
}

impl WasmNpmOptions {
    fn from_args(args: &[String]) -> Result<Self, String> {
        let mut scope = None;
        let mut crate_name = None;
        let mut package_name = None;
        let mut out_dir = None;
        let mut release = true;

        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "--crate" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| "Missing value for --crate".to_string())?;
                    crate_name = Some(value.clone());
                }
                "--scope" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| "Missing value for --scope".to_string())?;
                    scope = Some(normalize_scope(value));
                }
                "--package-name" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| "Missing value for --package-name".to_string())?;
                    package_name = Some(value.clone());
                }
                "--out-dir" => {
                    index += 1;
                    let value = args
                        .get(index)
                        .ok_or_else(|| "Missing value for --out-dir".to_string())?;
                    out_dir = Some(PathBuf::from(value));
                }
                "--dev" => release = false,
                "--release" => release = true,
                unknown => {
                    return Err(format!(
                        "Unknown argument: {unknown}\n\n{}",
                        wasm_npm_usage("wasm-npm-pack")
                    ));
                }
            }
            index += 1;
        }

        let crate_name = crate_name
            .or_else(|| env_var("SEVENMARK_WASM_CRATE"))
            .unwrap_or_else(|| DEFAULT_WASM_CRATE_NAME.to_string());
        let scope = scope.or_else(|| {
            env_var("SEVENMARK_NPM_SCOPE")
                .as_deref()
                .map(normalize_scope)
        });
        let package_name = package_name
            .or_else(|| env_var("SEVENMARK_NPM_PACKAGE_NAME"))
            .unwrap_or_else(|| default_npm_package_name(&crate_name));
        let out_dir = out_dir
            .or_else(|| env_var("SEVENMARK_NPM_OUT_DIR").map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from(DEFAULT_WASM_NPM_OUT_DIR));

        Ok(Self {
            crate_name,
            scope,
            package_name,
            out_dir,
            release,
        })
    }

    fn npm_package_name(&self) -> String {
        match &self.scope {
            Some(scope) if !scope.is_empty() => format!("@{scope}/{}", self.package_name),
            _ => self.package_name.clone(),
        }
    }
}

impl WasmNpmPublishOptions {
    fn from_args(args: &[String]) -> Result<Self, String> {
        let mut dry_run = false;
        let mut build_args = Vec::new();

        for argument in args {
            if argument == "--dry-run" {
                dry_run = true;
            } else {
                build_args.push(argument.clone());
            }
        }

        Ok(Self {
            build: WasmNpmOptions::from_args(&build_args)?,
            dry_run,
        })
    }
}

fn wasm_npm_usage(command: &str) -> String {
    format!(
        "Usage: cargo xtask {command} [--crate <crate-name>] [--scope <scope>] [--package-name <name>] [--out-dir <dir>] [--dev|--release]"
    )
}

fn env_var(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn normalize_scope(scope: &str) -> String {
    scope.trim().trim_start_matches('@').to_string()
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask directory should have a parent")
        .to_path_buf()
}

fn wasm_crate_dir(crate_name: &str) -> PathBuf {
    workspace_root().join("crates").join(crate_name)
}

fn resolve_output_dir(crate_dir: &Path, out_dir: &Path) -> PathBuf {
    if out_dir.is_absolute() {
        out_dir.to_path_buf()
    } else {
        crate_dir.join(out_dir)
    }
}

fn wasm_npm_pack(options: &WasmNpmOptions) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let workspace_root = workspace_root();
    let crate_dir = wasm_crate_dir(&options.crate_name);
    let output_dir = resolve_output_dir(&crate_dir, &options.out_dir);

    if !crate_dir.exists() {
        return Err(format!("Unknown wasm crate: {}", options.crate_name).into());
    }

    if output_dir.exists() {
        fs::remove_dir_all(&output_dir)?;
    }

    println!(
        "Building bundler package {} from {} into {}",
        options.npm_package_name(),
        options.crate_name,
        output_dir.display()
    );

    let mut build = Command::new("wasm-pack");
    build
        .current_dir(&crate_dir)
        .arg("build")
        .arg("--target")
        .arg("bundler")
        .arg("--mode")
        .arg("no-install")
        .arg("--out-dir")
        .arg(output_dir.as_os_str());

    if options.release {
        build.arg("--release");
    } else {
        build.arg("--dev");
    }

    run_command(build, "wasm-pack build")?;

    rewrite_package_json(&output_dir, options)?;
    write_package_readme(&crate_dir, &output_dir, &options.npm_package_name())?;
    fs::copy(workspace_root.join("LICENSE"), output_dir.join("LICENSE"))?;

    println!("npm package ready at {}", output_dir.display());
    Ok(output_dir)
}

fn wasm_npm_publish(options: &WasmNpmPublishOptions) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = wasm_npm_pack(&options.build)?;

    let mut publish = npm_command();
    publish
        .current_dir(&output_dir)
        .arg("publish")
        .arg("--access")
        .arg("public");

    if options.dry_run {
        publish.arg("--dry-run");
    }

    run_command(publish, "npm publish")?;
    Ok(())
}

fn rewrite_package_json(
    output_dir: &Path,
    options: &WasmNpmOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let package_json_path = output_dir.join("package.json");
    let package_json = fs::read_to_string(&package_json_path)?;
    let mut value: Value = serde_json::from_str(&package_json)?;
    let object = value
        .as_object_mut()
        .ok_or("Generated package.json was not a JSON object")?;
    let main = object
        .get("main")
        .and_then(Value::as_str)
        .ok_or("Generated package.json is missing a string 'main' field")?
        .to_string();
    let types = object
        .get("types")
        .and_then(Value::as_str)
        .ok_or("Generated package.json is missing a string 'types' field")?
        .to_string();

    object.insert(
        "name".to_string(),
        Value::String(options.npm_package_name()),
    );
    object.insert(
        "repository".to_string(),
        json!({
            "type": "git",
            "url": "git+https://github.com/sevenwiki/sevenmark.git"
        }),
    );
    object.insert(
        "homepage".to_string(),
        Value::String("https://github.com/sevenwiki/sevenmark".to_string()),
    );
    object.insert(
        "bugs".to_string(),
        json!({
            "url": "https://github.com/sevenwiki/sevenmark/issues"
        }),
    );
    object.insert(
        "keywords".to_string(),
        json!(default_keywords(&options.crate_name)),
    );
    object.insert(
        "publishConfig".to_string(),
        json!({
            "access": "public"
        }),
    );
    object.insert(
        "exports".to_string(),
        json!({
            ".": {
                "types": format!("./{types}"),
                "default": format!("./{main}")
            }
        }),
    );
    object.insert("module".to_string(), Value::String(main));

    fs::write(package_json_path, serde_json::to_string_pretty(&value)?)?;
    Ok(())
}

fn write_package_readme(
    crate_dir: &Path,
    output_dir: &Path,
    package_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let template = fs::read_to_string(crate_dir.join("README.npm.md"))?;
    let rendered = template.replace("{{PACKAGE_NAME}}", package_name);
    fs::write(output_dir.join("README.md"), rendered)?;
    Ok(())
}

fn run_command(mut command: Command, description: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = command.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{description} failed with status {status}").into())
    }
}

fn npm_command() -> Command {
    if cfg!(windows) {
        Command::new("npm.cmd")
    } else {
        Command::new("npm")
    }
}

fn default_npm_package_name(crate_name: &str) -> String {
    match crate_name {
        "sevenmark_wasm" => "sevenmark".to_string(),
        "sevenmark_wasm_lsp" => "sevenmark-lsp".to_string(),
        other => other.replace('_', "-"),
    }
}

fn default_keywords(crate_name: &str) -> Vec<&'static str> {
    match crate_name {
        "sevenmark_wasm_lsp" => vec!["sevenmark", "wiki", "lsp", "wasm", "editor", "worker"],
        _ => vec!["sevenmark", "wiki", "parser", "wasm", "codemirror"],
    }
}
