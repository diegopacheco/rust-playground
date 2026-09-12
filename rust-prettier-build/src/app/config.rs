//! Start-up options.

use crate::board::Board;

/// How a game should be set up.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    /// Board columns.
    pub columns: i16,
    /// Board rows.
    pub rows: i16,
    /// Piece-sequence seed; [`None`] draws one from the clock.
    pub seed: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            columns: Board::STANDARD_WIDTH,
            rows: Board::STANDARD_HEIGHT,
            seed: None,
        }
    }
}

impl Config {
    /// Parses `--seed N`, `--columns N` and `--rows N` from `args`.
    ///
    /// # Errors
    ///
    /// Returns a message naming the offending argument when one is unknown, is
    /// missing its value, or does not parse.
    pub fn from_args<I>(args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = String>,
    {
        let mut config = Self::default();
        let mut args = args.into_iter();
        while let Some(flag) = args.next() {
            match flag.as_str() {
                "--seed" => config.seed = Some(parse(&flag, &next_value(&mut args, &flag)?)?),
                "--columns" => config.columns = parse(&flag, &next_value(&mut args, &flag)?)?,
                "--rows" => config.rows = parse(&flag, &next_value(&mut args, &flag)?)?,
                other => return Err(format!("unknown argument: {other}")),
            }
        }
        config.validate()?;
        Ok(config)
    }

    fn validate(self) -> Result<(), String> {
        if self.columns < 4 {
            return Err("--columns must be at least 4".to_owned());
        }
        if self.rows < 6 {
            return Err("--rows must be at least 6".to_owned());
        }
        Ok(())
    }

    /// Builds the board this configuration describes.
    #[must_use]
    pub fn board(self) -> Board {
        Board::new(self.columns, self.rows)
    }
}

fn next_value<I: Iterator<Item = String>>(args: &mut I, flag: &str) -> Result<String, String> {
    args.next().ok_or_else(|| format!("{flag} needs a value"))
}

fn parse<T: std::str::FromStr>(flag: &str, raw: &str) -> Result<T, String> {
    raw.parse()
        .map_err(|_| format!("{flag} got an invalid value: {raw}"))
}

/// The usage text shown for `--help` and for a bad argument.
pub const USAGE: &str = "\
tetris-tui [--seed N] [--columns N] [--rows N]

  --seed N     replay a specific piece sequence
  --columns N  board width  (default 10, minimum 4)
  --rows N     board height (default 20, minimum 6)";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Playfield;

    fn parse_args(args: &[&str]) -> Result<Config, String> {
        Config::from_args(args.iter().map(|s| (*s).to_owned()))
    }

    #[test]
    fn no_arguments_gives_a_standard_game() {
        let config = parse_args(&[]).unwrap();
        assert_eq!(config, Config::default());
        assert_eq!(config.board().width(), Board::STANDARD_WIDTH);
    }

    #[test]
    fn a_seed_makes_the_piece_sequence_reproducible() {
        assert_eq!(parse_args(&["--seed", "42"]).unwrap().seed, Some(42));
    }

    #[test]
    fn the_board_size_can_be_chosen() {
        let config = parse_args(&["--columns", "8", "--rows", "16"]).unwrap();
        assert_eq!((config.columns, config.rows), (8, 16));
    }

    #[test]
    fn an_unknown_flag_is_reported_rather_than_ignored() {
        let error = parse_args(&["--turbo"]).unwrap_err();
        assert!(error.contains("--turbo"), "{error}");
    }

    #[test]
    fn a_flag_without_a_value_is_reported() {
        let error = parse_args(&["--seed"]).unwrap_err();
        assert!(error.contains("needs a value"), "{error}");
    }

    #[test]
    fn a_value_that_is_not_a_number_is_reported_with_the_offending_text() {
        let error = parse_args(&["--rows", "tall"]).unwrap_err();
        assert!(error.contains("tall"), "{error}");
    }

    #[test]
    fn a_board_too_narrow_to_hold_a_piece_is_refused() {
        assert!(parse_args(&["--columns", "3"]).is_err());
    }

    #[test]
    fn a_board_too_short_to_play_on_is_refused() {
        assert!(parse_args(&["--rows", "2"]).is_err());
    }

    #[test]
    fn the_usage_text_documents_every_flag_that_is_accepted() {
        for flag in ["--seed", "--columns", "--rows"] {
            assert!(USAGE.contains(flag), "{flag} is undocumented");
        }
    }
}
