use std::sync::Arc;

use reqwest::{cookie::Jar, Client as ReqwestClient, Response, Url};

pub struct Client {
    session: String,
}

impl Client {
    pub fn new(session: String) -> Self {
        Self { session }
    }

    pub async fn fetch_input(
        self: &Self,
        year: u16,
        day: u8,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
        let session = format!("session={}", self.session);
        let jar = Jar::default();
        jar.add_cookie_str(&session, &url.parse::<Url>()?);
        let client = ReqwestClient::builder()
            .cookie_provider(Arc::new(jar))
            .build()?;
        let req = client.get(url);
        let res = req.send().await?;

        Ok(res.text().await?)
    }

    pub async fn submit_answer(
        self: &Self,
        year: u16,
        day: u8,
        level: u8,
        answer: &str,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
        let session = format!("session={}", self.session);
        let jar = Jar::default();
        jar.add_cookie_str(&session, &url.parse::<Url>()?);
        let client = ReqwestClient::builder()
            .cookie_provider(Arc::new(jar))
            .build()?;
        let req = client.post(url);
        let res = req
            .form(&[("level", level.to_string().as_str()), ("answer", answer)])
            .send()
            .await;

        Ok(res?)
    }
}

// async fn autosubmit() -> Result<(), Box<dyn Error>> {
//     let session_key = read_to_string(".cookie")?;
//     let done = read_to_string(".next")?
//         .split_whitespace()
//         .map(|raw| raw.parse::<u16>().unwrap())
//         .collect::<Vec<_>>();
//     let [next_day, next_id] = done.as_slice() else {
//         panic!(".done corrupted")
//     };

//     if *next_day == 26 {
//         println!("You did it!");
//         return Ok(());
//     }

//     for day in (*next_day as u8)..=25 {
//         let client = Client::new(2023, day, session_key.as_str());
//         let input = client.fetch_input().await?;
//         for id in (*next_id as u8)..=2 {
//             println!("-- day {} puzzle {} --", day, id);
//             let Ok(answer) = run_puzzle(day, id, input.as_str()) else {
//                 println!("puzzle not finished yet.");
//                 return Ok(());
//             };
//             println!("Your answer: {:?}", &answer);
//             let answer = answer.to_string();
//             let response = client
//                 .submit_answer(1, answer.as_str())
//                 .await?
//                 .text()
//                 .await?;
//             let re =
//                 Regex::new(r"You have (.*) left to wait.|please wait (.*) before trying again.")
//                     .unwrap();

//             if response.contains("That's not the right answer.") {
//                 println!("Answer not accepted.");
//                 return Ok(());
//             } else if let Some(captures) = re.captures(response.as_str()) {
//                 // Get the first capture group (the time remaining)
//                 if let Some(time_remaining) = captures.get(1) {
//                     println!(
//                         "Answer not accepted. Wait {} before submitting again.",
//                         time_remaining.as_str()
//                     );
//                 }
//                 return Ok(());
//             } else if response.contains("Did you already complete it?") {
//                 println!("This is a repeat answer, and the API doesn't like it.");
//                 return Ok(());
//             } else {
//                 println!("This puzzle's solution was accepted!");

//                 let replace_day = if id % 2 == 0 { day + 1 } else { day };
//                 let replace_id = if id % 2 == 1 { 2 } else { 1 };
//                 std::fs::write(".next", format!("{} {}", replace_day, replace_id))?;
//             }
//         }
//     }

//     Ok(())
// }
