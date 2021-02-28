use structlog::event::Event;


#[test]
fn test_create_event() {
    let mut ev = Event::new();
    ev.add_field(String::from("message"), String::from("Some error message"));
    assert_eq!(ev.to_string().len() > 0, true);

    let ev2 = Event::from_str("stuff");
    assert_eq!(ev2.to_string().len() > 0, true);
}
