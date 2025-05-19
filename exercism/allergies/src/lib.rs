use lazy_static::lazy_static;
use regex::Regex;

pub struct Allergies {
    score: u8,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score: score as u8 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
       self.score & *allergen as u8 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let all_of_them = vec![
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        all_of_them.into_iter().filter(|x| self.is_allergic_to(x)).collect()
    }
}




fn extract_login(input: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^.*\[(?P<auditLogHost>.*)\]client-api-auth operationLog:(?P<auditLogBase64>.*)\"}"#).unwrap();
    }
    RE.captures(input).and_then(|cap| {
        cap.name("auditLogHost").map(|auditLogHost| auditLogHost.as_str())
    })
}

#[test]
#[ignore]
fn test_pattern() {
    let str = "2024-01-22T16:38:32.488797105+08:00 2024/01/22 08:38:32 [warn] 59#59: *394 [lua] init.lua:953: G 2024-01-22T08:38:32.488Z INFO [-] - biz/client_api_auth.go:512 - codeup.aliyun.com/midware/apisix-go-plugin-runner/pkg/plugin/biz.(*jwtAuthSegment).check  {\"level\": \"INFO\", \"msg\": \"[api-test-open.eiot6.com]client-api-auth operationLog:eyJhcHBDb2RlIjoicGhvdG92b2x0YWljIiwidGVuYW50Q29kZSI6IjcxMzc1NSIsInNpdGVDb2RlIjoicGhvdG92b2x0YWljLXByb3ZpZGVyIiwidWlkIjoidGVzdF91aWQiLCJtYWluQWNjdFVpZCI6InRlc3RfdWlkIiwiYWNjdE5hbWUiOiLpo57ova4xMjMiLCJvY2N1cnJlZE9uVHMiOjE3MDU5MTI3MTI0ODgsIm9wZXJhdGVUeXBlIjoiW1wiX19tYWluQWNjdF9fOjpvcGVyYXRpb25cIl0iLCJ0YXJnZXRUeXBlIjoiIiwidGFyZ2V0SWQiOiIiLCJvcGVyYXRlUmVzdWx0IjoicHJlUmVxdWVzdCIsIm9wZXJhdGlvbk5hbWUiOiIiLCJvcGVyYXRpb25VcmwiOiJHRVQ6cGhvdG92b2x0YWljL21pY3JvLWFwaS92MS9wcm92aWRlci9kZXZpY2VzLzE4NDY5YjYwNDU1ZWQwMDAvcHJvcGVydGllcyIsImlwIjoiMTAuNDAuMC4xMDkiLCJleHRyYUluZm8iOnt9fQ==\"}";
    assert_eq!(extract_login(str), Some("api-test-open.eiot6.com"));
}