use crate::unicode::{TERM_TAB_WIDTH, move_grapheme, string_width};
use crate::editor::{LineLayout, GraphemePosition};
use ropey::RopeSlice;
use crossterm::{
    cursor::{self, SetCursorStyle},
    event::{
        DisableMouseCapture, EnableMouseCapture, 
    },
    execute, queue, style,
    style::Color,
    terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen},
};
use std::io::{stdout, Write};

pub struct TermLineLayoutSettings {
    tab_width: usize,
}

impl TermLineLayoutSettings {
    pub fn new(tab_width: usize) -> Self {
        Self { tab_width }
    }
}

impl LineLayout for TermLineLayoutSettings {
    type Iter<'a> = TermLineLayout<'a>;

    fn layout_line<'a>(&self, line: RopeSlice<'a>) -> TermLineLayout<'a> {
        TermLineLayout::new(line, self.tab_width)
    }
}

pub struct TermLineLayout<'a> {
    line: RopeSlice<'a>,
    cursor: usize,
    column: usize,
    tab_width: usize,
}

impl<'a> TermLineLayout<'a> {
    pub fn new(line: RopeSlice<'a>, tab_width: usize) -> Self {
        Self {
            line,
            tab_width,
            cursor: 0,
            column: 0,
        }
    }
}

impl<'a> Iterator for TermLineLayout<'a> {
    type Item = GraphemePosition;

    fn next(&mut self) -> Option<GraphemePosition> {
        if self.cursor == self.line.len_bytes() {
            None
        } else {
            let next_cursor = move_grapheme(1, self.cursor, self.line);
            let rope_slice = self.line.byte_slice(self.cursor..next_cursor);
            let grapheme_width = string_width(rope_slice.chars(), self.tab_width);
            let grapheme = GraphemePosition {
                start_column: self.column,
                end_column: self.column + grapheme_width,
                cursor: self.cursor,
            };

            self.cursor = next_cursor;
            self.column += grapheme_width;
            Some(grapheme)
        }
    }
}

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Char {
    pub c: char,
    pub color: Highlight,
}

impl Char {
    pub fn new_text(c: char, selected: bool) -> Self {
        Self {
            c,
            color: if selected {
                Highlight::Selection
            } else {
                Highlight::Text
            },
        }
    }

    pub fn new(c: char, color: Highlight) -> Self {
        Self { c, color }
    }
}

pub type TerminalBuffer = (Vec<Char>, Option<(usize, usize)>);

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Highlight {
    Text,
    Selection,
    Gutter,
    Status,
}

impl Highlight {
    pub fn get_color_foreground_crossterm(self) -> Color {
        match self {
            Self::Text => Color::Reset,
            Self::Selection => Color::Black,
            Self::Gutter => Color::Yellow,
            Self::Status => Color::Black,
        }
    }

    pub fn get_color_background_crossterm(self) -> Color {
        match self {
            Self::Text => Color::Reset,
            Self::Selection => Color::Blue,
            Self::Gutter => Color::Reset,
            Self::Status => Color::Grey,
        }
    }
}

pub fn render(
    width: usize,
    cursor_position: Option<(usize, usize)>,
    buffer: &[Char],
    previous_buffer: &[Char],
) {
    let mut x = 0;
    let mut y = 0;
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut prev_chars = previous_buffer.iter().peekable();
    let mut force_move = true;
    let mut prev_fg = Color::Reset;
    let mut prev_bg = Color::Reset;

    queue!(
        stdout(),
        style::SetForegroundColor(Color::Reset),
        style::SetBackgroundColor(Color::Reset)
    )
    .unwrap();

    for c in buffer.iter() {
        if x != prev_x || y != prev_y || Some(&c) != prev_chars.peek() {
            if force_move {
                queue!(stdout(), cursor::MoveTo(x as u16, y as u16)).unwrap();
            }

            let fg = c.color.get_color_foreground_crossterm();
            let bg = c.color.get_color_background_crossterm();

            if fg != prev_fg {
                queue!(stdout(), style::SetForegroundColor(fg)).unwrap();
                prev_fg = fg;
            }

            if bg != prev_bg {
                queue!(stdout(), style::SetBackgroundColor(bg)).unwrap();
                prev_bg = bg;
            }

            queue!(stdout(), style::Print(c.c)).unwrap();

            force_move = !c.c.is_ascii() || c.c.is_ascii_control();
        } else {
            force_move = true;
        }

        x += string_width(std::iter::once(c.c), TERM_TAB_WIDTH);

        if x >= width {
            y += 1;
            x = 0;

            force_move = true;
        }

        while prev_x < x || prev_y < y {
            if let Some(c) = prev_chars.next() {
                prev_x += string_width(std::iter::once(c.c), TERM_TAB_WIDTH);

                if prev_x >= width {
                    prev_y += 1;
                    prev_x = 0;
                }
            } else {
                break;
            }
        }
    }

    if let Some((x, y)) = cursor_position {
        queue!(stdout(), cursor::Show, cursor::MoveTo(x as u16, y as u16),).unwrap();
    } else {
        queue!(stdout(), cursor::Hide).unwrap();
    };

    stdout().flush().unwrap();
}

pub fn setup_terminal(disable_mouse_interaction: bool) {
    std::panic::set_hook(Box::new(|info| {
        cleanup_terminal("Panic!");

        if let Some(msg) = info.payload().downcast_ref::<&str>() {
            println!("Cause: {:?}", msg);
        }

        if let Some(loc) = info.location() {
            println!("Location: {}", loc);
        }
    }));

    enable_raw_mode().unwrap();

    execute!(
        stdout(),
        cursor::SavePosition,
        EnterAlternateScreen,
        SetCursorStyle::BlinkingBar,
    )
    .unwrap();

    if !disable_mouse_interaction {
        execute!(stdout(), EnableMouseCapture).unwrap();
    }
}

pub fn cleanup_terminal(message: &str) {
    execute!(
        stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        cursor::RestorePosition,
        SetCursorStyle::BlinkingBlock,
        cursor::Show,
    )
    .unwrap();
    disable_raw_mode().unwrap();
    println!("{}", message);
}
