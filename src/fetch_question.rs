use bson::Document;
use serde::{Deserialize,Serialize};

use crate::env::get_env;

#[derive(Debug,Serialize,Deserialize)]
struct Question{
    _id:u32,
    update:u32,
    test_bench:Document,
}

pub fn fetch_question_by_id(question_id:u32){
    let question_folder=get_env("QUESTION_FOLDER");
    let url=get_env("API_URL")+"/question/"+&(question_id.to_string());
    let resp = reqwest::blocking::get(url)
        .unwrap()
        .json::<Question>()
        .unwrap();
    println!("{:?}",resp);
}
    
