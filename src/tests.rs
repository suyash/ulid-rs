use super::Ulid;

#[test]
fn new() {
    let ulid = Ulid::new(1_484_581_420, || 4);
    assert_eq!(ulid.to_string(), "0001C7STHC0G2081040G208104");
}

#[test]
fn unmarshal() {
    let ulid = Ulid::unmarshal("0001C7STHC0G2081040G208104");
    let ulid2 = Ulid::new(1_484_581_420, || 4);
    assert_eq!(ulid.unwrap(), ulid2);

    let res = Ulid::unmarshal("0001C7STHC0G2O81040G208104");
    assert!(res.is_err());

    let res = Ulid::unmarshal("0001C7STHC0G2O81040G20810");
    assert!(res.is_err());
}

#[test]
fn timestamp() {
    let ulid = Ulid::unmarshal("0001C7STHC0G2081040G208104").unwrap();
    assert_eq!(ulid.timestamp(), 1_484_581_420);
}

/// https://github.com/oklog/ulid/blob/master/ulid_test.go#L160-L169
#[test]
fn alizain_compatibility() {
    let ulid: Ulid = Ulid::new(1_469_918_176_385, || 0);
    assert_eq!(ulid.to_string(), "01ARYZ6S410000000000000000");
}

#[test]
fn lexicographical_order() {
    let ulid1 = Ulid::new(1_469_918_176_385, || 0);
    let ulid2 = Ulid::new(1_469_918_176_386, || 0);
    assert!(ulid1 < ulid2);
    assert!(ulid2 > ulid1);
}
