
mod command;
mod utils;
mod revision_files;

use crate::ui::style::Theme;
use anyhow::Result;
use crossterm::event::Event;
use ratatui::{
	layout::{Alignment, Rect},
	text::{Span, Text},
	widgets::{Block, Borders, Paragraph},
	Frame,
};


pub use revision_files::RevisionFilesComponent;
pub use command::{CommandInfo, CommandText};

#[derive(Copy, Clone)]
pub enum ScrollType {
	Up,
	Down,
	Home,
	End,
	PageUp,
	PageDown,
}

#[derive(Copy, Clone)]
pub enum HorizontalScrollType {
	Left,
	Right,
}

#[derive(Copy, Clone)]
pub enum Direction {
	Up,
	Down,
}

///
#[derive(PartialEq, Eq)]
pub enum CommandBlocking {
	Blocking,
	PassingOn,
}


pub trait DrawableComponent {
	///
	fn draw(&self, f: &mut Frame, rect: Rect) -> Result<()>;
}
/// base component trait
pub trait Component {
	///
	// fn commands(
	// 	&self,
	// 	out: &mut Vec<CommandInfo>,
	// 	force_all: bool,
	// ) -> CommandBlocking;

	///
	// fn event(&mut self, ev: &Event) -> Result<EventState>;

	///
	fn focused(&self) -> bool {
		false
	}
	/// focus/unfocus this component depending on param
	fn focus(&mut self, _focus: bool) {}
	///
	fn is_visible(&self) -> bool {
		true
	}
	///
	fn hide(&mut self) {}
	///
	fn show(&mut self) -> Result<()> {
		Ok(())
	}

	///
	fn toggle_visible(&mut self) -> Result<()> {
		if self.is_visible() {
			self.hide();
			Ok(())
		} else {
			self.show()
		}
	}
}
