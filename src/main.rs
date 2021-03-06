// Rescrobbled is a simple music scrobbler daemon.
//
// Copyright (C) 2019 Koen Bolhuis
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::process;

use rustfm_scrobble::Scrobbler;

mod auth;
mod config;
mod mainloop;

fn main() {
    let config = match config::load_config() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error while loading config: {}", err);
            process::exit(1);
        }
    };

    let mut scrobbler = Scrobbler::new(&config.lastfm_key, &config.lastfm_secret);

    match auth::authenticate(&mut scrobbler) {
        Ok(_) => println!("Authenticated with Last.fm successfully!"),
        Err(err) => {
            eprintln!("Failed to authenticate with Last.fm: {}", err);
            process::exit(1);
        }
    }

    mainloop::run(&config, &scrobbler);
}
