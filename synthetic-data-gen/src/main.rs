use axum::{
    extract::Query,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use chrono::{Datelike, Utc};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tokio;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize)]
struct GenerateRequest {
    fields: String,     
    count: Option<u32>,
}

#[derive(Debug, Serialize)]
struct PersonRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dob: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ssn: Option<String>,
}

#[derive(Debug, Serialize)]
struct GenerateResponse {
    data: Vec<PersonRecord>,
    count: u32,
}

struct DataGenerator {
    first_names: Vec<&'static str>,
    last_names: Vec<&'static str>,
    email_domains: Vec<&'static str>,
}

impl DataGenerator {
    fn new() -> Self {
        Self {
            first_names: vec![
                "James", "Mary", "John", "Patricia", "Robert", "Jennifer", "Michael", "Linda",
                "William", "Elizabeth", "David", "Barbara", "Richard", "Susan", "Joseph", "Jessica",
                "Thomas", "Sarah", "Christopher", "Karen", "Charles", "Nancy", "Daniel", "Lisa",
                "Matthew", "Betty", "Anthony", "Helen", "Mark", "Sandra", "Donald", "Donna",
                "Steven", "Carol", "Paul", "Ruth", "Andrew", "Sharon", "Joshua", "Michelle",
                "Kenneth", "Laura", "Kevin", "Sarah", "Brian", "Kimberly", "George", "Deborah",
                "Timothy", "Dorothy", "Ronald", "Lisa", "Jason", "Nancy", "Edward", "Karen",
                "Jeffrey", "Betty", "Ryan", "Helen", "Jacob", "Sandra", "Gary", "Donna",
                "Nicholas", "Carol", "Eric", "Ruth", "Jonathan", "Sharon", "Stephen", "Michelle",
                "Larry", "Laura", "Justin", "Sarah", "Scott", "Kimberly", "Brandon", "Deborah",
                "Benjamin", "Dorothy", "Samuel", "Amy", "Gregory", "Angela", "Alexander", "Ashley",
                "Patrick", "Brenda", "Frank", "Emma", "Raymond", "Olivia", "Jack", "Cynthia",
                "Dennis", "Marie", "Jerry", "Janet", "Tyler", "Catherine", "Aaron", "Frances",
                "Jose", "Christine", "Henry", "Samantha", "Adam", "Debra", "Douglas", "Rachel",
                "Nathan", "Carolyn", "Peter", "Janet", "Zachary", "Virginia", "Kyle", "Maria",
                "Walter", "Heather", "Ethan", "Diane", "Carl", "Julie", "Arthur", "Joyce",
                "Gerald", "Victoria", "Roger", "Kelly", "Keith", "Christina", "Jeremy", "Joan",
                "Terry", "Evelyn", "Aaron", "Megan", "Albert", "Hannah", "Joe", "Katherine",
                "Austin", "Lauren", "Willie", "Jacqueline", "Beverly", "Bruce", "Denise",
                "Jordan", "Amber", "Dylan", "Brittany", "Bryan", "Natalie", "Eugene", "Samantha",
                "Louis", "Katherine", "Russell"
            ],
            last_names: vec![
                "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis",
                "Rodriguez", "Martinez", "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson", "Thomas",
                "Taylor", "Moore", "Jackson", "Martin", "Lee", "Perez", "Thompson", "White",
                "Harris", "Sanchez", "Clark", "Ramirez", "Lewis", "Robinson", "Walker", "Young",
                "Allen", "King", "Wright", "Scott", "Torres", "Nguyen", "Hill", "Flores",
                "Green", "Adams", "Nelson", "Baker", "Hall", "Rivera", "Campbell", "Mitchell",
                "Carter", "Roberts", "Gomez", "Phillips", "Evans", "Turner", "Diaz", "Parker",
                "Cruz", "Edwards", "Collins", "Reyes", "Stewart", "Morris", "Morales", "Murphy",
                "Cook", "Rogers", "Gutierrez", "Ortiz", "Morgan", "Cooper", "Peterson", "Bailey",
                "Reed", "Kelly", "Howard", "Ramos", "Kim", "Cox", "Ward", "Richardson",
                "Watson", "Brooks", "Chavez", "Wood", "James", "Bennett", "Gray", "Mendoza",
                "Ruiz", "Hughes", "Price", "Alvarez", "Castillo", "Sanders", "Patel", "Myers",
                "Long", "Ross", "Foster", "Jimenez", "Powell", "Jenkins", "Perry", "Russell",
                "Sullivan", "Bell", "Coleman", "Butler", "Henderson", "Barnes", "Gonzales", "Fisher",
            ],
            email_domains: vec![
                "gmail.com", "yahoo.com", "hotmail.com", "outlook.com", "aol.com",
                "icloud.com", "protonmail.com", "mail.com", "yandex.com", "zoho.com",
                "simplemail.com", "fastmail.com", "theemail.com", "fakefortest.com"
            ],
        }
    }

    fn generate_first_name(&self) -> String {
        let mut rng = thread_rng();
        self.first_names[rng.gen_range(0..self.first_names.len())].to_string()
    }

    fn generate_last_name(&self) -> String {
        let mut rng = thread_rng();
        self.last_names[rng.gen_range(0..self.last_names.len())].to_string()
    }

    fn generate_email(&self, first_name: Option<&str>, last_name: Option<&str>) -> String {
        let mut rng = thread_rng();
        
        let first = first_name.unwrap_or(&self.generate_first_name()).to_lowercase();
        let last = last_name.unwrap_or(&self.generate_last_name()).to_lowercase();
        let domain = self.email_domains[rng.gen_range(0..self.email_domains.len())];
        
        match rng.gen_range(0..4) {
            0 => format!("{}.{}@{}", first, last, domain),
            1 => format!("{}{}@{}", first, last, domain),
            2 => format!("{}.{}{}@{}", first, last, rng.gen_range(1..999), domain),
            _ => format!("{}{}{}@{}", first, last, rng.gen_range(1..99), domain),
        }
    }

    fn generate_dob(&self) -> String {
        let mut rng = thread_rng();
        
        let years_ago = rng.gen_range(18..81);
        let current_year = Utc::now().year();
        let birth_year = current_year - years_ago;
        
        let month = rng.gen_range(1..13);
        let day = match month {
            2 => rng.gen_range(1..29), 
            4 | 6 | 9 | 11 => rng.gen_range(1..31),
            _ => rng.gen_range(1..32),
        };
        
        format!("{:04}-{:02}-{:02}", birth_year, month, day)
    }

    fn generate_ssn(&self) -> String {
        let mut rng = thread_rng();
        let area = rng.gen_range(100..900);
        let group = rng.gen_range(10..100);
        let serial = rng.gen_range(1000..10000);
        format!("{:03}-{:02}-{:04}", area, group, serial)
    }

    fn generate_record(&self, fields: &[String]) -> PersonRecord {
        let mut record = PersonRecord {
            first_name: None,
            last_name: None,
            email: None,
            dob: None,
            ssn: None,
        };

        let mut generated_first_name = None;
        let mut generated_last_name = None;

        for field in fields {
            match field.as_str() {
                "first_name" => {
                    let name = self.generate_first_name();
                    generated_first_name = Some(name.clone());
                    record.first_name = Some(name);
                }
                "last_name" => {
                    let name = self.generate_last_name();
                    generated_last_name = Some(name.clone());
                    record.last_name = Some(name);
                }
                _ => {}
            }
        }

        for field in fields {
            match field.as_str() {
                "email" => {
                    record.email = Some(self.generate_email(
                        generated_first_name.as_deref(),
                        generated_last_name.as_deref(),
                    ));
                }
                "dob" => {
                    record.dob = Some(self.generate_dob());
                }
                "ssn" => {
                    record.ssn = Some(self.generate_ssn());
                }
                _ => {}
            }
        }

        record
    }
}

async fn generate_data(Query(params): Query<GenerateRequest>) -> Result<Json<GenerateResponse>, StatusCode> {
    let count = params.count.unwrap_or(1).min(1000);
    
    if count == 0 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let fields: Vec<String> = params.fields
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if fields.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    for field in &fields {
        match field.as_str() {
            "first_name" | "last_name" | "email" | "dob" | "ssn" => {}
            _ => return Err(StatusCode::BAD_REQUEST),
        }
    }

    let generator = DataGenerator::new();
    let mut data = Vec::new();

    for _ in 0..count {
        data.push(generator.generate_record(&fields));
    }

    Ok(Json(GenerateResponse { data, count }))
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "synthetic-data-generator",
        "available_fields": ["first_name", "last_name", "email", "dob", "ssn"]
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/generate", get(generate_data))
        .route("/health", get(health_check))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Synthetic Data Generator API running on http://0.0.0.0:3000");
    println!("📋 Available endpoints:");
    println!("   GET /health - Health check");
    println!("   GET /generate?fields=first_name,last_name&count=5 - Generate data");
    println!("📝 Available fields: first_name, last_name, email, dob, ssn");

    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_generator() {
        let generator = DataGenerator::new();
        
        let first_name = generator.generate_first_name();
        assert!(!first_name.is_empty());
        
        let last_name = generator.generate_last_name();
        assert!(!last_name.is_empty());
        
        let email = generator.generate_email(Some("john"), Some("doe"));
        assert!(email.contains("@"));
        assert!(email.contains("john"));
        assert!(email.contains("doe"));
        
        let dob = generator.generate_dob();
        assert!(dob.len() == 10);
        
        let ssn = generator.generate_ssn();
        assert!(ssn.len() == 11);
        assert!(ssn.chars().filter(|&c| c == '-').count() == 2);
    }

    #[test]
    fn test_record_generation() {
        let generator = DataGenerator::new();
        let fields = vec!["first_name".to_string(), "email".to_string()];
        let record = generator.generate_record(&fields);
        
        assert!(record.first_name.is_some());
        assert!(record.email.is_some());
        assert!(record.last_name.is_none());
        assert!(record.dob.is_none());
        assert!(record.ssn.is_none());
    }
}