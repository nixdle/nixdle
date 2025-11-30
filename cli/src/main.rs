use clap::Parser;
use nixdle::api;
use reqwest::blocking::Client;

mod args;
mod crypto;
mod lockfile;
mod theme;
mod ui;

use color_eyre::Result;
use lockfile::Lockfile;

use crate::{args::Cli, ui::Ui};

const DEFAULT_API_URL: &str = "https://adamperkowski.dev/api/nixdle";
const LOCKFILE_PATH: &str = "/tmp/nixdle.lock";
const LOCKFILE_SIGNATURE_PATH: &str = "/tmp/nixdle.lock.sig";

fn main() -> Result<()> {
  color_eyre::install()?;
  let args = Cli::parse();
  let url = args.api;
  let mut ui = Ui::new(args.theme.into());
  ui.print_welcome()?;
  ui.print_status(&format!("connecting to {}", url))?;

  let client = Client::new();
  let start_message = request_start(&client, &format!("{}/start", url))?;

  if start_message.version != env!("CARGO_PKG_VERSION") {
    ui.print_warning(&format!(
      "version mismatch (server: {}, client: {})",
      start_message.version,
      env!("CARGO_PKG_VERSION")
    ))?;
  }

  let key = start_message.date.clone() + &start_message.version + &start_message.nix_commit;
  let mut lockfile = Lockfile::open(&key)?;

  if lockfile.success {
    ui.print_already_solved()?;
    return Ok(());
  }

  if !args.hide_rules {
    ui.print_rules(&start_message.rules)?;
  }

  lockfile.date = start_message.date;
  lockfile.version = start_message.version;
  lockfile.save(&key)?;

  let attempt_url = &start_message.attempt_url;
  let time_started = std::time::Instant::now();

  loop {
    let input = ui.dialog(&format!("guess#{}", lockfile.attempts))?;

    ui.print_status(&format!("sending to {}", attempt_url))?;

    let attempt_data = api::AttemptData {
      input: input.clone(),
      attempts: lockfile.attempts,
    };
    let attempt_message = request_attempt(&client, attempt_url, &attempt_data)?;

    if let Some(msg) = attempt_message {
      ui.print_status("saving")?;

      lockfile.attempts += 1;
      lockfile.attempted.push(input);

      if msg.success {
        ui.print_solved(
          &msg.func.unwrap_or_default(),
          &msg.description.unwrap_or_default(),
          lockfile.attempts,
          time_started.elapsed().as_secs(),
          &lockfile.date,
        )?;

        lockfile.success = true;
        lockfile.save(&key)?;
        break;
      } else {
        ui.print_attempt(msg.clues, msg.args.to_string(), msg.input, msg.output)?;
      }

      lockfile.save(&key)?;
    } else {
      ui.print_error("the server doesn't know this one :c")?;
      continue;
    }
  }

  Ok(())
}

fn request_start(client: &Client, url: &str) -> Result<api::StartMessage> {
  let res = client.get(url).send()?;

  res.json::<api::StartMessage>().map_err(Into::into)
}

fn request_attempt(
  client: &Client,
  url: &str,
  data: &api::AttemptData,
) -> Result<Option<api::AttemptMessage>> {
  let res = client.post(url).json(data).send()?;

  res
    .json::<Option<api::AttemptMessage>>()
    .map_err(Into::into)
}
