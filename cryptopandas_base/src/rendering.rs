use std::fs::{OpenOptions};
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;
use crate::traits::PandaAttributes;

// Should end with a slash
const DEST_ROOT_PATH : &'static str = "./static/pandas/";

const BLENDER_EXEC_LOCATION : &'static str = "/Applications/Blender.app/Contents/MacOS/Blender";
const SCENE_LOCATION : &'static str = "./blender/blender/panda.blend";

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

// Generate the key folder name for a render
pub fn panda_attribute_to_render_key(panda_attribute: &PandaAttributes) -> String {
	let name=format!("{:x}", panda_attribute.physique as i32)
		  + &format!("{:x}", panda_attribute.pattern as i32)
		  + &format!("{:x}", panda_attribute.eye_color as i32)
		  + &format!("{:x}", panda_attribute.eye_shape as i32)
		  + &format!("{:x}", panda_attribute.base_color as i32)
		  + &format!("{:x}", panda_attribute.highlight_color as i32)
		  + &format!("{:x}", panda_attribute.accent_color as i32)
		  + &format!("{:x}", panda_attribute.wild_element as i32)
		  + &format!("{:x}", panda_attribute.mouth as i32);

	return name;
}

pub fn render_panda_over(panda_attribute: &PandaAttributes) -> bool {

	let dest_folder = DEST_ROOT_PATH.to_owned() + &panda_attribute_to_render_key(&panda_attribute) + "/";
	let done_file = dest_folder.to_string() + &"/.done".to_string();

	return Path::new(&dest_folder).exists() && Path::new(&done_file).exists(); 
}

pub fn render_panda_is_ongoing(panda_attribute: &PandaAttributes) -> bool {

	let dest_folder = DEST_ROOT_PATH.to_owned() + &panda_attribute_to_render_key(&panda_attribute) + "/";

	return Path::new(&dest_folder).exists(); 
}

pub fn render_panda(panda_attribute: &PandaAttributes) -> Result<String, String> {
	let dest_folder = DEST_ROOT_PATH.to_owned() + &panda_attribute_to_render_key(&panda_attribute) + "/";

	if render_panda_is_ongoing(panda_attribute) {
		return Ok(format!("Rendering is on its way: {}\n", dest_folder));
	}

	if render_panda_over(panda_attribute) {
		// Nothing to do
		return Ok(format!("Already rendered: {}\n", dest_folder));
	}

	// Create the folder
	if fs::create_dir(&dest_folder).is_err() {
		return Err(format!("Can t create directory: {}\n", dest_folder));
	}

	let cmd = "dummy=0".to_owned()
		  + &format!("fps=15;nb_frames=90;")
		  + &format!("resolution_x=448;resolution_y=256")
		  + &format!("physique={:x};", panda_attribute.physique as i32)
		  + &format!("pattern={};", panda_attribute.pattern as i32)
		  + &format!("eye_color={};", panda_attribute.eye_color as i32)
		  + &format!("eye_shape={};", panda_attribute.eye_shape as i32)
		  + &format!("base_color={};", panda_attribute.base_color as i32)
		  + &format!("highlight_color={};", panda_attribute.highlight_color as i32)
		  + &format!("accent_color={};", panda_attribute.accent_color as i32)
		  + &format!("wild_element={};", panda_attribute.wild_element as i32)
		  + &format!("mouth={}", panda_attribute.mouth as i32);

	print!("Starting rendering of {}\n", dest_folder);

	let _output = Command::new(BLENDER_EXEC_LOCATION)
			.args(&[
					"-b",
					SCENE_LOCATION,
					"--python-text",
					"'camera_animation'",
					"--python-text",
					"'panda_generation'",
					"-o",
					&(dest_folder.to_string() + &"video".to_string()),
					"-a",
					"-F",
					"PNG",
					"-o",
					&(dest_folder.to_string() + &"picture".to_string()),
					"-f",
					"0",

					"--",
					&cmd,
				])
			.output()
			.expect("failed to execute process");

	// We notify we are done 
	let done_file = dest_folder.to_string() + &"/.done".to_string(); 
	if touch(Path::new(&done_file)).is_err() {
		return Err(format!("Can t touch: {}\n", done_file));
	}

	return Ok(format!("Rendering of {} is complete\n", dest_folder));
}
