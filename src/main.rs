mod data;
mod lab1;
mod render;

fn main() {
    render::render(
        data::VALUES,
        Some("lab1"),
        "values",
        "Дані про денні продажі товарів в магазині 9",
    );
    lab1::analyze();
}
