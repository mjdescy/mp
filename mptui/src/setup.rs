//! Setup wizard.
//!
//! Shown instead of the main UI when no config file exists, and on demand
//! from the editor (F2). Step 1 collects
//! the Micropub endpoint and auth token, step 2 the default behavior. Leaving
//! the wizard always goes through a confirmation dialog, so a stray Escape
//! never exits.

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use crate::config;
use crossterm::event::{
    self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind,
};
use ratatui::style::Stylize;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph, Wrap},
};
use tui_textarea::TextArea;

const DEFAULT_API_URL: &str = "https://micro.blog/micropub";

/// Where the wizard was launched from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SetupMode {
    /// No config file exists yet; leaving exits the app.
    FirstRun,
    /// Launched from the editor; leaving returns to it.
    Edit,
}

/// How the wizard ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SetupOutcome {
    /// The config file was written; the main UI can load.
    Completed,
    /// The user confirmed leaving without creating a config file.
    Exited,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    Service,
    Behavior,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Focus {
    ApiUrl,
    AuthToken,
    ShowToken,
    ExtractTitle,
    Back,
    Exit,
    Next,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExitDialogFocus {
    Stay,
    Leave,
}

#[derive(Default)]
struct Areas {
    api_url: Rect,
    auth_token: Rect,
    show_token: Rect,
    extract_yes: Rect,
    extract_no: Rect,
    back: Rect,
    exit: Rect,
    next: Rect,
    dialog_stay: Rect,
    dialog_leave: Rect,
}

struct Setup {
    path: PathBuf,
    mode: SetupMode,
    step: Step,
    focus: Focus,
    api_url: TextArea<'static>,
    auth_token: TextArea<'static>,
    show_token: bool,
    extract_title: bool,
    error: Option<String>,
    exit_dialog: Option<ExitDialogFocus>,
    outcome: Option<SetupOutcome>,
    areas: Areas,
}

/// Run the wizard until the config file is written or the user exits.
pub(crate) fn run(
    terminal: &mut DefaultTerminal,
    config_path: &Path,
    mode: SetupMode,
) -> io::Result<SetupOutcome> {
    crossterm::execute!(io::stdout(), event::EnableMouseCapture)?;
    let result = run_loop(terminal, Setup::new(config_path.to_path_buf(), mode));
    let disable = crossterm::execute!(io::stdout(), event::DisableMouseCapture);
    result.and_then(|outcome| disable.map(|()| outcome))
}

fn run_loop(terminal: &mut DefaultTerminal, mut setup: Setup) -> io::Result<SetupOutcome> {
    loop {
        terminal.draw(|frame| setup.draw(frame))?;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => setup.handle_key(key),
            Event::Mouse(mouse) => setup.handle_mouse(mouse),
            Event::Paste(text) => setup.paste(&text),
            _ => {}
        }
        if let Some(outcome) = setup.outcome {
            return Ok(outcome);
        }
    }
}

fn single_line_input() -> TextArea<'static> {
    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(Style::default());
    textarea
}

/// Render the config file contents for the given answers. `quiet` is always
/// written as false.
fn render_config(api_url: &str, auth_token: &str, extract_title: bool) -> String {
    let quote = |value: &str| toml::Value::String(value.to_string()).to_string();
    format!(
        "[service]\napi_url = {}\nauth_token = {}\n\n[default_behavior]\nquiet = false\nextract_title = {}\n",
        quote(api_url),
        quote(auth_token),
        extract_title,
    )
}

/// Write the config file, creating parent directories. The file holds a
/// secret, so it is created readable by the owner only.
fn write_config(path: &Path, contents: &str) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    #[cfg(unix)]
    {
        use std::{io::Write, os::unix::fs::OpenOptionsExt};
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)?;
        file.write_all(contents.as_bytes())
    }
    #[cfg(not(unix))]
    {
        fs::write(path, contents)
    }
}

impl Setup {
    fn new(path: PathBuf, mode: SetupMode) -> Self {
        let existing = match mode {
            SetupMode::Edit => config::load_micropub_settings_from(&path),
            SetupMode::FirstRun => config::MicropubSettings::default(),
        };
        let mut api_url = single_line_input();
        api_url.insert_str(
            existing
                .service_api_url
                .as_deref()
                .unwrap_or(DEFAULT_API_URL),
        );
        let mut auth_token = single_line_input();
        auth_token.set_mask_char('•');
        auth_token.insert_str(existing.service_auth_token.as_deref().unwrap_or(""));
        Self {
            path,
            mode,
            step: Step::Service,
            focus: Focus::ApiUrl,
            api_url,
            auth_token,
            show_token: false,
            extract_title: match mode {
                SetupMode::Edit => existing.extract_title,
                SetupMode::FirstRun => true,
            },
            error: None,
            exit_dialog: None,
            outcome: None,
            areas: Areas::default(),
        }
    }

    fn api_url_text(&self) -> String {
        self.api_url.lines().join("").trim().to_string()
    }

    fn auth_token_text(&self) -> String {
        self.auth_token.lines().join("").trim().to_string()
    }

    /// Whether every field on the current step has a value.
    fn fields_filled(&self) -> bool {
        match self.step {
            Step::Service => !self.api_url_text().is_empty() && !self.auth_token_text().is_empty(),
            Step::Behavior => true,
        }
    }

    /// Enter in a form field jumps to the Next/Finish button, but only once
    /// every field on the step is filled in.
    fn enter_from_field(&mut self) {
        if self.fields_filled() {
            self.focus = Focus::Next;
        }
    }

    fn focus_order(&self) -> &'static [Focus] {
        match self.step {
            Step::Service => &[
                Focus::ApiUrl,
                Focus::AuthToken,
                Focus::ShowToken,
                Focus::Exit,
                Focus::Next,
            ],
            Step::Behavior => &[Focus::ExtractTitle, Focus::Back, Focus::Exit, Focus::Next],
        }
    }

    fn move_focus(&mut self, delta: isize) {
        let order = self.focus_order();
        let current = order.iter().position(|f| *f == self.focus).unwrap_or(0);
        let next = (current as isize + delta).rem_euclid(order.len() as isize) as usize;
        self.focus = order[next];
    }

    fn is_button(focus: Focus) -> bool {
        matches!(focus, Focus::Back | Focus::Exit | Focus::Next)
    }

    // ---- actions -------------------------------------------------------

    fn go_next(&mut self) {
        match self.step {
            Step::Service => {
                let url = self.api_url_text();
                if !(url.starts_with("https://") || url.starts_with("http://")) {
                    self.error = Some("API URL must start with http:// or https://".into());
                    self.focus = Focus::ApiUrl;
                } else if self.auth_token_text().is_empty() {
                    self.error = Some("Auth token is required.".into());
                    self.focus = Focus::AuthToken;
                } else {
                    self.error = None;
                    self.step = Step::Behavior;
                    self.focus = Focus::ExtractTitle;
                }
            }
            Step::Behavior => self.finish(),
        }
    }

    fn go_back(&mut self) {
        if self.step == Step::Behavior {
            self.error = None;
            self.step = Step::Service;
            self.focus = Focus::ApiUrl;
        }
    }

    fn finish(&mut self) {
        let contents = render_config(
            &self.api_url_text(),
            &self.auth_token_text(),
            self.extract_title,
        );
        match write_config(&self.path, &contents) {
            Ok(()) => self.outcome = Some(SetupOutcome::Completed),
            Err(e) => {
                self.error = Some(format!("Could not write {}: {e}", self.path.display()));
            }
        }
    }

    fn activate(&mut self, focus: Focus) {
        self.focus = focus;
        match focus {
            Focus::Exit => self.exit_dialog = Some(ExitDialogFocus::Stay),
            Focus::Back => self.go_back(),
            Focus::Next => self.go_next(),
            _ => {}
        }
    }

    // ---- input ---------------------------------------------------------

    fn paste(&mut self, text: &str) {
        if self.exit_dialog.is_some() {
            return;
        }
        let line = text.lines().next().unwrap_or("");
        match self.focus {
            Focus::ApiUrl => _ = self.api_url.insert_str(line),
            Focus::AuthToken => _ = self.auth_token.insert_str(line),
            _ => {}
        }
        self.error = None;
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if let Some(dialog) = self.exit_dialog {
            self.handle_exit_dialog_key(key, dialog);
            return;
        }
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        match key.code {
            // Escape and Ctrl+C only ask; leaving needs a confirmation.
            KeyCode::Esc => self.exit_dialog = Some(ExitDialogFocus::Stay),
            KeyCode::Char('c') if ctrl => self.exit_dialog = Some(ExitDialogFocus::Stay),
            KeyCode::Char('t') if ctrl && self.step == Step::Service => {
                self.show_token = !self.show_token;
            }
            KeyCode::Tab | KeyCode::Down => self.move_focus(1),
            KeyCode::BackTab | KeyCode::Up => self.move_focus(-1),
            _ => match self.focus {
                Focus::ApiUrl | Focus::AuthToken => self.handle_field_key(key),
                Focus::ShowToken => match key.code {
                    KeyCode::Enter | KeyCode::Char(' ') => self.show_token = !self.show_token,
                    _ => {}
                },
                Focus::ExtractTitle => self.handle_toggle_key(key),
                button => self.handle_button_key(key, button),
            },
        }
    }

    fn handle_field_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Enter => self.enter_from_field(),
            _ => {
                let field = if self.focus == Focus::ApiUrl {
                    &mut self.api_url
                } else {
                    &mut self.auth_token
                };
                field.input(key);
                self.error = None;
            }
        }
    }

    fn handle_toggle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(' ') | KeyCode::Left | KeyCode::Right => {
                self.extract_title = !self.extract_title;
            }
            KeyCode::Char('y' | 'Y') => self.extract_title = true,
            KeyCode::Char('n' | 'N') => self.extract_title = false,
            KeyCode::Enter => self.enter_from_field(),
            _ => {}
        }
    }

    fn handle_button_key(&mut self, key: KeyEvent, button: Focus) {
        debug_assert!(Self::is_button(button));
        match key.code {
            KeyCode::Enter | KeyCode::Char(' ') => self.activate(button),
            KeyCode::Left => self.move_focus(-1),
            KeyCode::Right => self.move_focus(1),
            _ => {}
        }
    }

    fn handle_exit_dialog_key(&mut self, key: KeyEvent, dialog: ExitDialogFocus) {
        let toggled = match dialog {
            ExitDialogFocus::Stay => ExitDialogFocus::Leave,
            ExitDialogFocus::Leave => ExitDialogFocus::Stay,
        };
        match key.code {
            // Escape on the dialog just closes it; it never exits.
            KeyCode::Esc => self.exit_dialog = None,
            KeyCode::Tab | KeyCode::BackTab | KeyCode::Left | KeyCode::Right => {
                self.exit_dialog = Some(toggled);
            }
            KeyCode::Enter => match dialog {
                ExitDialogFocus::Stay => self.exit_dialog = None,
                ExitDialogFocus::Leave => self.outcome = Some(SetupOutcome::Exited),
            },
            _ => {}
        }
    }

    fn handle_mouse(&mut self, mouse: MouseEvent) {
        if mouse.kind != MouseEventKind::Down(MouseButton::Left) {
            return;
        }
        let pos = Position::new(mouse.column, mouse.row);
        let a = &self.areas;
        let (exit_hit, next_hit, back_hit) = (
            a.exit.contains(pos),
            a.next.contains(pos),
            a.back.contains(pos),
        );
        if self.exit_dialog.is_some() {
            if a.dialog_stay.contains(pos) {
                self.exit_dialog = None;
            } else if a.dialog_leave.contains(pos) {
                self.outcome = Some(SetupOutcome::Exited);
            }
            return;
        }
        match self.step {
            Step::Service => {
                if a.api_url.contains(pos) {
                    self.focus = Focus::ApiUrl;
                } else if a.auth_token.contains(pos) {
                    self.focus = Focus::AuthToken;
                } else if a.show_token.contains(pos) {
                    self.focus = Focus::ShowToken;
                    self.show_token = !self.show_token;
                }
            }
            Step::Behavior => {
                if a.extract_yes.contains(pos) {
                    self.focus = Focus::ExtractTitle;
                    self.extract_title = true;
                } else if a.extract_no.contains(pos) {
                    self.focus = Focus::ExtractTitle;
                    self.extract_title = false;
                } else if back_hit {
                    self.activate(Focus::Back);
                }
            }
        }
        if exit_hit {
            self.activate(Focus::Exit);
        } else if next_hit {
            self.activate(Focus::Next);
        }
    }

    // ---- drawing -------------------------------------------------------

    fn draw(&mut self, frame: &mut Frame) {
        self.areas = Areas::default();
        let area = frame.area();
        let width = area.width.min(76);
        let height = area.height.min(26);
        let outer = Rect::new(
            area.x + (area.width - width) / 2,
            area.y + (area.height - height) / 2,
            width,
            height,
        );
        let step_label = match self.step {
            Step::Service => "Step 1 of 2: Micropub service",
            Step::Behavior => "Step 2 of 2: Default behavior",
        };
        let title = match self.mode {
            SetupMode::FirstRun => " Welcome to mptui ",
            SetupMode::Edit => " Setup ",
        };
        let block = Block::bordered()
            .border_set(border::ROUNDED)
            .border_style(Style::default().fg(Color::Cyan))
            .title(Line::from(title).bold())
            .title_bottom(Line::from(format!(" {step_label} ")).right_aligned())
            .padding(ratatui::widgets::Padding::new(1, 1, 1, 0));
        let inner = block.inner(outer);
        frame.render_widget(Clear, outer);
        frame.render_widget(block, outer);

        match self.step {
            Step::Service => self.draw_service_step(frame, inner),
            Step::Behavior => self.draw_behavior_step(frame, inner),
        }
        if let Some(dialog) = self.exit_dialog {
            self.draw_exit_dialog(frame, outer, dialog);
        }
    }

    fn draw_service_step(&mut self, frame: &mut Frame, inner: Rect) {
        let path_text = self.path.display().to_string();
        let (intro_start, intro_end) = match self.mode {
            SetupMode::FirstRun => (
                "No configuration file found at ",
                ". Let's create one. \
                 You can change these values later by editing the file.",
            ),
            SetupMode::Edit => ("Editing configuration file at ", "."),
        };
        // The path may be a single unbreakable word, so allow one extra line
        // for the wrap on top of the plain character count.
        let width = usize::from(inner.width.max(1));
        let intro_chars =
            intro_start.chars().count() + path_text.chars().count() + intro_end.chars().count();
        let intro_height = (intro_chars.div_ceil(width) + 1).min(8) as u16;
        let rows = Layout::vertical([
            Constraint::Length(intro_height), // intro
            Constraint::Length(1),            // url label
            Constraint::Length(2),            // url description
            Constraint::Length(3),            // url input
            Constraint::Length(1),            // token label
            Constraint::Length(2),            // token description
            Constraint::Length(3),            // token input
            Constraint::Length(1),            // error
            Constraint::Min(0),
            Constraint::Length(3), // buttons
        ])
        .split(inner);

        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::raw(intro_start),
                Span::styled(path_text, Style::default().fg(Color::Cyan)),
                Span::raw(intro_end),
            ]))
            .wrap(Wrap { trim: false }),
            rows[0],
        );
        frame.render_widget(label("API URL"), rows[1]);
        frame.render_widget(
            description(
                "The Micropub endpoint that posts are published to. \
                 Micro.blog users can keep the default.",
            ),
            rows[2],
        );
        render_input(
            frame,
            &mut self.api_url,
            rows[3],
            self.focus == Focus::ApiUrl,
        );
        self.areas.api_url = rows[3];

        let token_head =
            Layout::horizontal([Constraint::Min(0), Constraint::Length(22)]).split(rows[4]);
        frame.render_widget(label("Auth token"), token_head[0]);
        let marker = if self.show_token { "[x]" } else { "[ ]" };
        let toggle_style = if self.focus == Focus::ShowToken {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        frame.render_widget(
            Paragraph::new(format!("{marker} Show token (^T)"))
                .alignment(Alignment::Right)
                .style(toggle_style),
            token_head[1],
        );
        self.areas.show_token = token_head[1];
        if self.show_token {
            self.auth_token.clear_mask_char();
        } else {
            self.auth_token.set_mask_char('•');
        }
        frame.render_widget(
            description(
                "Secret token that authorizes mptui to post for you. \
                 It is stored in the config file, readable only by you.",
            ),
            rows[5],
        );
        render_input(
            frame,
            &mut self.auth_token,
            rows[6],
            self.focus == Focus::AuthToken,
        );
        self.areas.auth_token = rows[6];

        self.draw_error(frame, rows[7]);
        let buttons = Layout::horizontal([
            Constraint::Length(12),
            Constraint::Min(0),
            Constraint::Length(14),
        ])
        .split(rows[9]);
        self.areas.exit = buttons[0];
        self.areas.next = buttons[2];
        render_button(frame, buttons[0], "Exit", self.focus == Focus::Exit);
        render_button(frame, buttons[2], "Next ▶", self.focus == Focus::Next);
    }

    fn draw_behavior_step(&mut self, frame: &mut Frame, inner: Rect) {
        let rows = Layout::vertical([
            Constraint::Length(2), // intro
            Constraint::Length(1), // label
            Constraint::Length(4), // description
            Constraint::Length(1), // yes
            Constraint::Length(1), // no
            Constraint::Length(2), // note
            Constraint::Length(1), // error
            Constraint::Min(0),
            Constraint::Length(3), // buttons
        ])
        .split(inner);

        frame.render_widget(
            Paragraph::new("Almost done. Choose how new posts behave by default.")
                .wrap(Wrap { trim: true }),
            rows[0],
        );
        frame.render_widget(label("Extract title"), rows[1]);
        frame.render_widget(
            description(
                "When your text starts with a Markdown heading (for example \
                 \"# My title\"), use it as the post's title and remove it \
                 from the body. Otherwise the post is published without a title.",
            ),
            rows[2],
        );
        let focused = self.focus == Focus::ExtractTitle;
        frame.render_widget(
            radio("Yes, extract the title", self.extract_title, focused),
            rows[3],
        );
        frame.render_widget(
            radio(
                "No, publish the text as written",
                !self.extract_title,
                focused,
            ),
            rows[4],
        );
        self.areas.extract_yes = rows[3];
        self.areas.extract_no = rows[4];
        frame.render_widget(
            description("Use Space or ←/→ to change the selection."),
            rows[5],
        );

        self.draw_error(frame, rows[6]);
        let buttons = Layout::horizontal([
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Min(0),
            Constraint::Length(16),
        ])
        .spacing(1)
        .split(rows[8]);
        self.areas.back = buttons[0];
        self.areas.exit = buttons[1];
        self.areas.next = buttons[3];
        render_button(frame, buttons[0], "◀ Back", self.focus == Focus::Back);
        render_button(frame, buttons[1], "Exit", self.focus == Focus::Exit);
        render_button(frame, buttons[3], "Finish ✓", self.focus == Focus::Next);
    }

    fn draw_error(&self, frame: &mut Frame, area: Rect) {
        if let Some(error) = &self.error {
            frame.render_widget(
                Paragraph::new(format!("⚠ {error}")).style(Style::default().fg(Color::Red)),
                area,
            );
        }
    }

    fn draw_exit_dialog(&mut self, frame: &mut Frame, outer: Rect, focus: ExitDialogFocus) {
        let width = outer.width.min(52);
        let height = 9.min(outer.height);
        let area = Rect::new(
            outer.x + (outer.width - width) / 2,
            outer.y + (outer.height - height) / 2,
            width,
            height,
        );
        let block = Block::bordered()
            .border_set(border::ROUNDED)
            .title(" Exit setup? ")
            .border_style(Style::default().fg(Color::Yellow));
        let inner = block.inner(area);
        frame.render_widget(Clear, area);
        frame.render_widget(block, area);

        let rows = Layout::vertical([Constraint::Min(1), Constraint::Length(3)]).split(inner);
        frame.render_widget(
            Paragraph::new(match self.mode {
                SetupMode::FirstRun => {
                    "No configuration file will be created,\nand mptui will quit."
                }
                SetupMode::Edit => "The configuration file will not be changed.",
            })
            .alignment(Alignment::Center),
            rows[0],
        );
        let buttons = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .spacing(1)
            .split(rows[1]);
        self.areas.dialog_stay = buttons[0];
        self.areas.dialog_leave = buttons[1];
        render_button(
            frame,
            buttons[0],
            "Continue setup",
            focus == ExitDialogFocus::Stay,
        );
        render_button(frame, buttons[1], "Exit", focus == ExitDialogFocus::Leave);
    }
}

fn label(text: &str) -> Paragraph<'_> {
    Paragraph::new(text).style(Style::default().add_modifier(Modifier::BOLD))
}

fn description(text: &str) -> Paragraph<'_> {
    Paragraph::new(text)
        .style(Style::default().fg(Color::DarkGray))
        .wrap(Wrap { trim: true })
}

fn radio(text: &str, selected: bool, focused: bool) -> Paragraph<'_> {
    let marker = if selected { "(●)" } else { "( )" };
    let style = match (selected, focused) {
        (true, true) => Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
        (true, false) => Style::default().fg(Color::Cyan),
        _ => Style::default(),
    };
    Paragraph::new(Line::from(Span::styled(
        format!("  {marker} {text}"),
        style,
    )))
}

fn render_input(frame: &mut Frame, textarea: &mut TextArea<'static>, area: Rect, focused: bool) {
    let color = if focused {
        Color::Cyan
    } else {
        Color::DarkGray
    };
    textarea.set_block(
        Block::bordered()
            .border_set(border::ROUNDED)
            .border_style(Style::default().fg(color)),
    );
    textarea.set_cursor_style(if focused {
        Style::default().add_modifier(Modifier::REVERSED)
    } else {
        Style::default()
    });
    frame.render_widget(&*textarea, area);
}

fn render_button(frame: &mut Frame, area: Rect, text: &str, focused: bool) {
    let style = if focused {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    frame.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(style)
            .block(
                Block::bordered()
                    .border_set(border::ROUNDED)
                    .border_style(style),
            ),
        area,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{EndpointState, parse_micropub_settings};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "mptui-setup-{}-{name}/mp/config.toml",
            std::process::id()
        ))
    }

    fn type_text(setup: &mut Setup, text: &str) {
        for c in text.chars() {
            setup.handle_key(key(KeyCode::Char(c)));
        }
    }

    #[test]
    fn rendered_config_parses_and_forces_quiet_false() {
        let text = render_config("https://example.com/micropub", "se\"cret", false);
        assert!(text.contains("quiet = false"));
        let settings = parse_micropub_settings(&text);
        assert_eq!(
            settings.endpoint,
            EndpointState::Configured("example.com/micropub".into())
        );
        assert_eq!(settings.service_auth_token.as_deref(), Some("se\"cret"));
        assert!(!settings.extract_title);
    }

    #[test]
    fn escape_asks_for_confirmation_instead_of_exiting() {
        let mut setup = Setup::new(temp_path("esc"), SetupMode::FirstRun);
        setup.handle_key(key(KeyCode::Esc));
        assert!(setup.exit_dialog.is_some());
        assert_eq!(setup.outcome, None);
        // Escape again only dismisses the dialog.
        setup.handle_key(key(KeyCode::Esc));
        assert!(setup.exit_dialog.is_none());
        assert_eq!(setup.outcome, None);
    }

    #[test]
    fn confirming_exit_ends_without_writing_config() {
        let path = temp_path("exit");
        let mut setup = Setup::new(path.clone(), SetupMode::FirstRun);
        setup.handle_key(key(KeyCode::Esc));
        setup.handle_key(key(KeyCode::Tab));
        setup.handle_key(key(KeyCode::Enter));
        assert_eq!(setup.outcome, Some(SetupOutcome::Exited));
        assert!(!path.exists());
    }

    #[test]
    fn edit_mode_intro_and_prefill() {
        use ratatui::{Terminal, backend::TestBackend};

        let path = temp_path("edit");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(
            &path,
            "[service]\napi_url = \"https://example.com/micropub\"\nauth_token = \"tok\"\n\n[default_behavior]\nextract_title = false\n",
        )
        .unwrap();
        let mut setup = Setup::new(path.clone(), SetupMode::Edit);
        assert_eq!(setup.api_url_text(), "https://example.com/micropub");
        assert_eq!(setup.auth_token_text(), "tok");
        assert!(!setup.extract_title);

        let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
        terminal.draw(|frame| setup.draw(frame)).unwrap();
        let text: String = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect();
        assert!(text.contains("Editing configuration file at"));
        assert!(!text.contains("No configuration file found"));

        // Exiting leaves the file untouched.
        setup.handle_key(key(KeyCode::Esc));
        setup.handle_key(key(KeyCode::Tab));
        setup.handle_key(key(KeyCode::Enter));
        assert_eq!(setup.outcome, Some(SetupOutcome::Exited));
        assert!(fs::read_to_string(&path).unwrap().contains("\"tok\""));
        let _ = fs::remove_dir_all(path.parent().unwrap());
    }

    #[test]
    fn exit_dialog_text_depends_on_mode() {
        use ratatui::{Terminal, backend::TestBackend};

        let render = |mode| {
            let mut setup = Setup::new(temp_path("dialog"), mode);
            setup.handle_key(key(KeyCode::Esc));
            let mut terminal = Terminal::new(TestBackend::new(100, 30)).unwrap();
            terminal.draw(|frame| setup.draw(frame)).unwrap();
            terminal
                .backend()
                .buffer()
                .content()
                .iter()
                .map(|cell| cell.symbol())
                .collect::<String>()
        };

        let first_run = render(SetupMode::FirstRun);
        assert!(first_run.contains("mptui will quit"));
        assert!(!first_run.contains("will not be changed"));

        let edit = render(SetupMode::Edit);
        assert!(edit.contains("Exit setup?"));
        assert!(edit.contains("The configuration file will not be changed."));
        assert!(!edit.contains("quit"));
    }

    #[test]
    fn first_step_intro_shows_wrapped_config_path() {
        use ratatui::{Terminal, backend::TestBackend};

        let path = PathBuf::from(format!("/{}/config.toml", "Q".repeat(150)));
        let mut setup = Setup::new(path, SetupMode::FirstRun);
        let mut terminal = Terminal::new(TestBackend::new(80, 30)).unwrap();
        terminal.draw(|frame| setup.draw(frame)).unwrap();
        let q_count = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .filter(|cell| cell.symbol() == "Q")
            .count();
        assert_eq!(q_count, 150, "path was clipped");

        // The second form no longer shows the path.
        setup.step = Step::Behavior;
        terminal.draw(|frame| setup.draw(frame)).unwrap();
        let text: String = terminal
            .backend()
            .buffer()
            .content()
            .iter()
            .map(|cell| cell.symbol())
            .collect();
        assert!(!text.contains("QQQ"));
    }

    #[test]
    fn show_token_toggle_changes_mask() {
        let mut setup = Setup::new(temp_path("show"), SetupMode::FirstRun);
        assert!(!setup.show_token);
        setup.handle_key(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::CONTROL));
        assert!(setup.show_token);
        // Reachable by keyboard: Tab to the checkbox, Space toggles.
        setup.focus = Focus::AuthToken;
        setup.handle_key(key(KeyCode::Tab));
        assert_eq!(setup.focus, Focus::ShowToken);
        setup.handle_key(key(KeyCode::Char(' ')));
        assert!(!setup.show_token);
    }

    #[test]
    fn next_requires_token() {
        let mut setup = Setup::new(temp_path("token"), SetupMode::FirstRun);
        setup.go_next();
        assert_eq!(setup.step, Step::Service);
        assert_eq!(setup.focus, Focus::AuthToken);
        assert!(setup.error.is_some());
    }

    #[test]
    fn full_flow_writes_config() {
        let path = temp_path("flow");
        let mut setup = Setup::new(path.clone(), SetupMode::FirstRun);
        setup.handle_key(key(KeyCode::Enter)); // token empty: stays put
        assert_eq!(setup.focus, Focus::ApiUrl);
        setup.handle_key(key(KeyCode::Tab));
        type_text(&mut setup, "tok");
        setup.handle_key(key(KeyCode::Enter)); // all filled: jumps to Next
        assert_eq!(setup.focus, Focus::Next);
        setup.handle_key(key(KeyCode::Enter));
        assert_eq!(setup.step, Step::Behavior);
        setup.handle_key(key(KeyCode::Char('n')));
        setup.handle_key(key(KeyCode::Enter)); // toggle: jumps to Finish
        assert_eq!(setup.focus, Focus::Next);
        setup.handle_key(key(KeyCode::Enter));
        assert_eq!(setup.outcome, Some(SetupOutcome::Completed));

        let written = fs::read_to_string(&path).unwrap();
        let settings = parse_micropub_settings(&written);
        assert_eq!(settings.service_api_url.as_deref(), Some(DEFAULT_API_URL));
        assert_eq!(settings.service_auth_token.as_deref(), Some("tok"));
        assert!(!settings.extract_title);
        assert!(written.contains("quiet = false"));
        fs::remove_dir_all(path.parent().unwrap().parent().unwrap()).ok();
    }
}
