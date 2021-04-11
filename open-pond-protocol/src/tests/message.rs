#[cfg(test)]
use crate::message::{MAX_PACKET_SIZE, MAX_PAYLOAD_SIZE, MIN_PACKET_SIZE};
#[cfg(test)]
use crate::Message;

#[test]
fn new_message_success() {
    let payload = vec![1, 2, 3];
    let id = 4;

    let message = Message::new(id, payload.clone()).unwrap();

    assert_eq!(message.id, id);
    assert_eq!(message.payload, payload);
    assert_eq!(message.length, payload.len() as u16);
}

#[test]
fn new_message_payload_too_large() {
    let size = MAX_PAYLOAD_SIZE + 1;
    let payload = vec![0; size];
    let id = 4;

    let result = Message::new(id, payload.clone());

    assert!(result.is_err());
}

#[test]
fn from_bytes_message_success() {
    let raw = vec![
        1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];
    let expected_id = 1;
    let expected_length = 1;
    let expected_payload = vec![1];

    let message = Message::from_bytes(raw).unwrap();

    assert_eq!(message.id, expected_id);
    assert_eq!(message.length, expected_length);
    assert_eq!(message.payload, expected_payload);
}

#[test]
fn from_bytes_minimum_message_success() {
    let raw = vec![0; MIN_PACKET_SIZE];

    let result = Message::from_bytes(raw);

    assert!(result.is_ok());
}

#[test]
fn from_bytes_maximum_message_success() {
    let raw = vec![0; MAX_PACKET_SIZE];

    let result = Message::from_bytes(raw);

    assert!(result.is_ok());
}

#[test]
fn from_bytes_message_vector_too_large() {
    let size = MAX_PACKET_SIZE + 1;
    let raw = vec![0; size];

    let result = Message::from_bytes(raw);

    assert!(result.is_err());
}

#[test]
fn from_bytes_message_vector_too_small() {
    let size = MIN_PACKET_SIZE - 1;
    let raw = vec![0; size];

    let result = Message::from_bytes(raw);

    assert!(result.is_err());
}

#[test]
fn as_bytes_message_success() {
    let message = Message {
        id: 1,
        flags: 128,
        index: 0,
        class: 0,
        port: 256,
        length: 1,
        signature: vec![0; 32],
        payload: vec![1],
    };

    let expected = vec![
        1, 128, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];

    let raw = message.as_bytes().unwrap();

    assert_eq!(raw, expected);
}

#[test]
fn as_bytes_minimum_message_success() {
    let message = Message {
        id: 1,
        flags: 128,
        index: 0,
        class: 0,
        port: 256,
        length: 0,
        signature: vec![0; 32],
        payload: vec![],
    };

    let expected = vec![
        1, 128, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let raw = message.as_bytes().unwrap();

    assert_eq!(raw, expected);
}
