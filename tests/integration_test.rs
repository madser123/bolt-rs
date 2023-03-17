use slack_rs as slack;
use slack::{
    block::{self, Blocks, AsBlock, AsBlocks},
    element::{self, AsElement, AsElements},
    comp::{self, Text},
    message::Message,
};

const CHANNEL_BOT_TESTING: &str = "CHANNEL";
const SLACK_BOT_TOKEN: &str = "TOKEN";

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