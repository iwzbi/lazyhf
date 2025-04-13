
impl GituiKeyEvent {
	pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
		Self { code, modifiers }
	}
}

#[derive(Debug, Clone, Patch)]
#[patch(attribute(derive(Deserialize, Debug)))]
pub struct KeysList {
	pub tab_status: GituiKeyEvent,
	pub tab_log: GituiKeyEvent,
	pub tab_files: GituiKeyEvent,
}
