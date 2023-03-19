use crate::{
    comp::{Any, Plain, Text},
    core::Build,
    element::{ActionsElement, ContextElement, Element, InputElement, SectionElement},
    pre::*,
};

mod actions;
mod context;
mod divider;
mod file;
mod header;
mod image;
mod input;
mod section;
mod video;

pub use actions::Actions;
pub use context::Context;
pub use divider::Divider;
pub use file::File;
pub use header::Header;
pub use image::Image;
pub use input::Input;
pub use section::Section;
pub use video::Video;

pub trait Block: Build {}

/// Convert any type into blocks!
pub trait AsBlocks {
    /// Turns `self` into a list of `Blocks`
    fn as_blocks(&self) -> Result<Blocks, Error>;
}

pub trait AsBlock<T: Block> {
    /// Turns `self` into a `Block` of type `T`
    fn as_block(&self) -> Result<T, Error>;
}

pub trait ModalBlock {}
pub trait MessagesBlock {}
pub trait HometabBlock {}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Blocks(Vec<json::Value>);

impl Blocks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, block: impl Block) -> Result<(), Error> {
        self.0.push(block.build()?);
        Ok(())
    }

    pub fn append(&mut self, blocks: Vec<impl Block>) -> Result<(), Error> {
        for b in blocks {
            self.push(b)?;
        }
        Ok(())
    }

    pub fn split_at(&self, mid: usize) -> (Blocks, Blocks) {
        let split = self.0.split_at(mid);

        let b1 = Blocks(split.0.to_vec());
        let b2 = Blocks(split.1.to_vec());

        (b1, b2)
    }

    pub fn json(self) -> Vec<json::Value> {
        self.0
    }
}
