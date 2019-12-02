#[derive(Clone)]
struct TestStruct;

trait Verify {
	type Signer: Clone;
}

impl Verify for TestStruct {
	type Signer = TestStruct;
}

trait IdentifyAccount {
	type AccountId;
}

impl IdentifyAccount for TestStruct {
	type AccountId = Self;
}

enum MultiSignature {
	Test(TestStruct),
}

impl Verify for MultiSignature {
	type Signer = TestStruct;
}

impl From<TestStruct> for MultiSignature {
	fn from(x: TestStruct) -> Self {
		Self::Test(x)
	}
}

type Account = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;

fn call<Example: Clone>(val: &str) -> String {
	val.to_owned()
}

fn main() {
	let result = call::<Account>("It doesn't work");
	println!("{}", result);
}
