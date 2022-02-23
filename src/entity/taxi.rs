pub trait Interface {
    fn reserved(&self) -> Taxi;
}

pub struct Taxi {
    pub id: u8,
    pub reserved: bool
}

impl Interface for Taxi {
    fn reserved(&self) -> Taxi {
        return Self {
            id: self.id,
            reserved: true
        }
    }
}

#[test]
fn default_test() {
    let taxi = Taxi {
        id: 1,
        reserved: false
    };
    assert_eq!(taxi.reserved, false);
    assert_eq!(taxi.reserved().reserved, true);
}
