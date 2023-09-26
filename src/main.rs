use spaceapi_server::api::{State, StatusBuilder, Location, Contact, Feeds, Feed, IssueReportChannel};
use spaceapi_server::api::sensors::{PeopleNowPresentSensorTemplate, SensorMetadata};
use spaceapi_server::modifiers::StateFromPeopleNowPresent;
use spaceapi_server::SpaceapiServerBuilder;

fn main() {
    let status = StatusBuilder::mixed("section77")
        .logo("https://section77.de/static/section77_logo_vector.svg")
        .url("https://section77.de/")
        .location(Location {
            address: Some("Hauptstraße 1, 77652 Offenburg, Germany".into()),
            lat: 48.4771,
            lon: 7.9461,
            timezone: None,
        })
        .contact(Contact {
            email: Some("info@section77.de".into()),
            twitter: Some("@section77de".into()),
            mastodon: Some("@section77@chaos.social".into()),
            ..Default::default()
        })
        .feeds(Feeds {
            calendar: Some(Feed {
                type_: Some("ical".into()),
                url: "https://section77.de/section77.ics".to_string(),
            }.into()),
            ..Default::default()
        })
        .add_issue_report_channel(IssueReportChannel::Email)
        .add_extension("ccc", "chaostreff")
        .state(State::default())
        .build()
        .expect("Creating status failed");

    let server = SpaceapiServerBuilder::new(status)
        .redis_connection_info("redis://127.0.0.1/")
        .add_sensor(
            PeopleNowPresentSensorTemplate {
                metadata: SensorMetadata {
                    location: Some("Gleis0".into()),
                    ..Default::default()
                },
            },
            "people_now_present".into(),
        )
        .add_status_modifier(StateFromPeopleNowPresent)
        .build()
        .unwrap();

    let _ = server.serve("127.0.0.1:8000");
}
