//! Core terminal view implementation

use gpui::*;
use crate::terminal::{AnsiEvent, AnsiParser, Pty, PtyConfig, PtyEvent};
use super::types::*;

/// Terminal view state
pub struct TerminalView {
    /// PTY instance
    pub(crate) pty: Option<Pty>,
    /// ANSI parser
    pub(crate) parser: AnsiParser,
    /// Terminal lines
    pub(crate) lines: Vec<TerminalLine>,
    /// Current line being built
    pub(crate) current_line: TerminalLine,
    /// Cursor position
    pub(crate) cursor_row: usize,
    pub(crate) cursor_col: usize,
    /// Input buffer
    pub(crate) input_buffer: String,
    /// Terminal size (cols, rows)
    pub(crate) size: (u16, u16),
    /// Scroll offset
    pub(crate) scroll_offset: usize,
    /// Focus handle
    pub(crate) focus_handle: FocusHandle,
    /// Command history
    pub(crate) command_history: Vec<String>,
    /// History index
    pub(crate) history_index: Option<usize>,
    /// Is running
    pub(crate) is_running: bool,
    /// Auto-scroll enabled
    pub(crate) auto_scroll: bool,
    /// Selection start (row, col)
    pub(crate) selection_start: Option<(usize, usize)>,
    /// Selection end (row, col)
    pub(crate) selection_end: Option<(usize, usize)>,
}

impl TerminalView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            pty: None,
            parser: AnsiParser::new(),
            lines: Vec::new(),
            current_line: TerminalLine { spans: Vec::new() },
            cursor_row: 0,
            cursor_col: 0,
            input_buffer: String::new(),
            size: (80, 24),
            scroll_offset: 0,
            focus_handle: cx.focus_handle(),
            command_history: Vec::new(),
            history_index: None,
            is_running: false,
            auto_scroll: true,
            selection_start: None,
            selection_end: None,
        }
    }

    /// Start the terminal
    pub fn start(&mut self, cwd: Option<std::path::PathBuf>, cx: &mut Context<Self>) {
        let config = PtyConfig {
            cwd,
            size: self.size,
            ..Default::default()
        };

        let mut pty = Pty::with_config(config);

        // Start in a spawned task
        cx.spawn(async move |this, cx| {
            let mut pty_instance = pty;
            match pty_instance.start().await {
                Ok(mut rx) => {
                    let _ = this.update(cx, |this, cx| {
                        this.is_running = true;
                        cx.notify();
                    })
                    .ok();

                    // Process events
                    while let Some(event) = rx.recv().await {
                        let should_break = this
                            .update(cx, |this, cx| {
                                this.handle_pty_event(event.clone(), cx);
                                matches!(event, PtyEvent::Exit(_))
                            })
                            .unwrap_or(true);

                        if should_break {
                            break;
                        }
                    }
                }
                Err(e) => {
                    this.update(cx, |this, cx| {
                        this.add_line(&format!("Error: {}", e), cx);
                        cx.emit(TerminalViewEvent::Error(e.to_string()));
                    })
                    .ok();
                }
            }
        })
        .detach();
    }

    /// Handle PTY event
    pub(crate) fn handle_pty_event(&mut self, event: PtyEvent, cx: &mut Context<Self>) {
        match event {
            PtyEvent::Output(text) => {
                self.process_output(&text, cx);
                cx.emit(TerminalViewEvent::OutputReceived(text));
            }
            PtyEvent::Exit(code) => {
                self.is_running = false;
                self.add_line(&format!("\n[Process exited with code {}]", code), cx);
                cx.emit(TerminalViewEvent::ProcessExited(code));
            }
            PtyEvent::Error(msg) => {
                self.add_line(&format!("\n[Error: {}]", msg), cx);
                cx.emit(TerminalViewEvent::Error(msg));
            }
            PtyEvent::TitleChanged(title) => {
                // Could emit title change event
            }
            PtyEvent::Bell => {
                // Could play bell sound or flash
            }
        }
        cx.notify();
    }

    /// Process terminal output
    pub(crate) fn process_output(&mut self, text: &str, _cx: &mut Context<Self>) {
        let events = self.parser.parse(text);

        for event in events {
            match event {
                AnsiEvent::Text(t) => {
                    self.current_line.spans.push(StyledSpan {
                        text: t,
                        style: self.parser.current_style().clone(),
                    });
                }
                AnsiEvent::Newline => {
                    self.lines.push(std::mem::replace(
                        &mut self.current_line,
                        TerminalLine { spans: Vec::new() },
                    ));
                    self.cursor_row += 1;
                    self.cursor_col = 0;
                }
                AnsiEvent::CarriageReturn => {
                    self.cursor_col = 0;
                }
                AnsiEvent::Tab => {
                    let spaces = 8 - (self.cursor_col % 8);
                    self.current_line.spans.push(StyledSpan {
                        text: " ".repeat(spaces),
                        style: crate::terminal::TextStyle::default(),
                    });
                    self.cursor_col += spaces;
                }
                AnsiEvent::ClearScreen => {
                    self.lines.clear();
                    self.current_line = TerminalLine { spans: Vec::new() };
                    self.cursor_row = 0;
                    self.cursor_col = 0;
                }
                AnsiEvent::ClearLine => {
                    self.current_line = TerminalLine { spans: Vec::new() };
                }
                AnsiEvent::Style(_) | AnsiEvent::Bell | AnsiEvent::SetTitle(_) => {
                    // Style is handled by parser, others are handled elsewhere
                }
                _ => {}
            }
        }

        // Auto-scroll
        if self.auto_scroll {
            self.scroll_to_bottom();
        }
    }

    /// Add a line of text
    pub(crate) fn add_line(&mut self, text: &str, cx: &mut Context<Self>) {
        self.process_output(text, cx);
        if !text.ends_with('\n') {
            self.process_output("\n", cx);
        }
    }
}
