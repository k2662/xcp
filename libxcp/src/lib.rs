/*
 * Copyright © 2024, Steve Smith <tarkasteve@gmail.com>
 *
 * This program is free software: you can redistribute it and/or
 * modify it under the terms of the GNU General Public License version
 * 3 as published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! `libxcp` is a high-level file-copy engine. It has a support for
//! multi-threading, fine-grained progress feedback, pluggable
//! drivers, and `.gitignore` filters. `libxcp` is the core
//! functionality of the [xcp] command-line utility.
//!
//! # Usage example
//!
//! ```
//! # use std::path::PathBuf;
//! # use std::sync::Arc;
//! # use tempfile::TempDir;
//! #
//! use libxcp::errors::Result;
//! use libxcp::config::Config;
//! use libxcp::operations::{ChannelUpdater, StatusUpdater, StatusUpdate};
//! use libxcp::drivers::{Drivers, load_driver};
//!
//! let sources = vec![PathBuf::from("src")];
//! let dest = TempDir::new().unwrap();
//!
//! let config = Arc::new(Config::default());
//! let updater = ChannelUpdater::new(&config);
//! let stat_rx = updater.rx_channel();
//! let stats: Arc<dyn StatusUpdater> = Arc::new(updater);
//!
//! let driver = load_driver(Drivers::ParFile, &config).unwrap();
//!
//! driver.copy_all(sources, dest.path(), stats).unwrap();
//!
//! // Gather the results as we go; our end of the channel has been
//! // moved to the driver call and will end when drained.
//! for stat in stat_rx {
//!     match stat {
//!         StatusUpdate::Copied(v) => {
//!             println!("Copied {} bytes", v);
//!         },
//!         StatusUpdate::Size(v) => {
//!             println!("Size update: {}", v);
//!         },
//!         StatusUpdate::Error(e) => {
//!             panic!("Error during copy: {}", e);
//!         }
//!     }
//! }
//! println!("Copy complete");
//! ```
//! [xcp]: https://crates.io/crates/xcp/

pub mod config;
pub mod drivers;
pub mod errors;
pub mod operations;
pub mod paths;

#[cfg(test)]
#[allow(unused)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Arc;

    use tempfile::TempDir;

    use crate::errors::Result;
    use crate::config::Config;
    use crate::operations::StatusUpdate;
    use crate::{operations::{ChannelUpdater, StatusUpdater}, drivers::{Drivers, load_driver}};


    #[test]
    fn example_usage_test() -> Result<()> {
        let sources = vec![PathBuf::from("src")];
        let dest = TempDir::new()?;

        let config = Arc::new(Config::default());
        let updater = ChannelUpdater::new(&config);
        let stat_rx = updater.rx_channel();
        let stats: Arc<dyn StatusUpdater> = Arc::new(updater);

        let driver = load_driver(Drivers::ParFile, &config)?;

        driver.copy_all(sources, dest.path(), stats)?;

        // Gather the results as we go; our end of the channel has been
        // moved to the driver call and will end when drained.
        for stat in stat_rx {
            match stat {
                StatusUpdate::Copied(v) => {
                    println!("Copied {} bytes", v);
                },
                StatusUpdate::Size(v) => {
                    println!("Size update: {}", v);
                },
                StatusUpdate::Error(e) => {
                    println!("Error during copy: {}", e);
                    return Err(e.into());
                }
            }
        }

        println!("Copy complete");

        Ok(())
    }
}
