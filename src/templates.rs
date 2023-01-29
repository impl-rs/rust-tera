use lazy_static::lazy_static;
use tera::{Error, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let Ok(tera) = Tera::new("templates/**/*.html") else {
            panic!("There was an error loading the templates");
        };
        tera
    };
}

pub fn get_tera_with_globbing() -> Tera {
    let Ok(tera) = Tera::new("templates/**/*.html") else {
        panic!("There was an error loading the templates");
    };
    tera
}

pub fn get_tera_with_raw_template() -> Result<Tera, Error> {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "numbers.html",
        "
        <ul>
            {% for number in numbers %}
            <li>
                {{ number }}
            </li>
            {% endfor %}
        </ul>
    ",
    )?;
    Ok(tera)
}

pub fn get_tera_with_file_in_binary() -> Result<Tera, Error> {
    let mut tera = Tera::default();
    tera.add_raw_template("user.html", include_str!("./templates/user.html"))?;
    tera.add_raw_template(
        "hello-world.txt",
        include_str!("./templates/hello-world.txt"),
    )?;
    Ok(tera)
}
