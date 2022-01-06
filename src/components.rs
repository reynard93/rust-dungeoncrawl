pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair, // helper class from bracket-lib that stores both a foreground and bg in a single struct
    pub glyph: FontCharType, // store a single char / glyph
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player; // empty struct, serves as a 'tag'

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;
