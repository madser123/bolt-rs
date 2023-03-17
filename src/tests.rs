#[cfg(test)]
mod test {
    use crate::{*, core::Build};
    use std::vec;
    use serde_json::json;

    use block::{Blocks, AsBlock, AsBlocks};
    use message::MessageAble;
    use comp::Text;

    #[derive(Clone)]
    pub struct TestUser {
        id: i64,
        name: &'static str,
        url: &'static str,
    }
    pub struct TestList {
        name: &'static str,
        users: Vec<TestUser>
    }

    impl AsBlocks for TestList {
        fn as_blocks(&self) -> Result<Blocks, Error> {
            let mut blocks = Blocks::new();
            blocks.push(block::Header::new(Text::plain(self.name)))?;
            for u in &self.users {
                blocks.push(u.as_block()?)?;
            }
            Ok(blocks)
        }
    }

    impl AsBlock<block::Section> for TestUser {
        fn as_block(&self) -> Result<block::Section, Error> {
            Ok(block::Section::new()
                .field(Text::mrkdwn(&format!("- Id: {}", self.id)).into())
                .field(Text::mrkdwn(&format!("- Name: {}", self.name)).into())
                .field(Text::mrkdwn(&format!("- Webpage: {}", self.url)).into())
            )
        }
    }

    impl MessageAble for TestList {
        fn into_message(self) -> Result<message::Message, Error> {
            Ok(message::Message::new()
                .channel("XXXXXX")
                .text("Test")
                .blocks(self.as_blocks()?)
            )
        }
    }


    fn get_test_types() -> (TestUser, TestUser, TestList) {
        let user1 = TestUser {
            id: 420,
            name: "Brugernavnmedæøå",
            url: "https://minhjemmeside.dk",
        };
        let user2 = TestUser {
            id: 69,
            name: "XxX_UsEr_xXX",
            url: "https://dinmor.dk",
        };
        let list = TestList {
            name: "Userlist #1",
            users: vec![user1.clone(), user2.clone()],
        };
        (user1, user2, list)
    }

    #[test]
    fn element_test() {
        let button = element::Button::new(Text::plain("Yeet"), "action_1");
        let _block = block::Section::new()
            .accessory(button)
            .unwrap();
    }

    #[test]
    fn section_block_full() {
        let t = block::Section::new()
            .text(Text::plain("Test").into())
            .id("section_01")
            .fields(vec![
                Text::plain("Field1").into(),
                Text::plain("Field2").emoji().into(),
            ])
            .build()
            .unwrap();

        assert_eq!(
            t,
            json!({
                "type": "section",
                "text": {
                    "type": "plain_text",
                    "text": "Test",
                    "emoji": false
                },
                "block_id": "section_01",
                "fields": [
                    {
                        "type": "plain_text",
                        "text": "Field1",
                        "emoji": false
                    },
                    {
                        "type": "plain_text",
                        "text": "Field2",
                        "emoji": true
                    }
                ]
            })
        )
    }

    #[test]
    fn user1() {
        let t = get_test_types().0.as_block().unwrap().build().unwrap();
        println!("{t}");
        assert_eq!(
            t,
            json!({
                "type": "section",
                "fields": [
                    {
                        "type": "mrkdwn",
                        "text": "- Id: 420",
                        "verbatim": false
                    },
                    {
                        "type": "mrkdwn",
                        "text": "- Name: Brugernavnmedæøå",
                        "verbatim": false
                    },
                    {
                        "type": "mrkdwn",
                        "text": "- Webpage: https://minhjemmeside.dk",
                        "verbatim": false
                    }
                ]
            })
        )
    }

    #[test]
    fn user2() {
        let t = get_test_types().1.as_block().unwrap().build().unwrap();
        println!("{t}");
        assert_eq!(
            t,
            json!({
                "type": "section",
                "fields": [
                    {
                        "type": "mrkdwn",
                        "text": "- Id: 69",
                        "verbatim": false
                    },
                    {
                        "type": "mrkdwn",
                        "text": "- Name: XxX_UsEr_xXX",
                        "verbatim": false
                    },
                    {
                        "type": "mrkdwn",
                        "text": "- Webpage: https://dinmor.dk",
                        "verbatim": false
                    }
                ]
            })
        )
    }

    #[test]
    fn messageable_simple() {
        let t = serde_json::to_value(get_test_types().2.into_message().unwrap()).unwrap();
        println!("{t}");
        assert_eq!(
            t,
            json!({
                "channel": "XXXXXX",
                "blocks": [
                    {
                        "type": "header",
                        "text": {
                            "type": "plain_text",
                            "text": "Userlist #1",
                            "emoji": false,
                        },
                    },
                    {
                        "type": "section",
                        "fields": [
                            {
                                "type": "mrkdwn",
                                "text": "- Id: 420",
                                "verbatim": false,
                            },
                            {
                                "type": "mrkdwn",
                                "text": "- Name: Brugernavnmedæøå",
                                "verbatim": false,
                            },
                            {
                                "type": "mrkdwn",
                                "text": "- Webpage: https://minhjemmeside.dk",
                                "verbatim": false,
                            }
                        ]
                    },
                    {
                        "type": "section",
                        "fields": [
                            {
                                "type": "mrkdwn",
                                "text": "- Id: 69",
                                "verbatim": false,
                            },
                            {
                                "type": "mrkdwn",
                                "text": "- Name: XxX_UsEr_xXX",
                                "verbatim": false,
                            },
                            {
                                "type": "mrkdwn",
                                "text": "- Webpage: https://dinmor.dk",
                                "verbatim": false,
                            }
                        ]
                    }
                ],
                "text": "Test",
            })
        )
    }
}
