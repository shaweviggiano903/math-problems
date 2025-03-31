use std::collections::HashMap;

struct Solution {
    memo: HashMap<String, String>,
}

impl Solution {
    fn solve(&self, s: &str) -> String {
        let mut map = self.memo.clone();
        
        for (i, c) in s.chars().enumerate() {
            match (map.get(&format!("{}{}", i, c)), map.get(&c)) {
                (Some(value), Some(other)) => { 
                    if other == value.to_string() {
                        continue;
                    }
                    
                    let mut result = String::from(self.solve(s));
                    return format!("{}{}{}", result, i as char, c);
                },
                _ => {}
            }
        }

        self.solve(&s)
    }
}
