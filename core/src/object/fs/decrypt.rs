use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::PathBuf;
// use tokio::fs::File;

use crate::{
	invalidate_query,
	job::{
		JobError, JobInitData, JobReportUpdate, JobResult, JobState, StatefulJob, WorkerContext,
	},
	// util::error::FileIOError,
};

use super::{context_menu_fs_info, FsInfo};
pub struct FileDecryptorJob;
#[derive(Serialize, Deserialize, Debug)]
pub struct FileDecryptorJobState {}

// decrypt could have an option to restore metadata (and another specific option for file name? - would turn "output file" into "output path" in the UI)
#[derive(Serialize, Deserialize, Debug, Type, Hash)]
pub struct FileDecryptorJobInit {
	pub location_id: i32,
	pub path_id: i32,
	pub mount_associated_key: bool,
	pub output_path: Option<PathBuf>,
	pub password: Option<String>, // if this is set, we can assume the user chose password decryption
	pub save_to_library: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileDecryptorJobStep {
	pub fs_info: FsInfo,
}

impl JobInitData for FileDecryptorJobInit {
	type Job = FileDecryptorJob;
}

#[async_trait::async_trait]
impl StatefulJob for FileDecryptorJob {
	type Init = FileDecryptorJobInit;
	type Data = FileDecryptorJobState;
	type Step = FileDecryptorJobStep;

	const NAME: &'static str = "file_decryptor";

	fn new() -> Self {
		Self {}
	}

	async fn init(&self, ctx: WorkerContext, state: &mut JobState<Self>) -> Result<(), JobError> {
		// enumerate files to decrypt
		// populate the steps with them (local file paths)
		let fs_info =
			context_menu_fs_info(&ctx.library.db, state.init.location_id, state.init.path_id)
				.await?;

		state.steps.push_back(FileDecryptorJobStep { fs_info });

		ctx.progress(vec![JobReportUpdate::TaskCount(state.steps.len())]);

		Ok(())
	}

	async fn execute_step(
		&self,
		_ctx: WorkerContext,
		_state: &mut JobState<Self>,
	) -> Result<(), JobError> {
		// let info = &&state.steps[0].fs_info;
		// let key_manager = &ctx.library.key_manager;

		// handle overwriting checks, and making sure there's enough available space
		// let output_path = state.init.output_path.clone().map_or_else(
		// 	|| {
		// 		let mut path = info.fs_path.clone();
		// 		let extension = path.extension().map_or("decrypted", |ext| {
		// 			if ext == BYTES_EXT {
		// 				""
		// 			} else {
		// 				"decrypted"
		// 			}
		// 		});
		// 		path.set_extension(extension);
		// 		path
		// 	},
		// 	|p| p,
		// );

		// let mut reader = File::open(info.fs_path.clone())
		// 	.await
		// 	.map_err(|e| FileIOError::from((&info.fs_path, e)))?;
		// let mut writer = File::create(&output_path)
		// 	.await
		// 	.map_err(|e| FileIOError::from((output_path, e)))?;

		// let header = FileHeader::from_reader_async(&mut reader, ENCRYPTED_FILE_MAGIC_BYTES).await?;

		// let keys = key_manager.enumerate_hashed_keys();
		// let master_key = header.decrypt_master_key(keys, FILE_KEYSLOT_CONTEXT)?;

		// let decryptor = Decryptor::new(master_key, header.get_nonce(), header.get_algorithm())?;

		// decryptor
		// 	.decrypt_streams_async(&mut reader, &mut writer, header.get_aad())
		// 	.await?;

		// // need to decrypt preview media/metadata, and maybe add an option in the UI so the user can chosoe to restore these values
		// // for now this can't easily be implemented, as we don't know what the new object id for the file will be (we know the old one, but it may differ)

		// ctx.progress(vec![JobReportUpdate::CompletedTaskCount(
		// 	state.step_number + 1,
		// )]);

		// Ok(())
		todo!()
	}

	async fn finalize(&mut self, ctx: WorkerContext, state: &mut JobState<Self>) -> JobResult {
		invalidate_query!(ctx.library, "search.paths");

		// mark job as successful
		Ok(Some(serde_json::to_value(&state.init)?))
	}
}
