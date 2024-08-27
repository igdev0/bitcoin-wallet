use std::{borrow::Borrow, rc::Rc};

use async_std::sync::Mutex;
use bip39::Mnemonic;
use bitcoin::hex::{Case, DisplayHex};
use bitcoin_wallet::{
    account::AccountBuilder,
    path_builder::PathBuilder,
    storage::{self},
    wallet::WalletBuilder,
};
use rand::RngCore;
use rand_core::{self, OsRng};

fn mnemonic_helper() -> Mnemonic {
    let mut entropy = [0u8; 32];
    let mut rng = OsRng;
    rng.fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
    mnemonic
}

#[tokio::test]
async fn create_wallet() {
    let connection = storage::DbFacade::new(Some("sqlite::memory:")).await;
    connection.migrate().await;
    let db = connection.pool;
    let mnemonic = mnemonic_helper();
    let mut wallet = WalletBuilder::new(&mnemonic.to_string());
    wallet.name("Main wallet".to_string());

    let wallet = wallet.build();

    wallet.save(&db).await;
}

#[tokio::test]
async fn load_wallet() {
    let conn_facade = storage::DbFacade::new(Some("sqlite::memory:")).await;
    conn_facade.migrate().await;

    let mnemonic = mnemonic_helper();

    let mut wallet = WalletBuilder::new(&mnemonic.to_string());
    wallet.name("Main wallet".to_string());
    wallet.build().save(&conn_facade.pool).await;

    let wallet = WalletBuilder::from_existing("Main wallet");
    let wallet = wallet.load(&conn_facade.pool).await;

    assert_eq!(wallet.unwrap().name, "Main wallet")
}

#[test]
fn can_build_bip32_path() {
    let path = PathBuilder::new();
    let path = path.build().to_string();

    assert_eq!(path, "49'/0'/0'/0/0");
}
#[test]
fn can_build_account() {
    // Account
    let mnemonic = mnemonic_helper();
    let seed = mnemonic.to_seed("passphrase");
    let mut account_builder = AccountBuilder::new();

    account_builder.seed(&seed.to_hex_string(Case::Lower));

    let account = account_builder.build().unwrap();
    println!("Address: {}", account.address);
    println!("Address length: {}", account.address.len());

    dbg!(&account.address);
}

#[tokio::test]

async fn can_store_accounts_for_wallet() {
    let conn_facade = storage::DbFacade::new(Some("sqlite::memory:")).await;
    conn_facade.migrate().await;

    let mnemonic = mnemonic_helper();

    let mut wallet = WalletBuilder::new(&mnemonic.to_string());
    wallet.name("Main wallet".to_string());
    wallet.build().save(&conn_facade.pool).await;

    let wallet = WalletBuilder::from_existing("Main wallet");

    let wallet = wallet.load(&conn_facade.pool).await.unwrap();

    let account_builder = wallet.create_account();
    let account = account_builder.build().unwrap();
    account.save(&conn_facade.pool).await;

    // Now lets try to load the wallet + accounts
    let wallet = wallet.load(&conn_facade.pool).await.unwrap();
    let accounts_ref = Rc::clone(&wallet.accounts);
    let accounts_len = accounts_ref.borrow_mut().len();
    // println!("{}")
    // let a  = *accounts_ref.borrow();
    // accs.into_inner()
    // let a = accs.borrow();
    assert!(accounts_len > 0);
}
