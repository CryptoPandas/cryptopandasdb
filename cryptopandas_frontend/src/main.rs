#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

use actix_web::web;
use actix_web::{App, HttpResponse, HttpServer};
use actix_files::Files;

use panda_base::traits::*;
use panda_base::rendering::*;

use handlebars::Handlebars;

use std::io;

mod hb_helpers;


#[get("/")]
fn index(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
    });
    let body = hb_helpers::debug_hb(hb).render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/pandas/")]
fn pandas(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
    });
    let body = hb_helpers::debug_hb(hb).render("pandas", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/breeders/")]
fn breeders(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
    });
    let body = hb_helpers::debug_hb(hb).render("breeders", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/halloffame/")]
fn halloffame(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
    });
    let body = hb_helpers::debug_hb(hb).render("halloffame", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[get("/mating/")]
fn mating(hb: web::Data<Handlebars>) -> HttpResponse {
    let data = json!({
    });
    let body = hb_helpers::debug_hb(hb).render("mating", &data).unwrap();

    HttpResponse::Ok().body(body)
}



#[get("/user/{user}/{data}")]
fn user(hb: web::Data<Handlebars>, info: web::Path<(String, String)>) -> HttpResponse {
    let data = json!({
        "user": info.0,
        "data": info.1
    });
    let body = hb_helpers::debug_hb(hb).render("user", &data).unwrap();

    HttpResponse::Ok().body(body)
}

/// Example templating handler
#[get("/panda/{txid}")]
fn panda_by_id(hb: web::Data<Handlebars>, txid: web::Path<String>) -> HttpResponse {
    // TODO: Get from database
    let panda_attribute = PandaAttributes {
        physique: PhysiqueTrait::SmallFace,
        pattern: PatternTrait::Stripes,
        eye_color: EyeColorTrait::Thundergrey,
        eye_shape: EyeShapeTrait::Caffeine,
        base_color: BaseColorTrait::Harbourfog,
        highlight_color: HighlightColorTrait::Lemonade,
        accent_color: AccentColorTrait::Belleblue,
        wild_element: WildElementTrait::ThirdEye,
        mouth: MouthTrait::Walrus,
    };
    let data = serde_json::to_value(panda_attribute).unwrap();
    let body = hb_helpers::debug_hb(hb).render("panda", &data).unwrap();

    HttpResponse::Ok().body(body)
}


fn main() -> io::Result<()> {
	let panda_attribute = PandaAttributes {
		  physique: PhysiqueTrait::SmallFace,
		  pattern: PatternTrait::Stripes,
		  eye_color: EyeColorTrait::Thundergrey,
		  eye_shape: EyeShapeTrait::Caffeine,
		  base_color: BaseColorTrait::Harbourfog,
		  highlight_color: HighlightColorTrait::Lemonade,
		  accent_color: AccentColorTrait::Belleblue,
		  wild_element: WildElementTrait::ThirdEye,
		  mouth: MouthTrait::Walrus,
	};


	let r = panda_base::rendering::render_panda(&panda_attribute);
	if (r.is_ok()) {
		print!("ok!");
	}

    let mut handlebars = Handlebars::new();
    hb_helpers::configure_handlebars(&mut handlebars);
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .register_data(handlebars_ref.clone())
            .service(index)
            .service(user)
            .service(pandas)
            .service(breeders)
            .service(halloffame)
            .service(mating)
            .service(panda_by_id)
	    // If no service, try to find a static file and to serve it
	    .default_service(
			    Files::new("", "./static/")
			    )
    })
        .bind("127.0.0.1:8080")?
        .run()
}
