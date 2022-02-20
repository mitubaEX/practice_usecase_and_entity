pub trait Interface {
    fn reserved(&self) -> u8;
}

pub struct Taxi {
    pub id: u8,
}

impl Interface for Taxi {
    fn reserved(&self) -> u8 {
        return self.id
    }
}

#[test]
fn default_test() {
    let taxi = Taxi {
        id: 1
    };
    assert_eq!(taxi.reserved(), 1)
}
