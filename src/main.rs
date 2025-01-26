mod data;
mod document;

fn main() {
    println!("Reading data...");
    let data = data::Data::new(
        &std::fs::read_to_string("data.json")
            .expect("Failed to read data.json file.")
    ).expect("Failed to parse data.json data.");

    println!("Crafting document...");
    let mut doc = document::init();
    document::personal_header(
        &mut doc, 
        data.personal.as_ref().expect("no personal in data")
    );
    doc.push(genpdf::elements::Break::new(1));
    for job in data.jobs.as_ref().expect("no jobs in data") {
        doc.push(genpdf::elements::Break::new(0));
        document::job_paragraph(&mut doc, job);
    }
    // TODO: more

    println!("Rendering document...");
    doc.render_to_file("output.pdf").expect("Failed to write PDF file");

    println!("Completed!");
}