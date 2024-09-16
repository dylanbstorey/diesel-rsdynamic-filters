use pedal_pal::models::bike::{NewBike, Condition};
use pedal_pal::models::person::NewPerson;
use pedal_pal::models::color::NewColor;
use pedal_pal::models::common::StringFilter;
use crate::fixtures::TestFixture;

fn setup() -> TestFixture {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    // Create persons
    let alice = fixture.create_person("Alice");
    let bob = fixture.create_person("Bob");

    // Create colors
    let red = fixture.create_color("Red");
    let blue = fixture.create_color("Blue");
    let green = fixture.create_color("Green");

    // Create bikes
    fixture.create_bike("Mountain Bike", Some(&alice.id), Some(&red.id));
    fixture.create_bike("Road Bike", Some(&bob.id), Some(&blue.id));
    fixture.create_bike("City Bike", Some(&alice.id), Some(&green.id));
    fixture.create_bike("BMX Bike", None, Some(&red.id));

    fixture
}

#[test]
fn test_bike_create() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    let person = fixture.create_person("Test Person");
    let color = fixture.create_color("Test Color");

    let new_bike = NewBike::new("Test Bike", Some(&person.id), Some(&color.id));
    let created_bike = dal.bike().create(&new_bike).unwrap();

    assert_eq!(created_bike.name, "Test Bike");
    assert_eq!(created_bike.owner_id, Some(person.id));
    assert_eq!(created_bike.color_id, Some(color.id));
}

#[test]
fn test_bike_read() {
    let fixture = setup();
    let dal = fixture.dal();

    let bikes = dal.bike().find_all().unwrap();
    assert_eq!(bikes.len(), 4);

    let first_bike = bikes.first().unwrap();
    let found_bike = dal.bike().find_by_id(&first_bike.id).unwrap();
    assert_eq!(found_bike.name, first_bike.name);
}

#[test]
fn test_bike_update() {
    let fixture = setup();
    let dal = fixture.dal();

    let bikes = dal.bike().find_all().unwrap();
    let bike_to_update = bikes.first().unwrap();

    let new_person = fixture.create_person("New Owner");
    let new_color = fixture.create_color("New Color");

    let mut updated_bike = bike_to_update.clone();
    updated_bike.name = "Updated Bike Name".to_string();
    updated_bike.owner_id = Some(new_person.id.clone());
    updated_bike.color_id = Some(new_color.id.clone());

    let result = dal.bike().update(&bike_to_update.id, &updated_bike).unwrap();
    assert_eq!(result.name, "Updated Bike Name");
    assert_eq!(result.owner_id, Some(new_person.id));
    assert_eq!(result.color_id, Some(new_color.id));
}

#[test]
fn test_bike_delete() {
    let fixture = setup();
    let dal = fixture.dal();

    let bikes = dal.bike().find_all().unwrap();
    let bike_to_delete = bikes.last().unwrap();

    let delete_result = dal.bike().delete(&bike_to_delete.id).unwrap();
    assert_eq!(delete_result, 1);

    let find_result = dal.bike().find_by_id(&bike_to_delete.id);
    assert!(find_result.is_err());
}

#[test]
fn test_bike_filter_by_name() {
    let fixture = setup();
    let dal = fixture.dal();

    let conditions = vec![Condition::name(StringFilter::Equal("Mountain Bike".to_string()))];
    let bikes = dal.bike().find_with_filters(conditions).unwrap();
    assert_eq!(bikes.len(), 1);
    assert_eq!(bikes[0].name, "Mountain Bike");
}

#[test]
fn test_bike_filter_by_color() {
    let fixture = setup();
    let dal = fixture.dal();

    let conditions = vec![Condition::color(StringFilter::Equal("Red".to_string()))];
    let bikes = dal.bike().find_with_filters(conditions).unwrap();
    assert_eq!(bikes.len(), 2);
}

#[test]
fn test_bike_filter_combined() {
    let fixture = setup();
    let dal = fixture.dal();

    let conditions = vec![
        Condition::name(StringFilter::Equal("City Bike".to_string())),
        Condition::color(StringFilter::Equal("Green".to_string())),
    ];
    let bikes = dal.bike().find_with_filters(conditions).unwrap();
    assert_eq!(bikes.len(), 1);
    assert_eq!(bikes[0].name, "City Bike");
}

#[test]
fn test_bike_filter_no_results() {
    let fixture = setup();
    let dal = fixture.dal();

    let conditions = vec![Condition::name(StringFilter::Equal("Nonexistent Bike".to_string()))];
    let bikes = dal.bike().find_with_filters(conditions).unwrap();
    assert_eq!(bikes.len(), 0);
}

#[test]
fn test_bike_filter_like() {
    let fixture = setup();
    let dal = fixture.dal();

    let conditions = vec![Condition::name(StringFilter::Like("%Bike%".to_string()))];
    let bikes = dal.bike().find_with_filters(conditions).unwrap();
    assert_eq!(bikes.len(), 4);
}

#[test]
fn test_bike_filter_in() {
    let fixture = setup();
    let dal = fixture.dal();

    let conditions = vec![Condition::name(StringFilter::In(vec![
        "Mountain Bike".to_string(),
        "Road Bike".to_string(),
    ]))];
    let bikes = dal.bike().find_with_filters(conditions).unwrap();
    assert_eq!(bikes.len(), 2);
}

#[test]
fn test_bike_filter_no_owner() {
    let fixture = setup();
    let dal = fixture.dal();

    let conditions = vec![Condition::name(StringFilter::Equal("BMX Bike".to_string()))];
    let bikes = dal.bike().find_with_filters(conditions).unwrap();
    assert_eq!(bikes.len(), 1);
    assert_eq!(bikes[0].name, "BMX Bike");
    assert_eq!(bikes[0].owner_id, None);
}