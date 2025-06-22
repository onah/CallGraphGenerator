use call_graph_generator::{CallGraph, Config, DotGenerator, LspClient, Result};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Parse command line arguments
    let args = Config::parse();

    info!("Starting CallGraphGenerator");
    info!("Configuration: {:?}", args);

    // Run the main analysis
    match run_analysis(args).await {
        Ok(_) => {
            info!("Analysis completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Analysis failed: {}", e);
            Err(e)
        }
    }
}

/// Main analysis workflow
async fn run_analysis(config: Config) -> Result<()> {
    // Step 1: Initialize LSP client
    info!("Initializing LSP client");
    let mut lsp_client = LspClient::new(&config).await?;

    // Step 2: Perform analysis
    info!("Performing call graph analysis");
    let call_graph = CallGraph::analyze(&mut lsp_client, &config).await?;

    // Step 3: Generate output
    info!("Generating output");
    let dot_generator = DotGenerator::new(&config);
    dot_generator.generate(&call_graph, &config.output_path)?;

    info!(
        "Call graph generated successfully at: {}",
        config.output_path
    );

    Ok(())
}
