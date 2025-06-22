//! Configuration management for the call graph generator

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// CLI Arguments
// ============================================================================

/// Command line arguments for the call graph generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(name = "call-graph-generator")]
#[command(about = "A call graph generator tool that leverages LSP to analyze function call relationships")]
pub struct CliArgs {
    /// Path to a single file to analyze
    #[arg(short = 'f', long = "file")]
    pub file: Option<PathBuf>,

    /// Root directory of the project to analyze
    #[arg(short = 'p', long = "project")]
    pub project: Option<PathBuf>,

    /// Path to output DOT file
    #[arg(short = 'o', long = "output")]
    pub output: Option<String>,

    /// Starting function name for analysis
    #[arg(short = 'r', long = "root")]
    pub root: Option<String>,

    /// Maximum call depth to analyze
    #[arg(short = 'd', long = "depth")]
    pub depth: Option<usize>,

    /// Package/module patterns to exclude
    #[arg(long = "exclude")]
    pub exclude: Vec<String>,

    /// LSP server command to use
    #[arg(long = "lsp-server")]
    pub lsp_server: Option<String>,

    /// Verbose logging output
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}

impl CliArgs {
    /// Validate the command line arguments
    pub fn validate(&self) -> anyhow::Result<()> {
        if self.file.is_none() && self.project.is_none() {
            anyhow::bail!("Either --file or --project must be specified");
        }
        
        if self.file.is_some() && self.project.is_some() {
            anyhow::bail!("Cannot specify both --file and --project");
        }
        
        if let Some(ref file) = self.file {
            if !file.exists() {
                anyhow::bail!("Specified file does not exist: {}", file.display());
            }
        }
        
        if let Some(ref project) = self.project {
            if !project.exists() || !project.is_dir() {
                anyhow::bail!("Specified project directory does not exist: {}", project.display());
            }
        }
        
        if let Some(depth) = self.depth {
            if depth == 0 {
                anyhow::bail!("Depth must be greater than 0");
            }
        }
        
        Ok(())
    }
}

// ============================================================================
// Configuration File
// ============================================================================

/// Configuration file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    #[serde(default)]
    pub analysis: AnalysisConfig,
    #[serde(default)]
    pub output: OutputConfig,
    #[serde(default)]
    pub lsp: LspConfig,
}

impl ConfigFile {
    /// Load configuration from file
    pub fn load() -> anyhow::Result<Self> {
        Self::load_from_path("callgraph.toml")
    }
    
    /// Load configuration from specific path
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: ConfigFile = toml::from_str(&content)?;
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save(&self) -> anyhow::Result<()> {
        self.save_to_path("callgraph.toml")
    }
    
    /// Save configuration to specific path
    pub fn save_to_path<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            analysis: AnalysisConfig::default(),
            output: OutputConfig::default(),
            lsp: LspConfig::default(),
        }
    }
}

// ============================================================================
// Configuration Structures
// ============================================================================

/// Main configuration structure combining CLI args and config file
#[derive(Debug, Clone)]
pub struct Config {
    pub file_path: Option<PathBuf>,
    pub project_path: Option<PathBuf>,
    pub output_path: String,
    pub root_function: Option<String>,
    pub max_depth: Option<usize>,
    pub exclude_patterns: Vec<String>,
    pub lsp_server_command: Option<String>,
    pub verbose: bool,
    pub analysis: AnalysisConfig,
    pub output: OutputConfig,
    pub lsp: LspConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub max_depth: Option<usize>,
    pub exclude_patterns: Vec<String>,
    pub include_external: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: String,
    pub show_types: bool,
    pub show_parameters: bool,
    pub cluster_by_module: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspConfig {
    pub server_command: Option<String>,
    pub timeout_seconds: u64,
}

// ============================================================================
// Implementations
// ============================================================================

impl Config {
    /// Parse configuration from command line arguments
    pub fn parse() -> Self {
        Self::from(CliArgs::parse())
    }
    
    /// Parse configuration from iterator
    pub fn parse_from<I, T>(itr: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        Self::from(CliArgs::parse_from(itr))
    }
}

impl From<CliArgs> for Config {
    fn from(args: CliArgs) -> Self {
        // Load config file if it exists
        let config_file = ConfigFile::load().unwrap_or_default();
        
        Self {
            file_path: args.file,
            project_path: args.project,
            output_path: args.output.unwrap_or_else(|| "callgraph.dot".to_string()),
            root_function: args.root,
            max_depth: args.depth.or(config_file.analysis.max_depth),
            exclude_patterns: if args.exclude.is_empty() {
                config_file.analysis.exclude_patterns.clone()
            } else {
                args.exclude
            },
            lsp_server_command: args.lsp_server.or(config_file.lsp.server_command.clone()),
            verbose: args.verbose,
            analysis: config_file.analysis.clone(),
            output: config_file.output.clone(),
            lsp: config_file.lsp.clone(),
        }
    }
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            max_depth: None,
            exclude_patterns: vec!["test_*".to_string(), "*_test".to_string()],
            include_external: false,
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            format: "dot".to_string(),
            show_types: true,
            show_parameters: false,
            cluster_by_module: true,
        }
    }
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            server_command: None,
            timeout_seconds: 30,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_config_file_roundtrip() {
        let config = ConfigFile::default();
        let temp_file = NamedTempFile::new().unwrap();
        
        // Save and load
        config.save_to_path(temp_file.path()).unwrap();
        let loaded_config = ConfigFile::load_from_path(temp_file.path()).unwrap();
        
        // Verify it matches
        assert_eq!(config.analysis.include_external, loaded_config.analysis.include_external);
        assert_eq!(config.output.format, loaded_config.output.format);
        assert_eq!(config.lsp.timeout_seconds, loaded_config.lsp.timeout_seconds);
    }
}
