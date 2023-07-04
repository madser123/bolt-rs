use super::*;

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

/// Converts any type into blocks
pub trait AsBlocks {
    /// Turns `self` into a list of `Blocks`
    fn as_blocks(&self) -> BoltResult<Blocks>;
}

/// Converts any type into a single block
pub trait AsBlock<B: Block> {
    /// Turns `self` into a `Block` of type `B`
    fn as_block(&self) -> BoltResult<B>;
}

trait ModalBlock {}
trait MessagesBlock {}
trait HometabBlock {}


/// A list of blocks as json-strings.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Blocks(Vec<json::Value>);

impl Blocks {
    /// Creates a new empty list of blocks.
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets a list of block-ID's (if present) from this block-collection.
    /// If a block doesn't have an id, it simply won't be in the list.
    /// Blocks returned from slack always has ID's. Only user created-blocks that
    /// haven't been sent to slack might not have ID's.
    pub fn ids(&self) -> Vec<String> {
        let mut ids = Vec::new();
        for b in &self.0 {
            if let Some(id) = b.get("block_id") {
                // Remove escaped `"` from string value and push to vec
                ids.push(id.to_string().replace(['\\', '"'], ""));
            }
        }
        ids
    }

    /// Returns the amount of blocks in this list.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if no blocks is in the list.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Pushes a new block to the top of the list.
    pub fn push(&mut self, block: impl Block) -> BoltResult<()> {
        self.0.push(block.build()?);
        Ok(())
    }

    /// Appends blocks to the end of the list.
    pub fn append(&mut self, blocks: Vec<impl Block>) -> BoltResult<()> {
        for b in blocks {
            self.push(b)?;
        }
        Ok(())
    }

    /// Splits the blocks into two elements at the chosen index.
    pub fn split_at(&self, mid: usize) -> (Blocks, Blocks) {
        let split = self.0.split_at(mid);

        let b1 = Blocks(split.0.to_vec());
        let b2 = Blocks(split.1.to_vec());

        (b1, b2)
    }

    /// Returns the blocklist as a vec of [`serde_json::Value`].
    pub fn json_vec(self) -> Vec<json::Value> {
        self.0
    }

    /// Returns the blocklist as a prettified JSON-[String]
    pub fn json_string(self) -> BoltResult<String> {
        let json = match json::to_value(self.0) {
            Ok(json) => json,
            Err(error) => return Err(Error::Parsing("Blocks".to_string(), error.to_string()))
        };
        match json::to_string_pretty(&json) {
            Ok(json) => Ok(json),
            Err(error) => Err(Error::Parsing("Blocks".to_string(), error.to_string()))
        }
    }
}

