const PIXELS_PER_CHAR: f32 = 2.1;
const CHARS_PER_PAGE: f32 = 90.5;

pub fn init(title: &str) -> genpdf::Document {
    let font_family = genpdf::fonts::from_files("./fonts", "RobotoMono", None)
        .expect("Failed to load font family");
    let mut doc = genpdf::Document::new(font_family);
    doc.set_font_size(10u8);
    doc.set_title(title);
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);
    doc
} // TODO: real title and decoration

pub fn personal_header(doc: &mut genpdf::Document, personal: &crate::data::Personal) {
    doc.push(crate::line::Line {
        width: CHARS_PER_PAGE as f32 * PIXELS_PER_CHAR,
        color: genpdf::style::Color::Rgb(0, 0, 0)
    });
    let name = format!(
        "{}",
        personal.name.as_ref().expect("no name"),
    );
    let info = format!(
        "{} | {} | {}",
        personal.email.as_ref().expect("no email"),
        personal.phone.as_ref().expect("no phone"),
        personal.website.as_ref().expect("no website"),
    );
    let mut name_section = genpdf::elements::Paragraph::default();
    name_section.set_alignment(genpdf::Alignment::Center);
    let mut name_style = genpdf::style::Style::new();
    name_style.set_bold();
    name_style.set_font_size(14u8);
    name_section.push_styled(&name.to_uppercase(), name_style);
    doc.push(name_section);
    let mut info_section = genpdf::elements::Paragraph::default();
    info_section.set_alignment(genpdf::Alignment::Center);
    info_section.push(&info);
    doc.push(info_section);
    doc.push(crate::line::Line {
        width: CHARS_PER_PAGE as f32 * PIXELS_PER_CHAR,
        color: genpdf::style::Color::Rgb(0, 0, 0)
    });
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
        width: header.len() as f32 * PIXELS_PER_CHAR,
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
    ); // TODO: better than tool, if all one tool
    doc.push(genpdf::elements::Text::new(&header));
    doc.push(crate::line::Line {
        width: header.len() as f32 * PIXELS_PER_CHAR,
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

pub fn education_paragraph(doc: &mut genpdf::Document, edu: &crate::data::Education) {
    let header = format!(
        "{}, {} ({})",
        edu.title.as_ref().expect("no edu title"),
        edu.location.as_ref().expect("no edu location"),
        edu.duration.as_ref().expect("no edu duration"),
    );
    doc.push(genpdf::elements::Text::new(&header));
    doc.push(crate::line::Line {
        width: header.len() as f32 * PIXELS_PER_CHAR,
        color: genpdf::style::Color::Rgb(0, 0, 0)
    });
    doc.push(genpdf::elements::Break::new(0));
    for item in edu.body.as_ref().expect("no edu body elements") {
        doc.push(genpdf::elements::Break::new(0));
        doc.push(
            genpdf::elements::BulletPoint::new(
                genpdf::elements::Text::new(format!("{}", item))
            )
        );
    }
}

pub fn skills_paragraph(doc: &mut genpdf::Document, skills: &crate::data::Skills) {
    for (label, set) in [
        ("Languages", skills.languages.as_ref().expect("no languages in skills")),
        ("Tools", skills.tools.as_ref().expect("no tools in skills")), 
        ("Topics", skills.topics.as_ref().expect("no topics in skills")),
    ] { // TODO: some limiter for number of skills
        let text = format!(
            "{}: {}",
            &label,
            set.join(", "),
        );
        doc.push(genpdf::elements::Text::new(&text));
        doc.push(crate::line::Line {
            width: label.len() as f32 * PIXELS_PER_CHAR,
            color: genpdf::style::Color::Rgb(0, 0, 0)
        });
        doc.push(genpdf::elements::Break::new(0));
    }
}

pub fn publication_paragraph(
    doc: &mut genpdf::Document,
    publication: &crate::data::Publication,
) {
    let header = format!(
        "{} ({})",
        publication.title.as_ref().expect("no publication title"),
        publication.role.as_ref().expect("no publication role"),
    );
    doc.push(genpdf::elements::Text::new(&header));
    doc.push(crate::line::Line {
        width: header.len() as f32 * PIXELS_PER_CHAR,
        color: genpdf::style::Color::Rgb(0, 0, 0)
    });
    doc.push(genpdf::elements::Break::new(0));
    doc.push(
        genpdf::elements::Text::new(
            format!(
                "{}",
                publication.link.as_ref().expect("no publication link"),
            )
        )
    );
    doc.push(genpdf::elements::Break::new(0));
    for item in publication.body.as_ref().expect("no publication body elements") {
        doc.push(genpdf::elements::Break::new(0));
        doc.push(
            genpdf::elements::BulletPoint::new(
                genpdf::elements::Text::new(format!("{}", item))
            )
        );
    }
}

pub fn heading(doc: &mut genpdf::Document, text: &str) {
    let mut section = genpdf::elements::Paragraph::default();
    section.set_alignment(genpdf::Alignment::Center);
    section.push_styled(text, genpdf::style::Effect::Bold);
    doc.push(section);
    doc.push(crate::line::Line {
        width: CHARS_PER_PAGE as f32 * PIXELS_PER_CHAR,
        color: genpdf::style::Color::Rgb(0, 0, 0)
    });
}