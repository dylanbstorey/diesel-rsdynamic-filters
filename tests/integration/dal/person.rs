use pedal_pal::models::person::NewPerson;
use pedal_pal::models::bike::NewBike;
use pedal_pal::models::common::StringFilter;
use crate::fixtures::TestFixture;

/// Sets up the test environment with persons and bikes
fn setup() -> TestFixture {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    let alice = fixture.create_person("Alice");
    let bob = fixture.create_person("Bob");
    let charlie = fixture.create_person("Charlie");

    fixture.create_bike("Mountain Bike", Some(&alice.id), None);
    fixture.create_bike("Road Bike", Some(&bob.id), None);
    fixture.create_bike("City Bike", Some(&alice.id), None);

    fixture
}

/// Tests creating a new person
#[test]
fn test_person_create() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    let new_person = NewPerson::new("Test Person");
    let created_person = dal.person().create(&new_person).unwrap();
    assert_eq!(created_person.name, "Test Person");
}

/// Tests reading all persons from the database
#[test]
fn test_person_read() {
    let fixture = setup();
    let dal = fixture.dal();

    let persons = dal.person().find_all().unwrap();
    assert_eq!(persons.len(), 3);
}

/// Tests updating a person in the database
#[test]
fn test_person_update() {
    let fixture = setup();
    let dal = fixture.dal();

    let persons = dal.person().find_all().unwrap();
    let person_to_update = persons.first().unwrap();

    let mut updated_person = person_to_update.clone();
    updated_person.name = "Updated Name".to_string();
    let result = dal.person().update(&person_to_update.id, &updated_person).unwrap();
    assert_eq!(result.name, "Updated Name");
}

/// Tests deleting a person from the database
#[test]
fn test_person_delete() {
    let fixture = setup();
    let dal = fixture.dal();

    let persons = dal.person().find_all().unwrap();
    let person_to_delete = persons.last().unwrap();

    let delete_result = dal.person().delete(&person_to_delete.id).unwrap();
    assert_eq!(delete_result, 1);

    let remaining_persons = dal.person().find_all().unwrap();
    assert_eq!(remaining_persons.len(), 2);
}
