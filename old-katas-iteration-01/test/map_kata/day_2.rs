pub use tdd_kata::map_kata::day_2::Map;

#[test]
fn it_should_create_an_empty_map() {
    let map = Map::new(10);

    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}

#[test]
fn it_should_increase_map_size_when_put() {
    let mut map = Map::new(10);
    let old_size = map.len();

    map.put(1, 1);

    assert_eq!(map.len(), old_size + 1);
}

#[test]
fn it_should_decrease_map_size_when_remove() {
    let mut map = Map::new(10);
    map.put(1, 1);
    let old_size = map.len();

    map.remove(1);

    assert_eq!(map.len(), old_size - 1);
}

#[test]
#[ignore]
fn it_should_not_increase_map_size_when_put_same_key() {
    let mut map = Map::new(10);
    map.put(1, 1);
    let old_size = map.len();

    map.put(1, 2);

    assert_eq!(map.len(), old_size);
}
