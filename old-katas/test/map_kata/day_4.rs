pub use tdd_kata::map_kata::day_4::Map;

#[test]
fn it_should_create_a_new_empty_map() {
    let map = Map::new(10);

    assert!(map.is_empty());
    assert_eq!(map.size(), 0);
}

#[test]
fn it_should_increase_size_when_put_value() {
    let mut map = Map::new(10);
    let old_size = map.size();
    map.put(1,1);

    assert_eq!(map.size(), old_size+1);
}

#[test]
fn it_should_contain_put_value() {
    let mut map = Map::new(10);
    map.put(1,1);

    assert!(map.contains(1));
    assert!(!map.contains(2));
}

#[test]
fn it_should_not_increase_size_if_it_already_in_map() {
    let mut map = Map::new(10);
    map.put(1,1);
    let old_size = map.size();
    map.put(1,2);

    assert_eq!(map.size(), old_size);
}

#[test]
#[ignore]
fn it_should_get_put_value() {
    let mut map = Map::new(10);
    map.put(1,1);

    assert_eq!(map.get(1), Some(1));
}
