use hermes::{Callback, EndSessionMessage, HermesProtocolHandler, NluIntentMessage};

use hermes_mqtt::MqttHermesProtocolHandler;

use snips_nlu_ontology::SlotValue;

use websocket::client::sync::Client;
use websocket::sync::Server;
use websocket::OwnedMessage;

use std::sync::{Arc, Mutex};

struct PresServer {
    pub clients: Vec<Client<std::net::TcpStream>>,
    protocol_handler: Box<dyn HermesProtocolHandler>,
}

fn generate_message_from_intent(message: &NluIntentMessage) -> String {
    match message.intent.intent_name.as_str() {
        "Deluvi:diapoNextSlide" => "NextSlide".to_string(),
        "Deluvi:diapoPreviousSlide" => "PreviousSlide".to_string(),
        "Deluvi:diapoGoNumber" => {
            let number = match message.slots[0].nlu_slot.value {
                SlotValue::Number(value) => value.value as u32,
                _ => panic!("Expected only a number on this slot"),
            };
            format!("GoToSlide({})", number)
        }
        _ => "unknown".to_string(),
    }
}

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();

    let hermes_client = MqttHermesProtocolHandler::new("127.0.0.1").unwrap();

    let pres_server = PresServer {
        clients: Vec::new(),
        protocol_handler: Box::new(hermes_client),
    };

    let pres_server = Arc::new(Mutex::new(pres_server));

    {
        let pres_server_copy = Arc::clone(&pres_server);
        let pres_server = pres_server.lock().unwrap();
        pres_server
            .protocol_handler
            .nlu()
            .subscribe_intent_parsed(Callback::<NluIntentMessage>::new(
                move |intent: &NluIntentMessage| {
                    let mut pres_server = pres_server_copy.lock().unwrap();
                    let message = OwnedMessage::Text(generate_message_from_intent(intent));

                    let mut to_remove: Vec<usize> = Vec::new();

                    for (client, i) in pres_server.clients.iter_mut().zip(0..) {
                        if let Err(e) = client.send_message(&message) {
                            println!("Client removed because of error {}", e);
                            to_remove.push(i);
                        }
                    }

                    to_remove.reverse();
                    for i in to_remove {
                        pres_server.clients.remove(i);
                    }

                    if let Some(session_id) = &intent.session_id {
                        pres_server
                            .protocol_handler
                            .dialogue()
                            .publish_end_session(EndSessionMessage {
                                session_id: session_id.to_string(),
                                text: None,
                            })
                            .unwrap();
                    }
                },
            ))
            .unwrap();
    }

    for request in server.filter_map(Result::ok) {
        if !request
            .protocols()
            .contains(&"snips-presentation-assistant".to_string())
        {
            println!("Rejected request");
            request.reject().unwrap();
            continue;
        }

        let client = request
            .use_protocol("snips-presentation-assistant")
            .accept()
            .unwrap();

        let ip = client.peer_addr().unwrap();

        println!("Connection accepted from {}", ip);

        {
            let mut pres_server = pres_server.lock().unwrap();
            pres_server.clients.push(client);
        }
    }
}
