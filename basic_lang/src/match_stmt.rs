pub fn execute(){
    let country_code = 44;
    let country = match country_code {
        1  => "USA",
        44 => "UK",
        55 => "Brazil",
        _  => "IDK"
    };

    println!("Country: {}", country);
}