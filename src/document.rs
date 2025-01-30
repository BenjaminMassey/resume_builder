use genpdf::Element;

pub fn init(title: &str) -> genpdf::Document {
    let font_family = genpdf::fonts::from_files("./fonts", "RobotoMono", None)
        .expect("Failed to load font family");
    let mut doc = genpdf::Document::new(font_family);
    doc.set_title(title);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    doc
} // TODO: real title and decoration

pub fn personal_header(doc: &mut genpdf::Document, personal: &crate::data::Personal) {
    doc.push(
        genpdf::elements::Text::new(
            format!(
                "{}",
                personal.name.as_ref().expect("no name"),
            )
        )
    ); // TODO: format fancier
    doc.push(genpdf::elements::Break::new(0));
    doc.push(
        genpdf::elements::Text::new(
            format!(
                "{} | {} | {}",
                personal.email.as_ref().expect("no email"),
                personal.phone.as_ref().expect("no phone"),
                personal.website.as_ref().expect("no website"),
            )
        )
    )
}

pub fn job_paragraph(doc: &mut genpdf::Document, job: &crate::data::Job) {
    let header = format!(
        "{}: {} ({})",
        job.title.as_ref().expect("no job title"),
        job.employer.as_ref().expect("no job title"),
        job.duration.as_ref().expect("no job duration"),
    );
    doc.push(genpdf::elements::Text::new(&header));
    doc.push(crate::line::Line {
        width: header.len() as f32 * 2.55,
        color: genpdf::style::Color::Rgb(0, 0, 0)
    });
    for item in job.body.as_ref().expect("no job body elements") {
        doc.push(genpdf::elements::Break::new(0));
        doc.push(
            genpdf::elements::BulletPoint::new(
                genpdf::elements::Text::new(format!("{}", item))
            )
        );
    }
}


pub fn project_paragraph(doc: &mut genpdf::Document, project: &crate::data::Project) {
    let header = format!(
        "{} ({})",
        project.title.as_ref().expect("no project title"),
        project.tool.as_ref().expect("no project tool"),
    );
    doc.push(genpdf::elements::Text::new(&header));
    doc.push(crate::line::Line {
        width: header.len() as f32 * 2.55,
        color: genpdf::style::Color::Rgb(0, 0, 0)
    });
    doc.push(genpdf::elements::Break::new(0));
    doc.push(
        genpdf::elements::Text::new(
            format!(
                "{}",
                project.link.as_ref().expect("no project link"),
            )
        )
    );
    doc.push(genpdf::elements::Break::new(0));
    for item in project.body.as_ref().expect("no project body elements") {
        doc.push(genpdf::elements::Break::new(0));
        doc.push(
            genpdf::elements::BulletPoint::new(
                genpdf::elements::Text::new(format!("{}", item))
            )
        );
    }
}