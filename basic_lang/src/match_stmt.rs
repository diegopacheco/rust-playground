pub fn execute(){
    let countryCode = 44;
    let country = match countryCode {
        1  => "USA",
        44 => "UK",
        55 => "Brazil",
        _  => "IDK"
    };

    println!("Country: {}", country);
}