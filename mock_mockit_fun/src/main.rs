use mock_it::Matcher::*;
use mock_it::*;

#[derive(Debug, PartialEq, Clone)]
struct House {
    price: f64,
}

impl House {
    fn new(price: f64) -> House {
        House { price }
    }
}

trait HouseFactory {
    fn create(&self, specs: Vec<String>) -> Result<House, String>;
}

struct HouseFactoryMock {
    create: Mock<Matcher<Vec<String>>, Result<House, String>>,
}

impl HouseFactoryMock {
    fn new() -> HouseFactoryMock {
        HouseFactoryMock {
            create: Mock::new(Err("Default value".to_string())),
        }
    }
}

impl HouseFactory for HouseFactoryMock {
    fn create(&self, specs: Vec<String>) -> Result<House, String> {
        self.create.called(Val(specs.clone()))
    }
}

fn main() {
    let house_factory_mock = HouseFactoryMock::new();
    let expected_house = House::new(250_000.99);
    house_factory_mock
        .create
        .given(Any)
        .will_return(Ok(expected_house.clone()));

    let house_factory = Box::new(house_factory_mock);
    let house = house_factory.create(vec![]);

    match house {
        Ok(house) => assert_eq!(expected_house, house),
        Err(message) => panic!(message),
    }
}

#[test]
fn mockit_main_astest(){
    let house_factory_mock = HouseFactoryMock::new();
    let expected_house = House::new(250_000.99);
    house_factory_mock
        .create
        .given(Any)
        .will_return(Ok(expected_house.clone()));

    let house_factory = Box::new(house_factory_mock);
    let house = house_factory.create(vec![]);

    match house {
        Ok(house) => assert_eq!(expected_house, house),
        Err(message) => panic!(message),
    }
}