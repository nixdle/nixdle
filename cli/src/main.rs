use nixdle::api;
use reqwest::Client;

mod cli;
mod crypto;
mod error;
mod lockfile;

use error::Result;
use lockfile::Lockfile;

const DEFAULT_API_URL: &str = "https://adamperkowski.dev/api/nixdle";
const LOCKFILE_PATH: &str = "/tmp/nixdle.lock";
const LOCKFILE_SIGNATURE_PATH: &str = "/tmp/nixdle.lock.sig";

#[tokio::main]
async fn main() {
  let code = if let Err(e) = run().await {
    cli::print_error(e.to_string());
    1
  } else {
    0
  };

  cli::reset();
  std::process::exit(code);
}

async fn run() -> Result<()> {
  let args = cli::args::parse();
  let url = args.api;

  cli::print_welcome();
  cli::print_status(format!("connecting to {}", url));

  let client = Client::new();
  let start_message = request_start(&client, &format!("{}/start", url)).await?;

  if start_message.version != env!("CARGO_PKG_VERSION") {
    cli::print_warning(format!(
      "version mismatch (server: {}, client: {})",
      start_message.version,
      env!("CARGO_PKG_VERSION")
    ));
  }

  let key = start_message.date.clone() + &start_message.version + &start_message.nix_commit;
  let mut lockfile = Lockfile::open(&key)?;

  if lockfile.success {
    cli::print_already_solved();
    return Ok(());
  }

  if !args.hide_rules {
    cli::print_rules(start_message.rules);
  }

  lockfile.date = start_message.date;
  lockfile.version = start_message.version;
  lockfile.save(&key)?;

  let attempt_url = &start_message.attempt_url;
  let time_started = std::time::Instant::now();

  loop {
    let input = cli::dialog(format!("guess#{}", lockfile.attempts));

    cli::print_status(format!("sending to {}", attempt_url));

    let attempt_data = api::AttemptData {
      input: input.clone(),
      attempts: lockfile.attempts,
    };
    let attempt_message = request_attempt(&client, attempt_url, &attempt_data).await?;

    if let Some(msg) = attempt_message {
      cli::print_status("saving".to_string());

      lockfile.attempts += 1;
      lockfile.attempted.push(input);

      if msg.success {
        cli::print_solved(
          &msg.func.unwrap_or_default(),
          &msg.description.unwrap_or_default(),
          lockfile.attempts,
          time_started.elapsed().as_secs(),
          &lockfile.date,
        );

        lockfile.success = true;
        lockfile.save(&key)?;
        break;
      } else {
        cli::print_attempt(msg.clues, msg.args.to_string(), msg.input, msg.output);
      }

      lockfile.save(&key)?;
    } else {
      cli::print_error("the server doesn't know this one :c".to_string());
      continue;
    }
  }

  Ok(())
}

async fn request_start(client: &Client, url: &str) -> Result<api::StartMessage> {
  let res = client.get(url).send().await?;

  res.json::<api::StartMessage>().await.map_err(Into::into)
}

async fn request_attempt(
  client: &Client,
  url: &str,
  data: &api::AttemptData,
) -> Result<Option<api::AttemptMessage>> {
  let res = client.post(url).json(data).send().await?;

  res
    .json::<Option<api::AttemptMessage>>()
    .await
    .map_err(Into::into)
}
