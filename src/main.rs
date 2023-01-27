use tera::{Context, Error, Tera};

fn get_tera_with_globbing() -> Tera {
    let Ok(tera) = Tera::new("templates/**/*.html") else {
        panic!("There was an error loading the templates");
    };
    tera
}

fn get_tera_with_raw_template() -> Tera {
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
    );
    tera
}

fn get_tera_with_file_in_binary() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_template("strings.html", include_str!("./templates/strings.html"));
    tera
}

fn main() -> Result<(), Error> {
    Ok(())
}
