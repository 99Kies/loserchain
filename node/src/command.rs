use crate::{
	chain_spec,
	cli::{Cli, Subcommand},
	service,
};
use loserchain_runtime::Block;
use sc_cli::{ChainSpec, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Substrate Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"support.anonymous.an".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"dev" => Box::new(chain_spec::development_config()?),
			"" | "local" => Box::new(chain_spec::local_testnet_config()?),
			path =>
				Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		})
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&loserchain_runtime::VERSION
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					service::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = service::new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = service::new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					service::new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, .. } =
					service::new_partial(&config)?;
				Ok((cmd.run(client, backend), task_manager))
			})
		},
		// Some(Subcommand::ExportGenesisState(params)) => {
		// 	let mut builder = sc_cli::LoggerBuilder::new("");
		// 	builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
		// 	let _ = builder.init();
		//
		// 	let spec = load_spec(&params.chain.clone().unwrap_or_default())?;
		// 	let state_version = Cli::native_runtime_version(&spec).state_version();
		// 	let block: Block = generate_genesis_block(&spec, state_version)?;
		// 	let raw_header = block.header().encode();
		// 	let output_buf = if params.raw {
		// 		raw_header
		// 	} else {
		// 		format!("0x{:?}", HexDisplay::from(&block.header().encode())).into_bytes()
		// 	};
		//
		// 	if let Some(output) = &params.output {
		// 		std::fs::write(output, output_buf)?;
		// 	} else {
		// 		std::io::stdout().write_all(&output_buf)?;
		// 	}
		//
		// 	Ok(())
		// }
		// Some(Subcommand::ExportGenesisWasm(params)) => {
		// 	let mut builder = sc_cli::LoggerBuilder::new("");
		// 	builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
		// 	let _ = builder.init();
		//
		// 	let raw_wasm_blob =
		// 		extract_genesis_wasm(&cli.load_spec(&params.chain.clone().unwrap_or_default())?)?;
		// 	let output_buf = if params.raw {
		// 		raw_wasm_blob
		// 	} else {
		// 		format!("0x{:?}", HexDisplay::from(&raw_wasm_blob)).into_bytes()
		// 	};
		//
		// 	if let Some(output) = &params.output {
		// 		std::fs::write(output, output_buf)?;
		// 	} else {
		// 		std::io::stdout().write_all(&output_buf)?;
		// 	}
		//
		// 	Ok(())
		// },
		Some(Subcommand::Benchmark(cmd)) =>
			if cfg!(feature = "runtime-benchmarks") {
				let runner = cli.create_runner(cmd)?;

				runner.sync_run(|config| cmd.run::<Block, service::ExecutorDispatch>(config))
			} else {
				Err("Benchmarking wasn't enabled when building the node. You can enable it with \
				     `--features runtime-benchmarks`."
					.into())
			},
		None => {
			let runner = cli.create_runner(&cli.run)?;
			runner.run_node_until_exit(|config| async move {
				service::new_full(config).map_err(sc_cli::Error::Service)
			})
		},
	}
}
