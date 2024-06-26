use crate::unicode::{is_newline, move_grapheme};
use ropey::{Rope, RopeSlice};
use std::collections::VecDeque;
use std::ops::Range;
use unicode_width::UnicodeWidthChar;

pub trait LineLayout {
    type Iter<'a>: Iterator<Item = GraphemePosition>;

    fn layout_line<'a>(&self, line: RopeSlice<'a>) -> Self::Iter<'a>;
}

pub struct GraphemePosition {

    pub start_column: usize,

    pub end_column: usize,

    pub cursor: usize,
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum EditorAction {

    Delete(usize, String),

    Insert(usize, String),
}

pub struct TextEditor<L: LineLayout> {

    text: Rope,

    cursor: usize,

    target_column: usize,

    scroll_lines: usize,

    scroll_columns: usize,

    layout_settings: L,

    selection_anchor: Option<usize>,

    history: VecDeque<EditorAction>,

    save_anchor: Option<usize>,

    current_history: usize,

    history_size: usize,

    tab_width: usize,
}

impl<L: LineLayout> TextEditor<L> {

    pub fn new(content: &str, layout_settings: L, tab_width: usize, newly_loaded: bool) -> Self {
        Self {
            text: Rope::from_str(content),
            cursor: 0,
            target_column: 0,
            scroll_lines: 0,
            scroll_columns: 0,
            layout_settings,
            selection_anchor: None,
            history: VecDeque::new(),
            save_anchor: if newly_loaded { None } else { Some(0) },
            current_history: 0,
            history_size: 16384,
            tab_width,
        }
    }

    pub fn get_text(&self) -> RopeSlice {
        self.text.slice(..)
    }

    pub fn get_lines_scrolled(&self) -> usize {
        self.scroll_lines
    }

    pub fn get_columns_scrolled(&self) -> usize {
        self.scroll_columns
    }

    pub fn get_tab_width(&self) -> usize {
        self.tab_width
    }

    pub fn to_string(&self) -> String {
        self.text.to_string()
    }

    pub fn len_lines(&self) -> usize {
        self.text.len_lines()
    }

    pub fn get_first_visible_line(&self) -> usize {
        self.scroll_lines
    }

    pub fn get_current_line(&self) -> usize {
        self.text.byte_to_line(self.cursor)
    }

    pub fn set_saved(&mut self) {
        self.save_anchor = Some(self.current_history);
    }

    pub fn has_changed_since_save(&self) -> bool {
        self.save_anchor != Some(self.current_history)
    }

    pub fn discard_changes(&mut self) {

        while self
            .save_anchor
            .map(|x| self.current_history > x)
            .unwrap_or(false)
        {
            self.undo();
        }

        while self
            .save_anchor
            .map(|x| self.current_history < x)
            .unwrap_or(false)
        {
            self.redo();
        }
    }

    pub fn undo(&mut self) {

        if self.current_history > 0 {

            self.current_history -= 1;

            let change = self.history[self.current_history].clone();

            match change {
                EditorAction::Delete(cursor, string) => {

                    self.insert_string(cursor, &string, false, true, true);
                }
                EditorAction::Insert(cursor, string) => {

                    self.remove_range(cursor, cursor + string.len(), false, true, true);
                }
            }
        }
    }

    pub fn redo(&mut self) {

        if self.history.len() > self.current_history {

            let change = self.history[self.current_history].clone();

            self.current_history += 1;

            match change {
                EditorAction::Insert(cursor, string) => {

                    self.insert_string(cursor, &string, false, true, true);
                }
                EditorAction::Delete(cursor, string) => {

                    self.remove_range(cursor, cursor + string.len(), false, true, true);
                }
            }
        }
    }

    pub fn do_change(&mut self, change: EditorAction) {

        while self.history.len() > self.current_history {
            self.history.pop_back();
        }

        self.history.push_back(change);
        self.current_history += 1;

        while self.current_history > 0
            && self.save_anchor.map(|x| x > 0).unwrap_or(true)
            && self.history.len() > self.history_size
        {

            self.current_history -= 1;
            self.save_anchor = self.save_anchor.map(|x| x - 1);

            self.history.pop_front();
        }
    }

    pub fn clear_selection(&mut self) {
        self.selection_anchor = None;
    }

    pub fn get_selection(&self) -> Option<String> {

        self.get_selection_range()
            .map(|range| self.text.byte_to_char(range.start)..self.text.byte_to_char(range.end))
            .map(|range| self.text.slice(range).to_string())
    }

    pub fn cut_selection(&mut self) -> Option<String> {

        let range = self.get_selection_range()?;

        let string = self
            .text
            .slice(self.text.byte_to_char(range.start)..self.text.byte_to_char(range.end))
            .to_string();

        self.remove_range(range.start, range.end, true, true, true);

        self.clear_selection();

        Some(string)
    }

    pub fn get_selection_range(&self) -> Option<Range<usize>> {
        self.selection_anchor.map(|x| {
            if x > self.cursor {
                self.cursor..x
            } else {
                x..self.cursor
            }
        })
    }

    pub fn add_selection(&mut self) {

        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor)
        }
    }

    pub fn get_character_under_cursor(&self) -> char {
        self.text.char(self.text.byte_to_char(self.cursor))
    }

    pub fn get_character_in_front_of_cursor(&self) -> Option<char> {
        self.text
            .get_char(self.text.byte_to_char(self.cursor).checked_sub(1)?)
    }

    pub fn move_cursor_horizontal(
        &mut self,
        amount: isize,
        add_selection: bool,
        save_column: bool,
    ) {

        if add_selection {
            self.add_selection();
        } else {

            self.cursor = if amount > 0 {
                self.cursor
                    .max(self.selection_anchor.unwrap_or(self.cursor))
            } else {
                self.cursor
                    .min(self.selection_anchor.unwrap_or(self.cursor))
            };

            self.clear_selection();
        }

        self.cursor = move_grapheme(amount, self.cursor, self.text.slice(..));

        if save_column {
            self.target_column = self.get_cursor_column();
        }
    }

    pub fn move_cursor_horizontal_words(
        &mut self,
        amount: isize,
        add_selection: bool,
        save_column: bool,
    ) {

        if amount > 0 {

            while self.cursor < self.text.len_bytes()
                && self.get_character_under_cursor().is_whitespace()
            {
                self.move_cursor_horizontal(1, add_selection, save_column);
            }

            while self.cursor < self.text.len_bytes()
                && !self.get_character_under_cursor().is_whitespace()
            {
                self.move_cursor_horizontal(1, add_selection, save_column);
            }
        } else {

            while self.cursor > 0
                && self
                .get_character_in_front_of_cursor()
                .unwrap()
                .is_whitespace()
            {
                self.move_cursor_horizontal(-1, add_selection, save_column);
            }

            while self.cursor > 0
                && !self
                .get_character_in_front_of_cursor()
                .unwrap()
                .is_whitespace()
            {
                self.move_cursor_horizontal(-1, add_selection, save_column);
            }
        }
    }

    pub fn move_cursor_vertical(&mut self, amount: isize, add_selection: bool, save_column: bool) {

        if add_selection {
            self.add_selection();
        } else {

            self.cursor = if amount > 0 {
                self.cursor
                    .max(self.selection_anchor.unwrap_or(self.cursor))
            } else {
                self.cursor
                    .min(self.selection_anchor.unwrap_or(self.cursor))
            };

            self.clear_selection();
        }

        let next_line = (self.text.byte_to_line(self.cursor) as isize + amount)
            .max(0)
            .min(self.text.len_lines().saturating_sub(1) as isize) as usize;

        self.cursor = self.text.line_to_byte(next_line);

        self.move_cursor_to_column(self.target_column, add_selection, save_column);
    }

    pub fn insert_character_at_cursor(&mut self, character: char) {

        self.cut_selection();

        let mut buffer = [0_u8; 4];
        let string = character.encode_utf8(&mut buffer);

        self.insert_string(self.cursor, string, true, true, true);
    }

    pub fn insert_string_at_cursor(&mut self, string: &str) {

        self.cut_selection();

        self.insert_string(self.cursor, string, true, true, true);
    }

    pub fn insert_tab_at_cursor(&mut self) {

        self.insert_character_at_cursor('\t');
    }

    pub fn insert_newline_at_cursor(&mut self) {

        self.cut_selection();

        let line_num = self.text.byte_to_line(self.cursor);

        let line = self.text.line(line_num);

        let line_char_start = self.text.line_to_char(line_num);

        let line_char_pos = self.text.byte_to_char(self.cursor);

        let pred_whitespace = line
            .chars()
            .take_while(|x| x.is_whitespace() && !is_newline(*x))
            .take(line_char_pos - line_char_start);

        let string = "\n".chars().chain(pred_whitespace).collect::<String>();

        self.insert_string(self.cursor, &string, true, true, true);
    }

    pub fn remove_character_or_selection_at_cursor(&mut self, before: bool) {

        if self.cut_selection().is_some() {

            return;
        }

        let end_range = move_grapheme(
            if before { -1 } else { 1 },
            self.cursor,
            self.text.slice(..),
        );

        let (start, end) = if before {
            (end_range, self.cursor)
        } else {
            (self.cursor, end_range)
        };

        self.remove_range(start, end, true, true, true);
    }

    pub fn insert_string(
        &mut self,
        start: usize,
        string: &str,
        record: bool,
        store_cursor: bool,
        move_cursor_after: bool,
    ) {

        self.clear_selection();

        let start_char = self.text.byte_to_char(start);

        self.text.insert(start_char, string);

        if record && !string.is_empty() {
            self.do_change(EditorAction::Insert(start, string.to_string()));
        }

        if self.cursor > start {

            self.cursor += string.len();
        }

        if move_cursor_after {

            self.cursor = start + string.len();
        }

        if store_cursor {
            self.target_column = self.get_cursor_column();
        }
    }

    pub fn remove_range(
        &mut self,
        start: usize,
        end: usize,
        record: bool,
        store_cursor: bool,
        move_cursor_after: bool,
    ) {

        self.clear_selection();

        let start_char = self.text.byte_to_char(start);
        let end_char = self.text.byte_to_char(end);

        let string = self.text.slice(start_char..end_char).to_string();

        self.text.remove(start_char..end_char);

        if record && !string.is_empty() {
            self.do_change(EditorAction::Delete(start, string));
        }

        if self.cursor >= start && self.cursor < end {

            self.cursor = start;
        } else if self.cursor > start {

            self.cursor -= end - start;
        }

        if move_cursor_after {
            self.cursor = start;
        }

        if store_cursor {
            self.target_column = self.get_cursor_column();
        }
    }

    pub fn move_cursor_to_column(&mut self, column: usize, add_selection: bool, save_column: bool) {

        if add_selection {
            self.add_selection();
        } else {
            self.clear_selection();
        }

        self.move_cursor_to_start_of_line(add_selection, save_column);

        let line = self.text.line(self.text.byte_to_line(self.cursor));

        let cursor_pos = self
            .layout_settings
            .layout_line(line)
            .take_while(|x| x.start_column <= column)
            .last()
            .map(|x| self.cursor + x.cursor);

        if let Some(cursor) = cursor_pos {
            self.cursor = cursor;
        } else {

            self.move_cursor_to_end_of_line(add_selection, save_column);
        }
    }

    pub fn move_cursor_to_start_of_line(&mut self, add_selection: bool, save_column: bool) {

        if add_selection {
            self.add_selection();
        } else {
            self.clear_selection();
        }

        self.cursor = self.text.line_to_byte(self.text.byte_to_line(self.cursor));
        if save_column {
            self.target_column = 0;
        }
    }

    pub fn move_cursor_to_end_of_line(&mut self, add_selection: bool, save_column: bool) {

        if add_selection {
            self.add_selection();
        } else {
            self.clear_selection();
        }

        if self.cursor != self.text.len_bytes() {

            let line = self.text.byte_to_line(self.cursor);

            let next_line_start = self
                .text
                .line_to_byte((line + 1).min(self.text.len_lines()));

            self.cursor = next_line_start;

            if line + 1 < self.text.len_lines() {
                self.move_cursor_horizontal(-1, add_selection, save_column);
            }
        }
    }

    pub fn get_cursor_column(&self) -> usize {

        let line_num = self.text.byte_to_line(self.cursor);

        let line = self.text.line(line_num);

        let line_start = self.text.line_to_byte(line_num);

        self.layout_settings
            .layout_line(line)
            .take_while(|x| x.cursor + line_start < self.cursor)
            .last()
            .map(|x| x.end_column)
            .unwrap_or(0)
    }

    pub fn get_row_and_column(&self) -> (usize, usize) {

        let line_num = self.text.byte_to_line(self.cursor);

        let line = self.text.line(line_num);

        let line_start = self.text.line_to_char(line_num);

        let line_pos = self.text.byte_to_char(self.cursor) - line_start;

        let column = line
            .chars()
            .take(line_pos)
            .map(|x| x.width_cjk().unwrap_or(0))
            .sum();

        (line_num, column)
    }

    pub fn get_cursor_pos(&self) -> (usize, usize) {

        let line = self.text.byte_to_line(self.cursor);

        let column = self.get_cursor_column();

        (column, line)
    }

    pub fn get_relative_cursor_pos(&self) -> Option<(usize, usize)> {
        let (x, y) = self.get_cursor_pos();

        Some((
            x.checked_sub(self.scroll_columns)?,
            y.checked_sub(self.scroll_lines)?,
        ))
    }

    pub fn set_cursor_pos(&mut self, x: usize, y: usize, add_selection: bool) {

        if add_selection {
            self.add_selection();
        } else {
            self.clear_selection();
        }

        let line = y.min(self.text.len_lines());

        if line != self.text.len_lines() {

            let line_start = self.text.line_to_byte(line);

            let cursor_pos = self
                .layout_settings
                .layout_line(self.text.line(line))
                .take_while(|y| y.start_column <= x)
                .last()
                .map(|x| line_start + x.cursor);

            if let Some(cursor) = cursor_pos {
                self.cursor = cursor;
            } else {

                self.move_cursor_to_end_of_line(add_selection, true);
            }
        } else {

            self.cursor = self.text.len_bytes();
        }
    }

    pub fn set_relative_cursor_pos(&mut self, x: usize, y: usize, add_selection: bool) {

        self.set_cursor_pos(
            x + self.scroll_columns,
            y + self.scroll_lines,
            add_selection,
        );
    }

    pub fn scroll_vertically(&mut self, amount: isize) {

        self.scroll_lines = self.scroll_lines.saturating_add_signed(amount);

        self.move_cursor_vertical(amount, false, false);
    }

    pub fn set_scroll(
        &mut self,
        width: usize,
        height: usize,
        width_margin: usize,
        height_margin: usize,
    ) {
        let (x, y) = self.get_cursor_pos();

        self.scroll_lines = self
            .scroll_lines
            .max(y.saturating_sub(height.saturating_sub(height_margin.min(height / 2) + 1)))
            .min(y.saturating_sub(height_margin.min(height / 2)));

        self.scroll_columns = self
            .scroll_columns
            .max(x.saturating_sub(width.saturating_sub(width_margin.min(width / 2) + 1)))
            .min(x.saturating_sub(width_margin.min(width / 2)));
    }
}