use std::path::Path;

use clap::ArgMatches;
use log::info;
use chain_db::ChainDB;
use config::Config;

pub async fn main<P: AsRef<Path>>(config_path: P, matches: &ArgMatches<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::load_from_file(config_path)?;
    info!("config file loaded");
    let db = ChainDB::new(&config.storage.data_dir);
    info!("db opened");

    db.await_background_jobs();

    if let Some(val) = matches.value_of("height") {
        info!("original block height => {}", db.get_block_height());
        let new_height = val.parse().expect("height number");
        db.force_update_block_height(new_height)?;
        info!("force update block height => {}", new_height);
    }

    if let Some(val) = matches.value_of("fork") {
        let block_number = val.parse().expect("height number");
        db.handle_chain_fork_at(block_number, /* dry_run */ false)?;
    }

    Ok(())
}
