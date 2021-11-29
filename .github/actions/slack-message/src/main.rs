use serde_json::json;
use std::env;
use std::process;

fn main() -> attohttpc::Result {
    env_logger::init();

    let slack_webhook = env::var("INPUT_SLACK_WEBHOOK").expect("SLACK_WEBHOOK env var is required");
    let slack_message = env::var("INPUT_MESSAGE").expect("MESSAGE env var is required");
    let github_repository = env::var("GITHUB_REPOSITORY").unwrap();
    let github_event_name = env::var("GITHUB_EVENT_NAME").unwrap();
    let github_ref = env::var("GITHUB_REF").unwrap();
    let github_sha = env::var("GITHUB_SHA").unwrap();

    println!("::debug ::Sending a request to slack");

    let obj = json!({
        "blocks": [
            {
                "type": "section",
                "text": {
                    "type": "mrkdwn",
                    "text": format!("*{}*",slack_message),
                }
            },
            {
                "type": "section",
                "fields": [
                    {
                        "type": "mrkdwn",
                        "text": format!("*Repository:*\n{}", github_repository),
                    },
                    {
                        "type": "mrkdwn",
                        "text": format!("*Event:*\n{}", github_event_name),
                    },
                    {
                        "type": "mrkdwn",
                        "text": format!("*Ref:*\n{}", github_ref),
                    },
                    {
                        "type": "mrkdwn",
                        "text": format!("*SHA:*\n{}", github_sha),
                    },
                ]
            }
        ]
    });

    let resp = attohttpc::post(slack_webhook).json(&obj)?.send()?;

    let is_success: bool = resp.is_success();
    let resp_text = resp.text().unwrap();

    println!("::group::Slack Reponse");
    println!("{}", &resp_text);
    println!("::endgroup::");

    if !is_success {
        process::exit(1);
    }

    Ok(())
}
