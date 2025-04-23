use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("how to work with json in Rust"),
        author: String::from("Aavesh Saifi"), 
        paragraphs: vec![ 
            Paragraph {
                name: String::from("first sentence")
            },
            Paragraph {
                name: String::from("body of the paragraph")
            },
            Paragraph {
                name: String::from("end of the paragraph")
            }
        ]
    };
    
    let json = serde_json::to_string(&article).unwrap();
    println!("the json is: {}", json); 
}