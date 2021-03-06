# rescrobbled

Rescrobbled is a music scrobbler daemon written in Rust. Among other things, due to sharing a Spotify account (I know, I know), I needed a way to scrobble to [Last.fm](https://last.fm) without connecting the Spotify account to my Last.fm account. Rescrobbled detects active media players running on D-Bus using `MPRIS`, automatically updates "now playing" status, and scrobbles songs as they play. [ListenBrainz](https://listenbrainz.org) is also (optionally) supported.

## How to install and use

You can download one of the prebuilt binaries [here](https://github.com/InputUsername/rescrobbled/releases). The binary can be placed anywhere you like.

Alternatively you can install from source using `cargo install --path .` from the crate root.

There is also an [Arch Linux AUR package](https://aur.archlinux.org/packages/rescrobbled-git/) by [brycied00d](https://github.com/brycied00d). (I do not maintain this so I cannot provide support for it.)

To use rescrobbled, you'll need a Last.fm API key and secret. These can be obtained [here](https://www.last.fm/api/account/create).

### Configuration

Rescrobbled expects a configuration file at `~/.config/rescrobbled/config.toml` with the following format:
```toml
# required

lastfm-key = "Last.fm API key"
lastfm-secret = "Last.fm API secret"

# optional

listenbrainz-token = "ListenBrainz API token"
enable-notifications = false
min-play-time = 0 # in seconds
player-whitelist = [ "Player MPRIS identity" ] # if empty or ommitted, will allow all players
```
By default, track submission respects Last.fm's recommended behavior; songs should only be scrobbled if they have been playing for at least half their duration, or for 4 minutes, whichever comes first. Using `min-play-time` you can override this.

### Running rescrobbled

To make sure that rescrobbled can scrobble to Last.fm, you need to run the program in a terminal. This will prompt you for your Last.fm username and password, and authenticate with Last.fm. A long-lasting session token is then obtained, which will be used on subsequent runs instead of your username/password.

If you want to run rescrobbled as a daemon, you can put the provided [systemd unit file](https://github.com/InputUsername/rescrobbled/blob/master/rescrobbled.service) in the `~/.config/systemd/user/` directory.
Change `ExecStart` to point to the location of the binary, as necessary. Then, to enable the program to run at startup, use:
```
systemctl --user enable rescrobbled.service
```
You can run it in the current session using:
```
systemctl --user start rescrobbled.service
```

## Project resources

- [Issues](https://github.com/InputUsername/rescrobbled/issues)
- [Changelog](https://github.com/InputUsername/rescrobbled/blob/master/CHANGELOG.md)
- [Releases](https://github.com/InputUsername/rescrobbled/releases)

## License

GPL-3.0, see [`LICENSE`](https://github.com/InputUsername/rescrobbled/blob/master/LICENSE).