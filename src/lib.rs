//! CallGraphGenerator - A call graph generation tool using LSP
//!
//! This library provides functionality to analyze code using Language Server Protocol (LSP)
//! and generate call graphs in DOT format for visualization with Graphviz.

pub mod analyzer;
pub mod config;
pub mod lsp;
pub mod output;

pub use analyzer::CallGraph;
pub use config::{Config, CliArgs};
pub use lsp::LspClient;
pub use output::DotGenerator;

/// Main result type for the library
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Error types for the call graph generator
#[derive(thiserror::Error, Debug)]
pub enum CallGraphError {
    #[error("LSP communication error: {0}")]
    LspError(#[from] tower_lsp::jsonrpc::Error),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Analysis error: {0}")]
    AnalysisError(String),
    
    #[error("Output generation error: {0}")]
    OutputError(String),
}
