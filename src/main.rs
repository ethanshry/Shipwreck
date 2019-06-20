#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use reqwest::Error;

use lapin_async as lapin;

use crate::lapin::{
  BasicProperties, Connection, ConnectionProperties, ConsumerSubscriber, Channel,
  message::Delivery,
  options::*,
  types::FieldTable,
};

use std::process::Command;

#[derive(Clone,Debug)]
struct Subscriber {
  channel: Channel,
}

impl ConsumerSubscriber for Subscriber {
    fn new_delivery(&self, delivery: Delivery) {
      print!("received message: {:?}", String::from_utf8_lossy(&delivery.data));
      let msg: Vec<&str> = std::str::from_utf8(&delivery.data).unwrap().split(',').collect();

      let res: Result<(), Error> = clone_repo(msg[0], msg[1]);

      self.channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default()).into_result().expect("basic_ack");
      println!(" - acked");
    }
    fn drop_prefetched_messages(&self) {}
    fn cancel(&self) {}
}

#[derive(Deserialize, Debug)]
struct RepoCommitMetadata {
    message: String
}

#[derive(Deserialize, Debug)]
struct RepoCommits {
    sha: String,
    url: String,
    commit: RepoCommitMetadata
}

fn clone_repo(username: &str, repository: &str) -> Result<(), Error> {
    
    let request_url = format!(
      "https://api.github.com/repos/{owner}/{repo}/commits",
        owner = username,
        repo = repository
    );

    let mut commits_response = reqwest::get(&request_url)?;

    if commits_response.status() != reqwest::StatusCode::OK {
    
        println!("Could not get repo commits");
    
    } else {
    
        let commits: Vec<RepoCommits> = commits_response.json()?;
    
        println!("=== Retrieved commits for {} from Github ===", repository);
        for commit in &commits {
            println!("{} - {}", commit.sha, commit.commit.message)
        }
        println!("=== End of commits ===");

        let last_commit_hash = &commits[0].sha;

        println!("Checking for shipwreck.toml in commit {}", last_commit_hash);

        let request_url = format!(
            "https://raw.githubusercontent.com/{owner}/{repo}/{commit_hash}/shipwreck.toml",
            owner = username,
            repo = repository,
            commit_hash = last_commit_hash
        );
        
        let mut toml_response = reqwest::get(&request_url)?;

        if toml_response.status() != reqwest::StatusCode::OK {
            println!("Commit {} does not have a valid shipwreck.toml file, skipping deploy", last_commit_hash);
        } else {

            let body: String = toml_response.text()?;

            println!("=== shipwreck.toml ===");

            let toml_lines: Vec<&str> = body.split('\n').collect();

            for line in &toml_lines {
                if !line.is_empty() {
                    println!("{}", line);
                }
            }

            println!("=== shipwreck.toml ===");

            let folder_name = format!("{}-{}", repository, last_commit_hash);

            // Clone repo and checkout specified commit
            let child = Command::new("/bin/bash")
                .args(&[
                    "-c", 
                    &format!(
                        "cd /home/ethanshry && rm -rf {} && git clone https://github.com/{}/{}.git {} && cd {} && git checkout {}", 
                        folder_name,
                        username,
                        repository,
                        folder_name,
                        folder_name,
                        last_commit_hash
                    )]).spawn().expect("failure");
        }
        
    }
    
    Ok(())
}



// https://raw.githubusercontent.com/ethanshry/scapegoat/master/shipwreck.toml

fn main() {
    
    // establish connection
    let addr = "amqp://127.0.0.1:5672/";
    let conn = Connection::connect(&addr, ConnectionProperties::default()).wait().expect("connection error");

    //receive channel
    let channel_b = conn.create_channel().wait().expect("create_channel");

    let queue = channel_b.queue_declare("deployment_trigger", QueueDeclareOptions::default(), FieldTable::default()).wait().expect("queue_declare");
  
    channel_b.basic_consume(&queue, "shipwreck_1", BasicConsumeOptions::default(), FieldTable::default(), Box::new(Subscriber { channel: channel_b.clone() })).wait().expect("basic_consume");

    loop {

    }

}