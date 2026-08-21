// This test suite is AI-generated (I suck at writing tests - the slot map itself was written by
// me). It was made using Claude Sonnet 5.

use dream_studio_v1::data::slotmap::{Key, SlotMap};

#[test]
fn insert_then_get_returns_value() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let key = map.insert(42);
    assert_eq!(map.get(key), Some(&42));
}

#[test]
fn insert_multiple_values_are_independently_retrievable() {
    let mut map: SlotMap<&str> = SlotMap::default();
    let a = map.insert("a");
    let b = map.insert("b");
    let c = map.insert("c");

    assert_eq!(map.get(a), Some(&"a"));
    assert_eq!(map.get(b), Some(&"b"));
    assert_eq!(map.get(c), Some(&"c"));
}

#[test]
fn insert_returns_distinct_keys() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let a = map.insert(1);
    let b = map.insert(2);
    assert_ne!(a, b);
}

#[test]
fn get_mut_allows_mutation() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let key = map.insert(10);
    if let Some(v) = map.get_mut(key) {
        *v += 5;
    }
    assert_eq!(map.get(key), Some(&15));
}

#[test]
fn get_mut_on_invalid_key_returns_none() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let key = map.insert(1);
    map.remove(key);
    assert_eq!(map.get_mut(key), None);
}

#[test]
fn remove_returns_the_value() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let key = map.insert(99);
    assert_eq!(map.remove(key), Some(99));
}

#[test]
fn get_after_remove_returns_none() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let key = map.insert(1);
    map.remove(key);
    assert_eq!(map.get(key), None);
}

#[test]
fn double_remove_returns_none_second_time() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let key = map.insert(1);
    assert_eq!(map.remove(key), Some(1));
    assert_eq!(map.remove(key), None);
}

#[test]
fn removing_one_key_does_not_affect_others() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let a = map.insert(1);
    let b = map.insert(2);
    let c = map.insert(3);

    map.remove(b);

    assert_eq!(map.get(a), Some(&1));
    assert_eq!(map.get(b), None);
    assert_eq!(map.get(c), Some(&3));
}

#[test]
fn stale_key_is_invalidated_after_slot_reuse() {
    let mut map: SlotMap<&str> = SlotMap::default();

    let key1 = map.insert("first");
    map.remove(key1);
    let key2 = map.insert("second");

    assert_eq!(map.get(key1), None);
    assert_eq!(map.get(key2), Some(&"second"));
}

#[test]
fn many_insert_remove_cycles_never_let_stale_key_resolve() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let mut stale_keys = Vec::new();

    let mut key = map.insert(0);
    for i in 1..1000 {
        stale_keys.push(key);
        map.remove(key);
        key = map.insert(i);
    }

    for stale in stale_keys {
        assert_eq!(map.get(stale), None);
    }
    assert!(map.get(key).is_some());
}

#[test]
fn key_is_copy_and_clone() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let key = map.insert(1);
    let copied = key;
    let cloned = key.clone();
    assert_eq!(key, copied);
    assert_eq!(key, cloned);
    assert_eq!(map.get(key), Some(&1));
}

#[test]
fn key_equality_is_reflexive_and_distinguishes_different_slots() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let a = map.insert(1);
    let b = map.insert(2);

    assert_eq!(a, a);
    assert_eq!(b, b);
    assert_ne!(a, b);
}

#[test]
fn key_can_be_used_as_hashmap_key() {
    use std::collections::HashMap;

    let mut map: SlotMap<i32> = SlotMap::default();
    let a = map.insert(1);
    let b = map.insert(2);

    let mut side_table: HashMap<Key, &str> = HashMap::new();
    side_table.insert(a, "alpha");
    side_table.insert(b, "beta");

    assert_eq!(side_table.get(&a), Some(&"alpha"));
    assert_eq!(side_table.get(&b), Some(&"beta"));
}

#[test]
fn insert_after_multiple_removes_reuses_slots_without_corruption() {
    let mut map: SlotMap<i32> = SlotMap::default();
    let keys: Vec<Key> = (0..10).map(|i| map.insert(i)).collect();

    for (i, &k) in keys.iter().enumerate() {
        if i % 2 == 0 {
            map.remove(k);
        }
    }

    let new_keys: Vec<Key> = (100..105).map(|i| map.insert(i)).collect();

    for (i, &k) in keys.iter().enumerate() {
        if i % 2 == 1 {
            assert_eq!(map.get(k), Some(&(i as i32)));
        } else {
            assert_eq!(map.get(k), None);
        }
    }

    for (i, &k) in new_keys.iter().enumerate() {
        assert_eq!(map.get(k), Some(&(100 + i as i32)));
    }
}

#[test]
fn works_with_non_copy_owned_values() {
    let mut map: SlotMap<String> = SlotMap::default();
    let key = map.insert(String::from("hello"));

    assert_eq!(map.get(key).map(String::as_str), Some("hello"));

    let removed = map.remove(key);
    assert_eq!(removed, Some(String::from("hello")));
}

#[test]
fn remove_actually_drops_value_no_double_free_or_leak_flag() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let mut map: SlotMap<Rc<RefCell<i32>>> = SlotMap::default();
    let payload = Rc::new(RefCell::new(0));
    let key = map.insert(Rc::clone(&payload));

    assert_eq!(Rc::strong_count(&payload), 2);
    map.remove(key);
    assert_eq!(Rc::strong_count(&payload), 1);
}
