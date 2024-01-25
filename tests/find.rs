use {
  super::*,
  ord::subcommand::find::{FindRangeOutput, Output},
};

#[test]
fn find_command_returns_satpoint_for_sat() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  assert_eq!(
    CommandBuilder::new("--index-sats find 0")
      .rpc_server(&rpc_server)
      .run_and_deserialize_output::<Output>(),
    Output {
      satpoint: "ed34050eb5909ee535fcb07af292ea55f3d2f291187617b44d3282231405b96d:0:0"
        .parse()
        .unwrap()
    }
  );
}

#[test]
fn find_range_command_returns_satpoints_and_ranges() {
  let rpc_server = test_bitcoincore_rpc::spawn();

  rpc_server.mine_blocks(1);

  pretty_assert_eq!(
    CommandBuilder::new(format!("--index-sats find 0 {}", 55 * COIN_VALUE))
      .rpc_server(&rpc_server)
      .run_and_deserialize_output::<Vec<FindRangeOutput>>(),
    vec![
      FindRangeOutput {
        start: 0,
        size: 50 * COIN_VALUE,
        satpoint: "ed34050eb5909ee535fcb07af292ea55f3d2f291187617b44d3282231405b96d:0:0"
          .parse()
          .unwrap()
      },
      FindRangeOutput {
        start: 50 * COIN_VALUE,
        size: 5 * COIN_VALUE,
        satpoint: "84aca0d43f45ac753d4744f40b2f54edec3a496b298951735d450e601386089d:0:0"
          .parse()
          .unwrap()
      }
    ]
  );
}

#[test]
fn find_range_command_fails_for_unmined_sat_ranges() {
  let rpc_server = test_bitcoincore_rpc::spawn();

  CommandBuilder::new(format!(
    "--index-sats find {} {}",
    50 * COIN_VALUE,
    100 * COIN_VALUE
  ))
  .rpc_server(&rpc_server)
  .expected_exit_code(1)
  .expected_stderr("error: range has not been mined as of index height\n")
  .run_and_extract_stdout();
}

#[test]
fn unmined_sat() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  CommandBuilder::new("--index-sats find 5000000000")
    .rpc_server(&rpc_server)
    .expected_stderr("error: sat has not been mined as of index height\n")
    .expected_exit_code(1)
    .run_and_extract_stdout();
}

#[test]
fn no_satoshi_index() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  CommandBuilder::new("find 0")
    .rpc_server(&rpc_server)
    .expected_stderr("error: find requires index created with `--index-sats` flag\n")
    .expected_exit_code(1)
    .run_and_extract_stdout();
}
