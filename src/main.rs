mod data;
mod document;
mod line;

const MAX_PROJECTS: usize = 5;

fn main() {
    let company = prompted::input!("Type Company Name: ");
    let tool = prompted::input!("Primary Tool: "); // TODO: less rudamentary

    println!("Reading data...");
    let data = data::Data::new(
        &std::fs::read_to_string("data.json")
            .expect("Failed to read data.json file.")
    ).expect("Failed to parse data.json data.");

    println!("Crafting document...");
    let title = format!(
        "{} {} Resume",
        data
            .personal.as_ref().expect("no personal in data")
            .name.as_ref().expect("no name in personal"),
        &company,
    );
    let mut doc = document::init(&title);
    document::personal_header(
        &mut doc, 
        data.personal.as_ref().expect("no personal in data")
    );
    doc.push(genpdf::elements::Break::new(1));

    document::heading(&mut doc, "WORK EXPERIENCE");
    for job in data.jobs.as_ref().expect("no jobs in data") {
        doc.push(genpdf::elements::Break::new(0));
        document::job_paragraph(&mut doc, job);
    }
    doc.push(genpdf::elements::Break::new(1));

    let mut project_count: usize = 0;
    document::heading(&mut doc, "WORKS AND PROJECTS");
    for project in data.projects.as_ref().expect("no projects in data") {
        if project_count >= MAX_PROJECTS {
            break;
        } // TODO: some better choosing than arbitrary first five (or decide fine)
        if &project.tool.as_ref().expect("no tool for project").to_lowercase()
            != &tool.to_lowercase() {
            continue;
        }
        doc.push(genpdf::elements::Break::new(0));
        document::project_paragraph(&mut doc, project);
        project_count += 1;
    }
    doc.push(genpdf::elements::Break::new(1));
    
    document::heading(&mut doc, "EDUCATION");
    for edu in data.education.as_ref().expect("no education in data") {
        doc.push(genpdf::elements::Break::new(0));
        document::education_paragraph(&mut doc, edu);
    }
    doc.push(genpdf::elements::Break::new(1));

    document::heading(&mut doc, "TECHNICAL SKILLS");
    document::skills_paragraph(
        &mut doc,
        data.skills.as_ref().expect("no skills in data"),
    );
    doc.push(genpdf::elements::Break::new(1));

    document::heading(&mut doc, "PUBLICATIONS");
    for publication in data.publications.as_ref().expect("no publications in data") {
        doc.push(genpdf::elements::Break::new(0));
        document::publication_paragraph(&mut doc, publication);
    }

    println!("Rendering document...");
    doc
        .render_to_file(format!("{}.pdf", title))
        .expect("Failed to write PDF file");

    println!("Completed!");
}