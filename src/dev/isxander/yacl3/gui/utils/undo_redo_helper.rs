//! Ported from dev.isxander.yacl3.gui.utils.UndoRedoHelper by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class UndoRedoHelper
#[derive(Clone)]
pub struct UndoRedoHelper {
    /// java: private final List<FieldState> history = new ArrayList<>();
    pub history: Vec<FieldState>,
    /// java: private int index = 0;
    pub index: i32,
}

impl UndoRedoHelper {
    /// java: public UndoRedoHelper(String text, int cursorPos, int selectionLength)
    pub fn new(mut text: String, mut cursor_pos: i32, mut selection_length: i32) -> Self {
        let mut this = Self {
            history: Vec::new(),
            index: 0,
        };
        this.history.push(FieldState::new(text.clone(), cursor_pos, selection_length));
        this
    }

    /// java: public void save(String text, int cursorPos, int selectionLength)
    pub fn save(&mut self, text: &str, mut cursor_pos: i32, mut selection_length: i32) {
        // java: history.subList(index, max).clear() - clears the list's own tail
        self.history.truncate(self.index as usize);
        self.history.push(FieldState::new(text.to_string(), cursor_pos, selection_length));
        self.index += 1;
    }

    /// java: public @Nullable FieldState undo()
    pub fn undo(&mut self) -> Option<FieldState> {
        self.index -= 1;
        self.index = self.index.max(0);
        if self.history.is_empty() {
            return None;
        }
        return Some(self.history[(self.index) as usize].clone());
    }

    /// java: public @Nullable FieldState redo()
    pub fn redo(&mut self) -> Option<FieldState> {
        if self.index < (self.history.len() as i32).wrapping_sub(1) {
            self.index += 1;
            return Some(self.history[(self.index) as usize].clone());
        } else {
            return None;
        }
    }

}

/// java: public record FieldState(String text, int cursorPos, int selectionLength)
#[derive(Clone, Debug, PartialEq)]
pub struct FieldState {
    /// java: String text
    pub text: String,
    /// java: int cursorPos
    pub cursor_pos: i32,
    /// java: int selectionLength
    pub selection_length: i32,
}

impl FieldState {
    /// java: the canonical record constructor
    pub fn new(mut text: String, mut cursor_pos: i32, mut selection_length: i32) -> Self {
        Self { text, cursor_pos, selection_length }
    }

    /// java: the record accessor `text()`
    pub fn text(&self) -> String {
        self.text.clone()
    }

    /// java: the record accessor `cursorPos()`
    pub fn cursor_pos(&self) -> i32 {
        self.cursor_pos
    }

    /// java: the record accessor `selectionLength()`
    pub fn selection_length(&self) -> i32 {
        self.selection_length
    }

}
