use ethers::prelude::*;
use mev_rs::utils::*;
use std::str::FromStr;

#[test]
fn test_get_univ2_factory_address() {
    let factory = get_univ2_factory_address().unwrap();
    assert_eq!(
        factory,
        Address::from_str("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f").unwrap()
    );
}
