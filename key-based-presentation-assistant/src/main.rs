use hermes::{Callback, EndSessionMessage, HermesProtocolHandler, NluIntentMessage};

use hermes_mqtt::MqttHermesProtocolHandler;

use snips_nlu_ontology::SlotValue;

use autopilot::key::{tap, Code, KeyCode};

fn generate_message_from_intent(message: &NluIntentMessage) -> Option<String> {
    match message.intent.intent_name.as_str() {
        "Deluvi:diapoNextSlide" => Some("NextSlide".to_string()),
        "Deluvi:diapoPreviousSlide" => Some("PreviousSlide".to_string()),
        "Deluvi:diapoGoNumber" => message.slots.get(0).and_then(|slot| {
            let number = match slot.nlu_slot.value {
                SlotValue::Number(value) => value.value as u32,
                _ => return None,
            };

            Some(format!("GoToSlide({})", number))
        }),
        _ => None,
    }
}

fn main() {
    let hermes_client = MqttHermesProtocolHandler::new("127.0.0.1").unwrap();

    hermes_client
        .nlu()
        .subscribe_intent_parsed(Callback::<NluIntentMessage>::new(
            move |intent: &NluIntentMessage| {
                if let Some(message) = generate_message_from_intent(intent) {
                    let action_performed = match message.as_str() {
                        "NextSlide" => {
                            tap(&Code(KeyCode::RightArrow), &[], 50);
                            true
                        }
                        "PreviousSlide" => {
                            tap(&Code(KeyCode::LeftArrow), &[], 50);
                            true
                        }
                        _ => false,
                    };

                    if let Some(session_id) = &intent.session_id {
                        if action_performed {
                            hermes_client
                                .dialogue()
                                .publish_end_session(EndSessionMessage {
                                    session_id: session_id.to_string(),
                                    text: None,
                                })
                                .unwrap();
                        }
                    }
                } else {
                    println!("Received a weird formatted intent : {:?}", intent);
                }
            },
        ))
        .unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
