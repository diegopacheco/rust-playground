#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub address: Address,
    pub phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub street: String,
    pub city: String,
}