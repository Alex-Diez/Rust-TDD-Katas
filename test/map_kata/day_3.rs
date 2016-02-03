pub use tdd_kata::map_kata::day_3::HashMap;

#[test]
fn it_should_create_a_new_empty_map() {
    let map = HashMap::new(10);

    assert!(map.is_empty());
    assert_eq!(map.size(), 0);
}

#[test]
fn it_should_increase_map_size_when_put() {
    let mut map = HashMap::new(10);
    let old_size = map.size();

    map.put(1,1);

    assert_eq!(map.size(), old_size+1);
}

#[test]
#[ignore]
fn it_should_contain_put_value() {
    let mut map = HashMap::new(10);
    map.put(1,1);

    assert!(map.contains(1));
}

#[test]
#[ignore]
fn it_should_not_increase_map_size_when_put_the_same_key() {
    let mut map = HashMap::new(10);
    map.put(1,1);
    let old_size = map.size();
    map.put(1,2);

    assert_eq!(map.size(), old_size);
}
