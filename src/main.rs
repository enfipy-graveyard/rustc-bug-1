use primitives::Public;
use sr_primitives::{
	traits::{IdentifyAccount, Verify},
	MultiSignature,
};

pub type Account = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;

pub fn call<TPublic: Public>(val: &str) -> String {
	val.to_owned()
}

fn main() {
	let result = call::<Account>("It doesn't work");
	println!("{}", result);
}
