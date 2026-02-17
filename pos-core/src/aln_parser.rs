use regex::Regex;

pub enum AlnCommand {
    PosInit { session: String },
    PosTxn { session: String, amount: String },
    SgcLoan { session: String, amount: String },
    SgcStake { session: String, amount: String, days: String },
    Unknown,
}

pub fn parse_command(input: &str) -> AlnCommand {
    let re_init =
        Regex::new(r"^POS_INIT_([A-Z0-9]{8})$").unwrap();
    let re_txn =
        Regex::new(r"^POS_TXN_([A-Z0-9]{8})_([0-9.]+)_SGC$").unwrap();
    let re_loan =
        Regex::new(r"^SGC_LOAN_([A-Z0-9]{8})_([0-9.]+)$").unwrap();
    let re_stake =
        Regex::new(r"^SGC_STAKE_([A-Z0-9]{8})_([0-9.]+)_([0-9]+)$").unwrap();

    if let Some(caps) = re_init.captures(input) {
        return AlnCommand::PosInit {
            session: caps[1].to_string(),
        };
    }
    if let Some(caps) = re_txn.captures(input) {
        return AlnCommand::PosTxn {
            session: caps[1].to_string(),
            amount: caps[2].to_string(),
        };
    }
    if let Some(caps) = re_loan.captures(input) {
        return AlnCommand::SgcLoan {
            session: caps[1].to_string(),
            amount: caps[2].to_string(),
        };
    }
    if let Some(caps) = re_stake.captures(input) {
        return AlnCommand::SgcStake {
            session: caps[1].to_string(),
            amount: caps[2].to_string(),
            days: caps[3].to_string(),
        };
    }
    AlnCommand::Unknown
}
