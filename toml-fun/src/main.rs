use toml::Value;

fn main() {
  let value = "foo = 'bar'".parse::<Value>().unwrap();
  assert_eq!(value["foo"].as_str(), Some("bar"));
  println!("{}",value);
}
