use frame_support::{
	dispatch::{Dispatchable, GetDispatchInfo, PostDispatchInfo},
};
use pallet_evm::{AddressMapping, Precompile, ExitError, ExitSucceed};
use precompile_utils::{
	EvmDataReader, EvmDataWriter, EvmResult, Gasometer, RuntimeHelper,
};

use fp_evm::{Context, PrecompileOutput};

use sp_std::{
	fmt::Debug,
	marker::PhantomData,
};

/// Each variant represents a method that is exposed in the public Solidity interface
/// The function selectors will be automatically generated at compile-time by the macros
#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq, num_enum::TryFromPrimitive)]
enum Action {
	DoSomething = "do_something(uint256)",
	GetValue = "get_value()",
}

pub struct PalletTemplatePrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> Precompile for PalletTemplatePrecompile<Runtime> 
where
	Runtime: pallet_template::Config + pallet_evm::Config,
	Runtime::Call: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::Call as Dispatchable>::Origin: From<Option<Runtime::AccountId>>,
	Runtime::Call: From<pallet_template::Call<Runtime>>,
{
	fn execute(
		input: &[u8], //Reminder this is big-endian
		target_gas: Option<u64>,
		context: &Context,
	) -> Result<PrecompileOutput, ExitError> {
		let mut input = EvmDataReader::new(input);

		match &input.read_selector()? {
			// Check for accessor methods first. These return results immediately
			Action::DoSomething => Self::do_something(input, target_gas, context),
			Action::GetValue => Self::get_value(input, target_gas, context),
		}
	}
}

impl<Runtime> PalletTemplatePrecompile<Runtime>
where
	Runtime: pallet_template::Config + pallet_evm::Config,
	Runtime::Call: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::Call as Dispatchable>::Origin: From<Option<Runtime::AccountId>>,
	Runtime::Call: From<pallet_template::Call<Runtime>>,
{
	fn do_something(
		mut input: EvmDataReader,
		target_gas: Option<u64>,
		context: &Context,
	) -> EvmResult<PrecompileOutput> {
		// This gasometer is a handy utility that will help us account for gas as we go.
		let mut gasometer = Gasometer::new(target_gas);

		// Bound check. We expect a single argument passed in.
		input.expect_arguments(1)?;

		// Parse the u32 value that will be dispatched to the pallet.
		let value = input.read::<u32>()?.into();

		// Use pallet-evm's account mapping to determine what AccountId to dispatch from.
		let origin = Runtime::AddressMapping::into_account_id(context.caller);
		let call =
			pallet_template::Call::<Runtime>::do_something{something: value};

		// Dispatch the call into the runtime.
		// The RuntimeHelper tells how much gas was actually used.
		let used_gas = RuntimeHelper::<Runtime>::try_dispatch(
			Some(origin).into(),
			call,
			gasometer.remaining_gas()?,
		)?;

		// Record the gas used in the gasometer
		gasometer.record_cost(used_gas)?;

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Stopped,
			cost: gasometer.used_gas(),
			output: Default::default(),
			logs: Default::default(),
		})
	}

	fn get_value(
		input: EvmDataReader,
		target_gas: Option<u64>,
		_context: &Context,
	) -> EvmResult<PrecompileOutput> {
		let mut gasometer = Gasometer::new(target_gas);

		// Bound check
		input.expect_arguments(0)?;

		// fetch data from pallet
		let stored_value = pallet_template::Something::<Runtime>::get().unwrap_or_default();

		// Record one storage red worth of gas.
		// The utility internally uses pallet_evm's GasWeightMapping.
		gasometer.record_cost(RuntimeHelper::<Runtime>::db_read_gas_cost())?;

		// Construct to Solidity-formatted output data
		let output = EvmDataWriter::new().write(stored_value).build();

		Ok(PrecompileOutput {
			exit_status: ExitSucceed::Returned,
			cost: gasometer.used_gas(),
			output,
			logs: Default::default(),
		})
	}
}


// TODO Mock runtime
// TODO tests
// See Moonbeam for examples https://github.com/PureStake/moonbeam/tree/master/precompiles