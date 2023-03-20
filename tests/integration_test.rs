use std::env::var;

use bolt_rs as slack;
use slack::{
    block::{self, Blocks, AsBlock, AsBlocks},
    element::{self, AsElement, AsElements},
    comp::{self, Text},
    message::Message,
};

const TEST_CHANNEL: &str = "CHANNEL";

#[test]
#[should_panic]
fn section_too_many_fields_failure() {
    let mut blocks = Blocks::new();

    let result = blocks.push(block::Section::new()
        .field(Text::mrkdwn("*Bold text*").into())
        .field(Text::mrkdwn("*Bold text*").into())
        .field(Text::mrkdwn("*Bold text*").into())
        .field(Text::mrkdwn("*Bold text*").into())
        .field(Text::mrkdwn("*Bold text*").into())
        .field(Text::mrkdwn("*Bold text*").into())
        .field(Text::mrkdwn("*Bold text*").into())
        .field(Text::mrkdwn("*Bold text*").into())
        .field(Text::mrkdwn("*Bold text*").into())
        .field(Text::mrkdwn("*Bold text*").into())
        .field(Text::mrkdwn("1 too many fields :))").into())
    );

    println!("{result:?}");
    result.unwrap();
}

fn create_app() {
    let bot_token = var("BOT_TOKEN").unwrap();
}
