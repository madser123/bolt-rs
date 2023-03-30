#[cfg(test)]
mod test {
    use crate::pre::*;
    use block::{AsBlock, AsBlocks, Blocks};
    use comp::Text;
    use message::{AsMessage, Message};

    use json::json;
    

    #[derive(Clone)]
    pub struct TestUser {
        id: i64,
        name: &'static str,
        url: &'static str,
    }
    pub struct TestList {
        name: &'static str,
        users: Vec<TestUser>,
    }

    impl AsBlocks for TestList {
        fn as_blocks(&self) -> BoltResult<Blocks> {
            let mut blocks = Blocks::new();

            blocks.push(block::Header::new(Text::plain(self.name)))?;

            for u in &self.users {
                blocks.push(u.as_block()?)?;
            }

            Ok(blocks)
        }
    }

    impl AsBlock<block::Section> for TestUser {
        fn as_block(&self) -> BoltResult<block::Section> {
            Ok(block::Section::new()
                .field(Text::mrkdwn(&format!("- Id: {}", self.id)).into())
                .field(Text::mrkdwn(&format!("- Name: {}", self.name)).into())
                .field(Text::mrkdwn(&format!("- Webpage: {}", self.url)).into())
            )
        }
    }

    impl AsMessage for TestList {
        fn as_message(&self) -> BoltResult<Message> {
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
            name: "WeirdUsernameæøå",
            url: "https://mywebpage.com",
        };
        let user2 = TestUser {
            id: 69,
            name: "XxX_UsEr_xXX",
            url: "https://otherpage.com",
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
        let _block = block::Section::new().accessory(button).unwrap();
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
                        "text": "- Name: WeirdUsernameæøå",
                        "verbatim": false
                    },
                    {
                        "type": "mrkdwn",
                        "text": "- Webpage: https://mywebpage.com",
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
                        "text": "- Webpage: https://otherpage.com",
                        "verbatim": false
                    }
                ]
            })
        )
    }

    #[test]
    fn messageable_simple() {
        let t = serde_json::to_value(get_test_types().2.as_message().unwrap()).unwrap();
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
                                "text": "- Name: WeirdUsernameæøå",
                                "verbatim": false,
                            },
                            {
                                "type": "mrkdwn",
                                "text": "- Webpage: https://mywebpage.com",
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
                                "text": "- Webpage: https://otherpage.com",
                                "verbatim": false,
                            }
                        ]
                    }
                ],
                "text": "Test",
            })
        )
    }

    #[test]
    fn block_ids() {
        let mut blocks = Blocks::new();

        blocks.push(block::Actions::new().id("Action1")).unwrap();
        blocks.push(block::Divider::new()).unwrap();
        blocks.push(block::Section::new().id("Section1")).unwrap();

        assert_eq!(blocks.ids(), vec!["Action1".to_string(), "Section1".to_string()])
    }
}
