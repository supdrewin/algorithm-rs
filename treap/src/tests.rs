use super::TreapMap;

#[test]
fn map() {
    let mut map = TreapMap::new();
    map.insert(10, 10);
    map.insert(20, 20);
    map.insert(30, 30);
    map.insert(40, 40);
    map.insert(50, 50);
    map.insert(60, 60);
    map.insert(70, 70);
    map.insert(80, 80);
    map.insert(90, 90);
    println!("{map:#?}");
    assert_eq!(
        map.clone().into_iter().collect::<Vec<_>>(),
        vec![
            (10, 10),
            (20, 20),
            (30, 30),
            (40, 40),
            (50, 50),
            (60, 60),
            (70, 70),
            (80, 80),
            (90, 90)
        ]
    );
    assert_eq!(
        map.range(20..60).collect::<Vec<_>>(),
        vec![(&20, &20), (&30, &30), (&40, &40), (&50, &50),]
    );
    map.iter_mut().for_each(|(_, val)| *val += 5);
    println!("{map:#?}");
    assert_eq!(
        map.clone().into_iter().collect::<Vec<_>>(),
        vec![
            (10, 15),
            (20, 25),
            (30, 35),
            (40, 45),
            (50, 55),
            (60, 65),
            (70, 75),
            (80, 85),
            (90, 95)
        ]
    );
    assert_eq!(map.remove(&1), None);
    assert_eq!(map.remove(&10), Some(15));
    assert_eq!(map.remove_entry(&40), Some((40, 45)));
    assert_eq!(map.remove_entry(&40), None);
    println!("{map:#?}");
    assert_eq!(
        map.clone().into_iter().collect::<Vec<_>>(),
        vec![
            (20, 25),
            (30, 35),
            (50, 55),
            (60, 65),
            (70, 75),
            (80, 85),
            (90, 95)
        ]
    );
    map.extend([(45, 100), (15, 100)]);
    map.range_mut(16..=80).for_each(|(_, val)| *val -= 20);
    println!("{map:#?}");
    assert_eq!(
        map.into_iter().collect::<Vec<_>>(),
        vec![
            (15, 100),
            (20, 5),
            (30, 15),
            (45, 80),
            (50, 35),
            (60, 45),
            (70, 55),
            (80, 65),
            (90, 95)
        ]
    );
}
