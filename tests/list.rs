use {super::*, ord::subcommand::list::Output};

#[test]
fn output_found() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  let output = CommandBuilder::new(
    "--index-sats list ed34050eb5909ee535fcb07af292ea55f3d2f291187617b44d3282231405b96d:0",
  )
  .rpc_server(&rpc_server)
  .run_and_deserialize_output::<Vec<Output>>();

  assert_eq!(
    output,
    vec![Output {
      output: "ed34050eb5909ee535fcb07af292ea55f3d2f291187617b44d3282231405b96d:0"
        .parse()
        .unwrap(),
      start: 0,
      end: 50 * COIN_VALUE,
      size: 50 * COIN_VALUE,
      offset: 0,
      rarity: "mythic".parse().unwrap(),
      name: "nvtdijuwxlp".into(),
    }]
  );
}

#[test]
fn output_not_found() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  CommandBuilder::new(
    "--index-sats list 0000000000000000000000000000000000000000000000000000000000000000:0",
  )
  .rpc_server(&rpc_server)
  .expected_exit_code(1)
  .expected_stderr("error: output not found\n")
  .run_and_extract_stdout();
}

#[test]
fn no_satoshi_index() {
  let rpc_server = test_bitcoincore_rpc::spawn();
  CommandBuilder::new("list ed34050eb5909ee535fcb07af292ea55f3d2f291187617b44d3282231405b96d:0")
    .rpc_server(&rpc_server)
    .expected_stderr("error: list requires index created with `--index-sats` flag\n")
    .expected_exit_code(1)
    .run_and_extract_stdout();
}
