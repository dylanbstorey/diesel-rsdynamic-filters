use pedal_pal::models::{
    bike_trip::NewBikeTrip,
    bike::NewBike,
    person::NewPerson,
    color::NewColor,
};
use crate::fixtures::TestFixture;

fn setup() -> (TestFixture, NewBikeTrip) {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    // Create a person, color, and bike for the bike trip
    let new_person = NewPerson::new("Trip Taker");
    let person = dal.person().create(&new_person).unwrap();
    let new_color = NewColor::new("Green");
    let color = dal.color().create(&new_color).unwrap();
    let new_bike = NewBike::new("City Bike", Some(&person.id), Some(&color.id));
    let bike = dal.bike().create(&new_bike).unwrap();

    let new_bike_trip = NewBikeTrip::new("City Tour", Some(&bike.id));
    
    (fixture, new_bike_trip)
}

#[test]
fn test_bike_trip_create() {
    let (fixture, new_bike_trip) = setup();
    let dal = fixture.dal();

    let created_bike_trip = dal.bike_trip().create(&new_bike_trip).unwrap();
    assert_eq!(created_bike_trip.name, "City Tour");
}

#[test]
fn test_bike_trip_read() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    fixture.setup_bike_trips();

    let bike_trips = dal.bike_trip().find_all().unwrap();
    assert_eq!(bike_trips.len(), 5);

    let first_trip = bike_trips.first().unwrap();
    let found_trip = dal.bike_trip().find_by_id(&first_trip.id).unwrap();
    assert_eq!(found_trip.id, first_trip.id);
}

#[test]
fn test_bike_trip_update() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    fixture.setup_bike_trips();

    let bike_trips = dal.bike_trip().find_all().unwrap();
    let trip_to_update = bike_trips.first().unwrap();

    let mut updated_trip = trip_to_update.clone();
    updated_trip.name = "Updated Trip Name".to_string();
    let result = dal.bike_trip().update(&trip_to_update.id, &updated_trip).unwrap();
    assert_eq!(result.name, "Updated Trip Name");

    let found_trip = dal.bike_trip().find_by_id(&trip_to_update.id).unwrap();
    assert_eq!(found_trip.name, "Updated Trip Name");
}

#[test]
fn test_bike_trip_delete() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    fixture.setup_bike_trips();

    let bike_trips = dal.bike_trip().find_all().unwrap();
    let trip_to_delete = bike_trips.last().unwrap();

    let delete_result = dal.bike_trip().delete(&trip_to_delete.id).unwrap();
    assert_eq!(delete_result, 1);

    let find_result = dal.bike_trip().find_by_id(&trip_to_delete.id);
    assert!(find_result.is_err());

    let remaining_trips = dal.bike_trip().find_all().unwrap();
    assert_eq!(remaining_trips.len(), 4);
}