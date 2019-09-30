use actix_web::web;
use handlebars::{Handlebars, TemplateFileError, RenderError, RenderContext, Helper, Output, HelperResult, Context, Renderable};
use panda_base::traits::*;
use std::fs::{File, OpenOptions};
use std::fs;
use std::io::Write;
use std::io::prelude::*;
use std::io;
use std::os::unix;
use std::path::Path;
use std::process::Command;


// Allows the templates to be reloaded each time a webpage is accessed
// (else its TOO MUCH painfull)
pub fn debug_hb(hb: web::Data<Handlebars>) -> web::Data<Handlebars> {
	const DEBUG_HANDLEBARS: bool = true;

	if DEBUG_HANDLEBARS {
		// Reload the data...
		let mut handlebars = Handlebars::new();
		configure_handlebars(&mut handlebars);

		return web::Data::new(handlebars);
	} else {
		return hb;
	}
}

fn debug(h: &Helper, _hbars: &Handlebars, ctx: &Context, _rc: &mut RenderContext, _out: &mut dyn Output) -> HelperResult {
	print!("==============================\n");
	print!("Context {}\n", ctx.data().to_string());
	let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
	print!("Value: {}\n", param);
	print!("==============================\n");
	io::stdout().flush().ok().expect("Could not flush stdout");

	Ok(())
}

fn set_variable(h: &Helper, _hbars: &Handlebars, _ctx: &Context, rc: &mut RenderContext, _out: &mut dyn Output) -> HelperResult {
	let key = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("").to_string();
	let value = h.param(1).and_then(|v| v.value().as_str()).unwrap_or("");

	rc.set_local_var(key, json!(value));
	rc.promote_local_vars();

	Ok(())
}

pub fn if_modulo<'reg, 'rc>(helper: &Helper<'reg, 'rc>, hbars: &'reg Handlebars, ctx: &'rc Context,
		render_ctx: &mut RenderContext<'reg>, out: &mut dyn Output) -> Result<(), RenderError> {

	const MODULO_VALUE_IDX: usize = 0;
	const FACTOR_OF_INTEREST_IDX: usize = 1;
	const CANDIDATE_IDX: usize = 2;

	let modulo_value =
		helper.param(MODULO_VALUE_IDX)
		.map(|json| json.value())
		.and_then(|val| val.as_u64())
		.ok_or_else(|| RenderError::new("Modulo is not the correct type."))?;

	let factor_of_interest =
		helper.param(FACTOR_OF_INTEREST_IDX)
		.map(|json| json.value())
		.and_then(|val| val.as_u64())
		.and_then(|u64_val| if u64_val > 0 { Some(u64_val) } else { None } )
		.ok_or_else(|| RenderError::new("Factor of interest must be a number greater than 0."))?;

	let candidate = 
		helper.param(CANDIDATE_IDX)
		.map(|json| json.value())
		.and_then(|val| val.as_u64())
		.ok_or_else(|| RenderError::new("Candidate must be a number greater than or equal to 0."))?;

	let possible_template = if (candidate) % factor_of_interest == modulo_value {
		helper.template()
	} else {
		helper.inverse()
	};

	match possible_template {
		Some(t) => t.render(hbars, ctx, render_ctx, out),
			None => Ok(()),
	}
}


pub fn configure_handlebars(hb: &mut Handlebars ) {
	// TODO: Error handling
	hb.register_helper("debug", Box::new(debug));
	hb.register_helper("set_variable", Box::new(set_variable));
	hb.register_helper("if_modulo", Box::new(if_modulo));

	hb.register_templates_directory(".hbs", "./static/templates");
	hb.register_templates_directory(".html", "./static/pages");
}

