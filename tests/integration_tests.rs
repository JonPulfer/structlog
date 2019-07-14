use structlog::event::Event;

#[test]
fn test_create_event() {
    let mut ev = Event::new();
    ev.add_field(String::from("message"), String::from("Some error message"));
    println!("{}", ev);
}
