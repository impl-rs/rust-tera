pub mod context;
pub mod filter;
pub mod templates;

use tera::Error;

use crate::context::{get_context, User};
use crate::filter::markdown_to_html;
use crate::templates::get_tera_with_file_in_binary;

fn main() -> Result<(), Error> {
    let mut tera = get_tera_with_file_in_binary()?;
    tera.register_filter("markdown", markdown_to_html);

    let context = get_context("John Doe".into(), 42, true);
    let html = tera.render("user.html", &context)?;
    dbg!(html);

    let context = User::new("John Doe".into(), 42, true).get_context()?;
    let html = tera.render("user.html", &context)?;
    dbg!(html);

    let context = User::new("John Doe".into(), 42, true).get_context()?;
    let txt = tera.render("hello-world.txt", &context)?;
    dbg!(txt);

    Ok(())
}
