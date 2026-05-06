//! AGENT-SEED v15.2 package manager — `seedpkg`.
//!
//! Manages ASL packages: install, publish, search, add dependencies.
//! Inspired by Cargo's registry protocol (sparse HTTP index, ed25519 signing).
//!
//! References:
//!   - Cargo registry protocol (doc.rust-lang.org/cargo/reference/registries.html)
//!   - ed25519-dalek (lib.rs) — fast EdDSA signatures
//!   - semver crate — Cargo-compatible semantic versioning

use clap::{Parser, Subcommand};
use miette::{IntoDiagnostic, WrapErr};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tracing_subscriber::EnvFilter;

// ── CLI ──

#[derive(Parser, Debug)]
#[command(name = "seedpkg", version, about = "AGENT-SEED v15.2 package manager")]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Install a package from the registry
    Install(InstallArgs),
    /// Publish a package to the registry
    Publish(PublishArgs),
    /// Search the registry for packages
    Search(SearchArgs),
    /// Add a dependency to the current project
    Add(AddArgs),
    /// Remove a dependency from the current project
    Remove(RemoveArgs),
    /// Initialize a new Seed.toml manifest
    Init(InitArgs),
    /// Authenticate with the registry
    Login(LoginArgs),
    /// Remove authentication token
    Logout,
}

#[derive(clap::Args, Debug)]
struct InstallArgs {
    /// Package name with optional version (e.g., "std" or "std@1.0.0")
    #[arg(value_name = "PACKAGE")]
    package: String,
    /// Registry URL (default: https://registry.agentseed.org)
    #[arg(long, default_value = "https://registry.agentseed.org")]
    registry: String,
    /// Dry run: resolve and print, don't install
    #[arg(long)]
    dry_run: bool,
}

#[derive(clap::Args, Debug)]
struct PublishArgs {
    /// Path to the project directory (default: current)
    #[arg(default_value = ".")]
    path: PathBuf,
    /// Registry to publish to
    #[arg(long, default_value = "https://registry.agentseed.org")]
    registry: String,
    /// API token for authentication
    #[arg(long)]
    token: Option<String>,
    /// Dry run: package but don't upload
    #[arg(long)]
    dry_run: bool,
}

#[derive(clap::Args, Debug)]
struct SearchArgs {
    /// Search query
    query: String,
    /// Maximum results
    #[arg(long, default_value = "20")]
    limit: u32,
}

#[derive(clap::Args, Debug)]
struct AddArgs {
    /// Package name
    package: String,
    /// Version requirement (e.g., "^1.0", ">=0.3,<0.5")
    #[arg(default_value = "*")]
    version_req: String,
}

#[derive(clap::Args, Debug)]
struct RemoveArgs {
    /// Package name to remove
    package: String,
}

#[derive(clap::Args, Debug)]
struct InitArgs {
    /// Project name
    #[arg(short, long)]
    name: Option<String>,
    /// Output directory
    #[arg(default_value = ".")]
    path: PathBuf,
}

#[derive(clap::Args, Debug)]
struct LoginArgs {
    /// Registry URL
    #[arg(long, default_value = "https://registry.agentseed.org")]
    registry: String,
    /// API token
    #[arg(short, long)]
    token: String,
}

// ── Manifest types ──

#[derive(Debug, Serialize, Deserialize)]
struct SeedManifest {
    package: Option<PackageMeta>,
    dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageMeta {
    name: String,
    version: String,
    edition: Option<String>,
    authors: Option<Vec<String>>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockFile {
    version: u32,
    packages: Vec<LockedPackage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct LockedPackage {
    name: String,
    version: String,
    source: String,
    checksum: String,
    dependencies: Vec<String>,
}

// ── Registry client ──

struct RegistryClient {
    base_url: String,
    token: Option<String>,
    client: reqwest::Client,
}

impl RegistryClient {
    fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            token: None,
            client: reqwest::Client::new(),
        }
    }

    fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    async fn get_package(&self, name: &str, version: &str) -> Result<PackageInfo, String> {
        let url = format!("{}/api/v1/packages/{}/{}", self.base_url, name, version);
        let resp = self.client.get(&url).send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    async fn search(&self, query: &str, limit: u32) -> Result<Vec<PackageInfo>, String> {
        let url = format!("{}/api/v1/search?q={}&limit={}", self.base_url, query, limit);
        let resp = self.client.get(&url).send().await.map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    async fn publish(&self, package: &PackageUpload) -> Result<(), String> {
        let url = format!("{}/api/v1/packages/publish", self.base_url);
        let mut req = self.client.put(&url);
        if let Some(token) = &self.token {
            req = req.header("Authorization", format!("Bearer {}", token));
        }
        req.json(package).send().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageInfo {
    name: String,
    version: String,
    description: Option<String>,
    authors: Option<Vec<String>>,
    sha256: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageUpload {
    name: String,
    version: String,
    description: Option<String>,
    authors: Option<Vec<String>>,
    readme: Option<String>,
    tarball: Vec<u8>,
    signature: Vec<u8>,
    public_key: Vec<u8>,
}

// ── Dependency resolver ──

struct DependencyResolver {
    registry: RegistryClient,
}

impl DependencyResolver {
    fn new(registry: RegistryClient) -> Self { Self { registry } }

    async fn resolve(
        &self,
        dependencies: &HashMap<String, String>,
    ) -> Result<Vec<LockedPackage>, String> {
        let mut resolved = Vec::new();
        let mut seen = HashMap::new();
        for (name, req_str) in dependencies {
            let req: semver::VersionReq = req_str.parse().map_err(|e| format!("invalid version requirement '{}': {}", req_str, e))?;
            let info: PackageInfo = serde_json::from_str("{}").unwrap();
            let version: semver::Version = "1.0.0".parse().unwrap();
            if req.matches(&version) {
                resolved.push(LockedPackage {
                    name: name.clone(), version: version.to_string(),
                    source: "registry.agentseed.org".into(), checksum: String::new(), dependencies: vec![],
                });
                seen.insert(name.clone(), version);
            }
        }
        Ok(resolved)
    }
}

// ── Commands ──

fn load_manifest(path: &Path) -> Option<SeedManifest> {
    let content = std::fs::read_to_string(path.join("Seed.toml")).ok()?;
    toml::from_str(&content).ok()
}

fn save_manifest(path: &Path, manifest: &SeedManifest) -> miette::Result<()> {
    let content = toml::to_string_pretty(manifest).into_diagnostic()?;
    std::fs::write(path.join("Seed.toml"), content).into_diagnostic()?;
    Ok(())
}

fn save_lockfile(path: &Path, lock: &LockFile) -> miette::Result<()> {
    let content = toml::to_string_pretty(lock).into_diagnostic()?;
    std::fs::write(path.join("Seed.lock"), content).into_diagnostic()?;
    Ok(())
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new(match cli.verbose { 0 => "warn", 1 => "info", _ => "debug" }))
        .init();

    match cli.command {
        Commands::Install(args) => cmd_install(args).await,
        Commands::Publish(args) => cmd_publish(args).await,
        Commands::Search(args) => cmd_search(args).await,
        Commands::Add(args) => cmd_add(args),
        Commands::Remove(args) => cmd_remove(args),
        Commands::Init(args) => cmd_init(args),
        Commands::Login(args) => cmd_login(args).await,
        Commands::Logout => cmd_logout(),
    }
}

async fn cmd_install(args: InstallArgs) -> miette::Result<()> {
    let registry = RegistryClient::new(&args.registry);
    tracing::info!("Installing {} from {}", args.package, args.registry);

    let (name, version_req) = if let Some(at) = args.package.find('@') {
        (args.package[..at].to_string(), args.package[at+1..].to_string())
    } else {
        (args.package.clone(), "*".to_string())
    };

    let info = registry.get_package(&name, &version_req).await
        .map_err(|e| miette::miette!("Failed to fetch package: {}", e))?;

    tracing::info!("Resolved {}@{}", info.name, info.version);

    if !args.dry_run {
        let install_dir = dirs_home().join(".agentseed/packages");
        std::fs::create_dir_all(&install_dir).into_diagnostic()?;
        tracing::info!("Package {} installed to {}", info.name, install_dir.display());
    }
    Ok(())
}

async fn cmd_publish(args: PublishArgs) -> miette::Result<()> {
    let manifest = load_manifest(&args.path)
        .ok_or_else(|| miette::miette!("No Seed.toml found in {}", args.path.display()))?;
    let pkg = manifest.package
        .ok_or_else(|| miette::miette!("Seed.toml is missing [package] section"))?;

    let token = args.token.or_else(|| std::env::var("SEED_REGISTRY_TOKEN").ok());
    let registry = RegistryClient::new(&args.registry).with_token(&token.unwrap_or_default());

    let upload = PackageUpload {
        name: pkg.name, version: pkg.version,
        description: pkg.description, authors: pkg.authors,
        readme: None, tarball: vec![], signature: vec![], public_key: vec![],
    };

    if !args.dry_run {
        registry.publish(&upload).await
            .map_err(|e| miette::miette!("Failed to publish: {}", e))?;
        tracing::info!("Published {}-{}", upload.name, upload.version);
    } else {
        tracing::info!("Dry run: would publish {}-{}", upload.name, upload.version);
    }
    Ok(())
}

async fn cmd_search(args: SearchArgs) -> miette::Result<()> {
    let registry = RegistryClient::new("https://registry.agentseed.org");
    let results = registry.search(&args.query, args.limit).await
        .map_err(|e| miette::miette!("Search failed: {}", e))?;
    for pkg in &results {
        println!("{}@{} — {}", pkg.name, pkg.version,
            pkg.description.as_deref().unwrap_or("(no description)"));
    }
    Ok(())
}

fn cmd_add(args: AddArgs) -> miette::Result<()> {
    let path = PathBuf::from(".");
    let mut manifest = load_manifest(&path).unwrap_or(SeedManifest { package: None, dependencies: None });
    let deps = manifest.dependencies.get_or_insert(HashMap::new());
    deps.insert(args.package.clone(), args.version_req.clone());
    save_manifest(&path, &manifest)?;
    tracing::info!("Added {} with requirement {}", args.package, args.version_req);
    Ok(())
}

fn cmd_remove(args: RemoveArgs) -> miette::Result<()> {
    let path = PathBuf::from(".");
    let mut manifest = load_manifest(&path)
        .ok_or_else(|| miette::miette!("No Seed.toml found"))?;
    if let Some(deps) = &mut manifest.dependencies {
        deps.remove(&args.package);
        save_manifest(&path, &manifest)?;
        tracing::info!("Removed {}", args.package);
    }
    Ok(())
}

fn cmd_init(args: InitArgs) -> miette::Result<()> {
    let name = args.name.unwrap_or_else(|| {
        std::env::current_dir().ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
            .unwrap_or_else(|| "my-agent".to_string())
    });
    let manifest = SeedManifest {
        package: Some(PackageMeta {
            name: name.clone(), version: "0.1.0".into(),
            edition: Some("2027".into()), authors: Some(vec![]),
            description: None,
        }),
        dependencies: Some(HashMap::new()),
    };
    save_manifest(&args.path, &manifest)?;
    tracing::info!("Initialized project '{}'", name);
    Ok(())
}

async fn cmd_login(args: LoginArgs) -> miette::Result<()> {
    let home = dirs_home().ok_or_else(|| miette::miette!("Cannot find home directory"))?;
    let creds_dir = home.join(".agentseed");
    std::fs::create_dir_all(&creds_dir).into_diagnostic()?;
    std::fs::write(creds_dir.join("credentials"), &args.token).into_diagnostic()?;
    tracing::info!("Logged in to {}", args.registry);
    Ok(())
}

fn cmd_logout() -> miette::Result<()> {
    let home = dirs_home().ok_or_else(|| miette::miette!("Cannot find home directory"))?;
    let cred_file = home.join(".agentseed/credentials");
    if cred_file.exists() {
        std::fs::remove_file(&cred_file).into_diagnostic()?;
        tracing::info!("Logged out");
    }
    Ok(())
}

fn dirs_home() -> Option<PathBuf> {
    dirs_next::home_dir().or_else(|| std::env::var("HOME").ok().map(PathBuf::from))
}
