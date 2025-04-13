use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use std::{fs::canonicalize, path::PathBuf, rc::Rc};

use crate::{args::get_app_config_path, strings::symbol};
use super::{
	key_list::{GituiKeyEvent, KeysList},
	symbols::KeySymbols,
};

pub type SharedKeyConfig = Rc<KeyConfig>;
const KEY_LIST_FILENAME: &str = "key_bindings.ron";
const KEY_SYMBOLS_FILENAME: &str = "key_symbols.ron";

#[derive(Default, Clone)]
pub struct KeyConfig {
	pub keys: KeysList,
	symbols: KeySymbols,
}

impl KeyConfig {
	fn get_config_file() -> Result<PathBuf> {
		let app_home = get_app_config_path()?;
		let config_file = app_home.join(KEY_LIST_FILENAME);
		canonicalize(&config_file)
			.map_or_else(|_| Ok(config_file), Ok)
	}

	fn get_symbols_file() -> Result<PathBuf> {
		let app_home = get_app_config_path()?;
		let symbols_file = app_home.join(KEY_SYMBOLS_FILENAME);
		canonicalize(&symbols_file)
			.map_or_else(|_| Ok(symbols_file), Ok)
	}

	pub fn init() -> Result<Self> {
		let keys = KeysList::init(Self::get_config_file()?);
		let symbols = KeySymbols::init(Self::get_symbols_file()?);
		Ok(Self { keys, symbols })
	}

	#[expect(
		clippy::missing_const_for_fn,
		reason = "as of 1.86.0 clippy wants this to be const even though that breaks"
	)]
	fn get_key_symbol(&self, k: KeyCode) -> &str {
		match k {
			KeyCode::Enter => &self.symbols.enter,
			KeyCode::Left => &self.symbols.left,
			KeyCode::Right => &self.symbols.right,
			KeyCode::Up => &self.symbols.up,
			KeyCode::Down => &self.symbols.down,
			KeyCode::Backspace => &self.symbols.backspace,
			KeyCode::Home => &self.symbols.home,
			KeyCode::End => &self.symbols.end,
			KeyCode::PageUp => &self.symbols.page_up,
			KeyCode::PageDown => &self.symbols.page_down,
			KeyCode::Tab => &self.symbols.tab,
			KeyCode::BackTab => &self.symbols.back_tab,
			KeyCode::Delete => &self.symbols.delete,
			KeyCode::Insert => &self.symbols.insert,
			KeyCode::Esc => &self.symbols.esc,
			_ => "?",
		}
	}

	pub fn get_hint(&self, ev: GituiKeyEvent) -> String {
		match ev.code {
			KeyCode::Down
			| KeyCode::Up
			| KeyCode::Right
			| KeyCode::Left
			| KeyCode::Enter
			| KeyCode::Backspace
			| KeyCode::Home
			| KeyCode::End
			| KeyCode::PageUp
			| KeyCode::PageDown
			| KeyCode::Tab
			| KeyCode::BackTab
			| KeyCode::Delete
			| KeyCode::Insert
			| KeyCode::Esc => {
				format!(
					"{}{}",
					self.get_modifier_hint(ev.modifiers),
					self.get_key_symbol(ev.code)
				)
			}
			KeyCode::Char(' ') => String::from(symbol::SPACE),
			KeyCode::Char(c) => {
				format!(
					"{}{}",
					self.get_modifier_hint(ev.modifiers),
					c
				)
			}
			KeyCode::F(u) => {
				format!(
					"{}F{}",
					self.get_modifier_hint(ev.modifiers),
					u
				)
			}
			KeyCode::Null => {
				self.get_modifier_hint(ev.modifiers).into()
			}
			_ => String::new(),
		}
	}

	#[expect(
		clippy::missing_const_for_fn,
		reason = "as of 1.86.0 clippy wants this to be const even though that breaks"
	)]
	fn get_modifier_hint(&self, modifier: KeyModifiers) -> &str {
		match modifier {
			KeyModifiers::CONTROL => &self.symbols.control,
			KeyModifiers::SHIFT => &self.symbols.shift,
			KeyModifiers::ALT => &self.symbols.alt,
			_ => "",
		}
	}
}

