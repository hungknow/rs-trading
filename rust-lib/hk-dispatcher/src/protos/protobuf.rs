impl From<::protobuf::Message> for Vec<u8> {
    fn from(message: ::protobuf::Message) -> Self {
        let mut buf = Vec::new();
        message.write_to_vec(&mut buf).unwrap();
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protobuf_message_to_vec() {
        let mut request = FFIRequest::new();
        request.set_event("test".to_string());
        request.set_payload(vec![1, 2, 3, 4]);

        let bytes: Vec<u8> = request.into();
        let parsed_request = FFIRequest::parse_from_bytes(&bytes).unwrap();

        assert_eq!(parsed_request.get_event(), "test");
        assert_eq!(parsed_request.get_payload(), vec![1, 2, 3, 4]);
    }
}
