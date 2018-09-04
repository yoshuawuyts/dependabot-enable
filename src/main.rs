#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations)]

#[macro_use]
extern crate human_panic;
extern crate github_auth;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate dependabot_enable;
extern crate exitfailure;
extern crate github_local_remote;

use dependabot_enable::Cli;
use exitfailure::ExitFailure;
use github_auth::{Authenticator, Scope};
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
  setup_panic!();
  let args = Cli::from_args();
  let auth = Authenticator::builder(env!("CARGO_PKG_NAME").into())
    .scope(Scope::PublicRepo)
    .build();

  let token = auth.auth()?;
  let stats = github_local_remote::stat(".")?;
  println!("stats {:?}", stats);
  args.log(env!("CARGO_PKG_NAME"))?;
  info!("program started");
  Ok(())
}
