use crate::entity::taxi::{Taxi};

pub fn find_taxi(taxi_id: u8) -> Taxi {
    return Taxi {
        id: taxi_id,
        reserved: false
    }
}

#[test]
fn test_find_taxi() {
    assert_eq!(find_taxi(1).id, 1)
}
