use std::{
    env,
    io::{stdin, stdout, Result, Write},
};

use hyper::{
    body::{self, Buf},
    header, Body, Client, Request,
};
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use serde_json::json;
use spinners::Spinner;

#[derive(Serialize, Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logpb: Option<u8>,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OAIResp {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OAIReq {
    prompt: String,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Idk {
    model: String,
    prompt: String,
    temperature: f32,
    max_tokens: u32,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
    stop: Vec<String>,
}

#[tokio::main]
async fn main() {
    new_ver().await;
}

async fn old_ver() -> Result<()> {
    let http = HttpsConnector::new();
    let client = Client::builder().build(http);
    let url = "https://api.openai.com/v1/engines/text-davinci-003/completions";

    let oai_token: String = env::var("OAI_TOKEN").unwrap();

    let auth_header_val = format!("Bearer {}", oai_token);

    let mut user_text = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut user_text).expect("Fucked");
        user_text.push_str(" \t\t\t\t");
        println!("");

        let mut sp = Spinner::new(spinners::Spinners::Dots, "\t\t.....".into());

        let oai_request = OAIReq {
            prompt: format!("{user_text}"),
            max_tokens: 100,
        };

        let body = Body::from(serde_json::to_vec(&oai_request).unwrap());
        let relbd = Body::from(serde_json::to_vec(&oai_request).unwrap());

        let req = Request::post(url)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, &auth_header_val)
            .body(body)
            .unwrap();

        println!("{:?}", relbd);

        let res = client.request(req).await.unwrap();

        let body = body::aggregate(res).await.unwrap();

        let json: OAIResp = serde_json::from_reader(body.reader()).unwrap();

        sp.stop();
        println!("");

        println!("{}", json.choices[0].text);
    }
}

async fn new_ver() {
    let https = HttpsConnector::new();
    let client = Client::builder().build(https);
    let uri = "https://api.openai.com/v1/completions";
    let oai_token: String = env::var("OAI_TOKEN").unwrap();

    let header_token = format!("Bearer {oai_token}");

    let mut text = String::new();

    loop {
        text.push_str(" Human:");

        print!("> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut text).expect("Fucked!");
        text.push_str(" Ai:");

        let oai_request = Idk {
            model: "text-davinci-003".to_string(),
            prompt: text.clone(),
            temperature: 0.9,
            max_tokens: 150,
            top_p: 1.,
            frequency_penalty: 0.,
            presence_penalty: 0.6,
            stop: vec![" Human:".to_string(), " Ai:".to_string()],
        };

        let body = Body::from(serde_json::to_vec(&oai_request).unwrap());

        let req = Request::post(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, header_token.clone())
            .body(body)
            .unwrap();

        let res = client.request(req).await.unwrap();

        let body = body::aggregate(res).await.unwrap();

        let json: OAIResp = serde_json::from_reader(body.reader()).unwrap();

        println!("");

        println!("{}", json.choices[0].text);
    }
}
