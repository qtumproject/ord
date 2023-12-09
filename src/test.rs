pub(crate) use {
  super::*,
  crate::inscription::TransactionInscription,
  bitcoin::{
    blockdata::{opcodes, script, script::PushBytesBuf},
    ScriptBuf, Witness,
  },
  pretty_assertions::assert_eq as pretty_assert_eq,
  std::iter,
  test_bitcoincore_rpc::TransactionTemplate,
  unindent::Unindent,
};

macro_rules! assert_regex_match {
  ($value:expr, $pattern:expr $(,)?) => {
    let regex = Regex::new(&format!("^(?s){}$", $pattern)).unwrap();
    let string = $value.to_string();

    if !regex.is_match(string.as_ref()) {
      panic!(
        "Regex:\n\n{}\n\n…did not match string:\n\n{}",
        regex, string
      );
    }
  };
}

macro_rules! assert_matches {
  ($expression:expr, $( $pattern:pat_param )|+ $( if $guard:expr )? $(,)?) => {
    match $expression {
      $( $pattern )|+ $( if $guard )? => {}
      left => panic!(
        "assertion failed: (left ~= right)\n  left: `{:?}`\n right: `{}`",
        left,
        stringify!($($pattern)|+ $(if $guard)?)
      ),
    }
  }
}

pub(crate) fn blockhash(n: u64) -> BlockHash {
  let hex = format!("{n:x}");

  if hex.is_empty() || hex.len() > 1 {
    panic!();
  }

  hex.repeat(64).parse().unwrap()
}

pub(crate) fn txid(n: u64) -> Txid {
  let hex = format!("{n:x}");

  if hex.is_empty() || hex.len() > 1 {
    panic!();
  }

  hex.repeat(64).parse().unwrap()
}

pub(crate) fn outpoint(n: u64) -> OutPoint {
  format!("{}:{}", txid(n), n).parse().unwrap()
}

pub(crate) fn satpoint(n: u64, offset: u64) -> SatPoint {
  SatPoint {
    outpoint: outpoint(n),
    offset,
  }
}

pub(crate) fn address() -> Address {
  "qc1p6p8u0j2z4p339suwg5dygkfxjxft5nkuxrmh23s39rvavlwsmhpqx4fc0w"
    .parse::<Address<NetworkUnchecked>>()
    .unwrap()
    .assume_checked()
}

pub(crate) fn recipient() -> Address {
  "tq1p354czyf0l5rvxujpv74k3wpkyxslvsghqyem3gmcna7j0wpygfuq2clvq6"
    .parse::<Address<NetworkUnchecked>>()
    .unwrap()
    .assume_checked()
}

pub(crate) fn change(n: u64) -> Address {
  match n {
    0 => "tq1pecjdvnwzj8tey2jqes8adn2cn7pvsjdntkfzquy4rncj0pf3cg6qeknkdd",
    1 => "tq1ppj5qwekc6s0ln9a2e47l3mk82htv7dylqegvzwrjgqs8509p7xjsg24ek0",
    2 => "tq1pll5war0scsf697fd6sev39dm9dwg74vphvu7etczpxsmhm35q49s0mqm0q",
    3 => "tq1plq3guwmrw970v60r3harz4c9mv9z8u0xgr2e8cp8ffa8fmr3pr2qs3ssk8",
    _ => panic!(),
  }
  .parse::<Address<NetworkUnchecked>>()
  .unwrap()
  .assume_checked()
}

pub(crate) fn tx_in(previous_output: OutPoint) -> TxIn {
  TxIn {
    previous_output,
    script_sig: ScriptBuf::new(),
    sequence: Sequence::ENABLE_RBF_NO_LOCKTIME,
    witness: Witness::new(),
  }
}

pub(crate) fn tx_out(value: u64, address: Address) -> TxOut {
  TxOut {
    value,
    script_pubkey: address.script_pubkey(),
  }
}

pub(crate) fn inscription(content_type: &str, body: impl AsRef<[u8]>) -> Inscription {
  Inscription::new(Some(content_type.into()), Some(body.as_ref().into()))
}

pub(crate) fn transaction_inscription(
  content_type: &str,
  body: impl AsRef<[u8]>,
  tx_in_index: u32,
  tx_in_offset: u32,
) -> TransactionInscription {
  TransactionInscription {
    inscription: inscription(content_type, body),
    tx_in_index,
    tx_in_offset,
  }
}

pub(crate) fn inscription_id(n: u32) -> InscriptionId {
  let hex = format!("{n:x}");

  if hex.is_empty() || hex.len() > 1 {
    panic!();
  }

  format!("{}i{n}", hex.repeat(64)).parse().unwrap()
}

pub(crate) fn envelope(payload: &[&[u8]]) -> Witness {
  let mut builder = script::Builder::new()
    .push_opcode(opcodes::OP_FALSE)
    .push_opcode(opcodes::all::OP_IF);

  for data in payload {
    let mut buf = PushBytesBuf::new();
    buf.extend_from_slice(data).unwrap();
    builder = builder.push_slice(buf);
  }

  let script = builder.push_opcode(opcodes::all::OP_ENDIF).into_script();

  Witness::from_slice(&[script.into_bytes(), Vec::new()])
}
