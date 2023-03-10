use super::internal::*;

pub struct SoundPack {
	assets: AssetsDir,
	mixer: Option<Mixer>,
	clips: RefCell<HashMap<String, Arc<Clip>>>,
}

type Clip = Vec<f32>;

impl SoundPack {
	pub fn new(config: &SoundOpts, assets: AssetsDir) -> Result<Self> {
		let mixer = match config.enabled {
			true => Some(Mixer::new(Duration::from_secs(10))?), // {
			//Ok(mixer) => Some(mixer),
			//Err(err) => {
			//	eprintln!("ERROR initializing sound: {}", err);
			//	None
			//}
			//},
			false => None,
		};
		Ok(Self { mixer, assets, clips: default() })
	}

	/// Start playing an audio clip (e.g. "fight") without spatial effects.
	/// Useful for, e.g., the announcer's voice which does not have a physical location.
	pub fn play_raw(&self, clip_name: &str) {
		self.play_raw_volume(clip_name, 1.0)
	}

	/// Start playing an audio clip (e.g. "fight") without spatial effects.
	/// Useful for, e.g., the announcer's voice which does not have a physical location.
	pub fn play_raw_volume(&self, clip_name: &str, volume: f32) {
		if let Some(mixer) = &self.mixer {
			if let Some(clip) = self.clip(clip_name) {
				mixer.play_raw_mono(clip.iter().copied().map(|v| v * volume))
			}
		}
	}

	/// Start playing an audio clip (e.g. "footstep1") with spatial filtering.
	pub fn play_spatial(&self, clip_name: &str, azimuth: f32, volume: f32) {
		if let Some(mixer) = &self.mixer {
			if let Some(clip) = self.clip(clip_name) {
				mixer.play_spatial(azimuth, volume, clip.iter().copied())
			}
		}
	}

	// Get a clip (name without path or extension, e.g. "footstep") from the audio cache,
	// lazily loading it if necessary.
	fn clip(&self, clip_name: &str) -> Option<Arc<Clip>> {
		if !self.clips.borrow().contains_key(clip_name) {
			self.load_clip(clip_name)
		}
		self.clips.borrow().get(clip_name).map(Arc::clone)
	}

	// Load a clip (name without path or extension, e.g. "footstep") into the audio cache.
	fn load_clip(&self, clip_name: &str) {
		let clip_file = self.assets.audio_dir().join(clip_name).with_extension("ogg");
		LOG.write(format!("loading {}", clip_file.to_string_lossy()));
		match decode_44khz_mono_f32(clip_file) {
			Ok(clip) => {
				self.clips.borrow_mut().insert(clip_name.to_owned(), Arc::new(clip));
			}
			Err(err) => {
				println!("error loading sound `{}`: {}", clip_name, err);
				self.clips.borrow_mut().insert(clip_name.to_owned(), Arc::new(Clip::new()));
			}
		}
	}
}
