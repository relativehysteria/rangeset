extern crate std;
use super::*;

const DEFAULT_RS: RangeSet<256> = RangeSet::new();

#[test]
fn test_range_new_valid() {
    let range = Range::new(5, 10);
    assert!(range.is_ok());
    let range = range.unwrap();
    assert_eq!(range.start, 5);
    assert_eq!(range.end, 10);
}

#[test]
fn test_range_new_invalid() {
    let range = Range::new(10, 5);
    assert!(range.is_err());
    assert_eq!(range.err().unwrap(), Error::InvalidRange);
}

#[test]
fn test_range_contains() {
    let range1 = Range::new(5, 15).unwrap();
    let range2 = Range::new(7, 10).unwrap();
    assert_eq!(range1.contains(&range2), true);
}

#[test]
fn test_range_overlaps() {
    let range1 = Range::new(5, 15).unwrap();
    let range2 = Range::new(10, 20).unwrap();
    let overlap = range1.overlaps(&range2);
    assert!(overlap.is_some());
    let overlap = overlap.unwrap();
    assert_eq!(overlap.start, 10);
    assert_eq!(overlap.end, 15);
}

#[test]
fn test_range_no_overlap() {
    let range1 = Range::new(5, 15).unwrap();
    let range2 = Range::new(16, 20).unwrap();
    assert!(range1.overlaps(&range2).is_none());
}

#[test]
fn test_rangeset_new() {
    let rangeset = DEFAULT_RS.clone();
    assert!(rangeset.is_empty());
    assert_eq!(rangeset.entries().len(), 0);
}

#[test]
fn test_rangeset_insert() {
    let mut rangeset = DEFAULT_RS.clone();
    rangeset.insert(Range::new(5, 15).unwrap()).unwrap();
    rangeset.insert(Range::new(20, 30).unwrap()).unwrap();

    let entries = rangeset.entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].start, 5);
    assert_eq!(entries[0].end, 15);
    assert_eq!(entries[1].start, 20);
    assert_eq!(entries[1].end, 30);
}

#[test]
fn test_rangeset_insert_overlap() {
    let mut rangeset = DEFAULT_RS.clone();
    rangeset.insert(Range::new(5, 15).unwrap()).unwrap();
    rangeset.insert(Range::new(10, 20).unwrap()).unwrap();

    let entries = rangeset.entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].start, 5);
    assert_eq!(entries[0].end, 20);
}

#[test]
fn test_rangeset_insert_touching() {
    let mut rangeset = DEFAULT_RS.clone();
    rangeset.insert(Range::new(5, 10).unwrap()).unwrap();
    rangeset.insert(Range::new(11, 15).unwrap()).unwrap();

    let entries = rangeset.entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].start, 5);
    assert_eq!(entries[0].end, 15);
}

#[test]
fn test_rangeset_remove() {
    let mut rangeset = DEFAULT_RS.clone();
    rangeset.insert(Range::new(5, 15).unwrap()).unwrap();
    rangeset.insert(Range::new(20, 30).unwrap()).unwrap();
    let removed = rangeset.remove(Range::new(7, 10).unwrap()).unwrap();
    assert!(removed);

    let entries = rangeset.entries();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].start, 5);
    assert_eq!(entries[0].end, 6);
    assert_eq!(entries[1].start, 11);
    assert_eq!(entries[1].end, 15);
}

#[test]
fn test_rangeset_remove_full_range() {
    let mut rangeset = DEFAULT_RS.clone();
    rangeset.insert(Range::new(5, 15).unwrap()).unwrap();
    let removed = rangeset.remove(Range::new(5, 15).unwrap()).unwrap();
    assert!(removed);
    assert!(rangeset.is_empty());
}

#[test]
fn test_rangeset_remove_noop() {
    let mut rangeset = DEFAULT_RS.clone();
    rangeset.insert(Range::new(5, 15).unwrap()).unwrap();
    let removed = rangeset.remove(Range::new(16, 20).unwrap()).unwrap();
    assert!(!removed);

    let entries = rangeset.entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].start, 5);
    assert_eq!(entries[0].end, 15);
}

#[test]
fn test_rangeset_insert_ordering() {
    let mut rangeset = DEFAULT_RS.clone();
    assert_eq!(rangeset.insert(Range::new(0x1a, 0x9ffff).unwrap()), Ok(()));
    assert_eq!(rangeset.insert(Range::new(0x2, 0x9).unwrap()), Ok(()));

    assert_eq!(rangeset.entries(),
        [Range { start: 0x2, end: 0x9 }, Range { start: 0x1a, end: 0x9ffff}]);
}
