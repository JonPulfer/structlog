pub mod event;

#[cfg(test)]
mod test {
    use super::event;

    #[test]
    fn test_simple_event() {
        let mut test_event = event::Event::new();
        test_event.add_field(
            String::from("message"),
            String::from("this is the error message"),
        );
        let expected_output = format!(
            "{{\"attributes\":{{\"message\":\"this is the error message\"}},\"created\":{},\"level\":\"INFO\",\"severity\":\"INFO\",\"caller\":\"src/lib.rs:9:30\"}}",
            serde_json::to_string(&test_event.created).unwrap()
        );
        assert_eq!(&test_event.to_string(), &expected_output);
    }

    #[test]
    fn test_event_parse_from_string() {
        let test_event = event::Event::from_str("some event");
        println!("{}", test_event);
    }

    #[test]
    fn test_make_error() {
        let mut ev = event::Event::from_str("something bad happened");
        assert_eq!(ev.error().to_string().contains("ERROR"), true);
        assert_eq!(ev.to_string().contains("src/lib.rs"), true);
    }
}
