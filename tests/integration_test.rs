use std::env::var;

use bolt_rs as slack;
use slack::{
    block::{self, Blocks, AsBlock, AsBlocks},
    element::{self, AsElement, AsElements},
    comp::{self, Text},
    message::Message,
};

const TEST_CHANNEL: &str = "CHANNEL";

#[tokio::test]
async fn create_app() {
    todo!()
}
