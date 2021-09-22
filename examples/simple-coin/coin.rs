// use orga::client::Client;
use core::pin::Pin;
use orga::client::AsyncCall;
use orga::client::Client;
use orga::coins::*;
use orga::contexts::load_keypair;
use orga::encoding::{Decode, Encode, Terminated};
use orga::prelude::*;
use std::future::Future;

#[derive(Encode, Decode)]
pub struct Simp;
impl Symbol for Simp {}

#[derive(State, Call, Client, Query)]
pub struct SimpleCoin {
    balances: Map<Address, Coin<Simp>>,
}

impl InitChain for SimpleCoin {
    fn init_chain(&mut self, ctx: &InitChainCtx) -> Result<()> {
        let my_address = load_keypair().unwrap().public.to_bytes();
        println!("my address: {:?}", my_address);
        self.balances.insert(my_address, Simp::mint(100))?;
        Ok(())
    }
}
impl BeginBlock for SimpleCoin {
    fn begin_block(&mut self, ctx: &BeginBlockCtx) -> Result<()> {
        for entry in self.balances.iter()? {
            let (key, balance) = entry?;
            println!("{:?} has {}", *key, balance.amount.value);
        }
        println!("\n\n\n");
        // self.balances.insert()
        // self.balances.
        Ok(())
    }
}

impl SimpleCoin {
    #[call]
    pub fn transfer(&mut self, to: Address, amount: Amount<Simp>) -> Result<()> {
        let signer = self
            .context::<Signer>()
            .ok_or_else(|| failure::format_err!("No signer context available"))?
            .signer
            .ok_or_else(|| failure::format_err!("Transfer calls must be signed"))?;

        let mut sender = self.balances.entry(signer)?.or_default()?;
        let coins = sender.take(amount)?;
        let mut receiver = self.balances.entry(to)?.or_default()?;
        receiver.give(coins).unwrap();

        Ok(())
    }

    pub fn balances(&mut self) -> &mut Map<Address, Coin<Simp>> {
        &mut self.balances
    }
}