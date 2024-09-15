
use crate::fixtures::TestFixture;


#[test]
fn test_color_read() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    fixture.setup_colors();

    let colors = dal.color().find_all().unwrap();
    assert_eq!(colors.len(), 16);

    let blue = colors.iter().find(|c| c.name == "Blue").unwrap();
    let found_color = dal.color().find_by_id(&blue.id).unwrap();
    assert_eq!(found_color.name, "Blue");
}

#[test]
fn test_color_update() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    fixture.setup_colors();

    let colors = dal.color().find_all().unwrap();
    let red = colors.iter().find(|c| c.name == "Red").unwrap();

    let mut updated_color = red.clone();
    updated_color.name = "Crimson".to_string();
    let result = dal.color().update(&red.id, &updated_color).unwrap();
    assert_eq!(result.name, "Crimson");

    let found_color = dal.color().find_by_id(&red.id).unwrap();
    assert_eq!(found_color.name, "Crimson");
}

#[test]
fn test_color_delete() {
    let fixture = TestFixture::new();
    let dal = fixture.dal();

    fixture.setup_colors();

    let colors = dal.color().find_all().unwrap();
    let yellow = colors.iter().find(|c| c.name == "Yellow").unwrap();

    let delete_result = dal.color().delete(&yellow.id).unwrap();
    assert_eq!(delete_result, 1);

    let find_result = dal.color().find_by_id(&yellow.id);
    assert!(find_result.is_err());

    let remaining_colors = dal.color().find_all().unwrap();
    assert_eq!(remaining_colors.len(), 15);
}