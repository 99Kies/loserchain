use pallet_evm::{Context, Precompile, PrecompileResult, PrecompileSet};
use sp_core::H160;
use sp_std::marker::PhantomData;

pub struct LoserPrecompiles<R>(PhantomData<R>);

impl<R> LoserPrecompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}

	pub fn used_address() -> sp_std::vec::Vec<H160> {
		sp_std::vec![]
	}
}

impl<R> PrecompileSet for LoserPrecompiles<R>
where
	R: pallet_evm::Config,
{
	fn execute(
		&self,
		address: H160,
		_input: &[u8],
		_gas_limit: Option<u64>,
		_context: &Context,
		_is_static: bool,
	) -> Option<PrecompileResult> {
		match address {
			// TODO: should add more precompile function to expand the runtime
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160) -> bool {
		Self::used_address().contains(&address)
	}
}
