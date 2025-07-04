use std::path::PathBuf;

#[derive(clap::Parser, Clone, Debug)]
pub enum CustomCommand {
    #[command()]
    Starship(StarshipOptions),
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum StarshipCommands {
    /// Print the configured Prompt
    Prompt {
        /// Path to the jj-starship config file
        #[arg(long, env = "STARSHIP_JJ_CONFIG")]
        starship_config: Option<PathBuf>,
    },

    /// Interact with the configuration
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum ConfigCommands {
    /// Print the path to the config file
    Path,
    /// Print the default Config
    Default,
}

#[derive(clap::Args, Clone, Debug)]
pub struct StarshipOptions {
    #[command(subcommand)]
    pub command: StarshipCommands,
}
