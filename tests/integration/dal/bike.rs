use pedal_pal::models::bike::NewBike;
use pedal_pal::models::person::NewPerson;
use pedal_pal::models::color::NewColor;
use pedal_pal::models::common::StringFilter;
use crate::fixtures::TestFixture;

fn setup() -> (TestFixture, NewBike) {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    let p = NewPerson::new("Bike Owner");
    let c = NewColor::new("Red");

    let person = dal.person().create(&p).unwrap();
    let color = dal.color().create(&c).unwrap();

    let new_bike = NewBike::new("Mountain Bike", Some(&person.id), Some(&color.id));
    
    (fixture, new_bike)
}

#[test]
fn test_bike_create() {
    let (fixture, new_bike) = setup();
    let dal = fixture.dal();

    let created_bike = dal.bike().create(&new_bike).unwrap();
    assert_eq!(created_bike.name, "Mountain Bike");
}

#[test]
fn test_bike_read() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    fixture.setup_bikes();

    let bikes = dal.bike().find_all().unwrap();
    assert_eq!(bikes.len(), 3);

    let first_bike = bikes.first().unwrap();
    let found_bike = dal.bike().find_by_id(&first_bike.id).unwrap();
    assert_eq!(found_bike.name, first_bike.name);
}

#[test]
fn test_bike_update() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    fixture.setup_bikes();

    let bikes = dal.bike().find_all().unwrap();
    let bike_to_update = bikes.first().unwrap();

    let mut updated_bike = bike_to_update.clone();
    updated_bike.name = "Updated Bike Name".to_string();
    let result = dal.bike().update(&bike_to_update.id, &updated_bike).unwrap();
    assert_eq!(result.name, "Updated Bike Name");

    let found_bike = dal.bike().find_by_id(&bike_to_update.id).unwrap();
    assert_eq!(found_bike.name, "Updated Bike Name");
}

#[test]
fn test_bike_delete() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    fixture.setup_bikes();

    let bikes = dal.bike().find_all().unwrap();
    let bike_to_delete = bikes.last().unwrap();

    let delete_result = dal.bike().delete(&bike_to_delete.id).unwrap();
    assert_eq!(delete_result, 1);

    let find_result = dal.bike().find_by_id(&bike_to_delete.id);
    assert!(find_result.is_err());

    let remaining_bikes = dal.bike().find_all().unwrap();
    assert_eq!(remaining_bikes.len(), 2);
}

#[test]
fn test_bike_filter() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    fixture.setup_bikes();

    // Filter by color
    let red_bikes = dal.bike().find_with_filters(
        Some(StringFilter::Equal("Red".to_string())),
        None
    ).unwrap();
    assert!(red_bikes.iter().all(|b| b.color_id == Some("Red".to_string())));

    // Filter by owner
    let alice_bikes = dal.bike().find_with_filters(
        None,
        Some(StringFilter::Equal("Alice".to_string()))
    ).unwrap();
    assert!(alice_bikes.iter().all(|b| b.owner_id == Some("Alice".to_string())));

    // Filter by both color and owner
    let alice_red_bikes = dal.bike().find_with_filters(
        Some(StringFilter::Equal("Red".to_string())),
        Some(StringFilter::Equal("Alice".to_string()))
    ).unwrap();
    assert!(alice_red_bikes.iter().all(|b| b.color_id == Some("Red".to_string()) && b.owner_id == Some("Alice".to_string())));
}