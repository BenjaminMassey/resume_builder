// https://users.rust-lang.org/t/genpdf-print-straight-line-with-colour/92040/4

pub struct Line {
    pub width: f32,
    pub color: genpdf::style::Color,
}
impl genpdf::Element for Line {
    fn render(
        &mut self,
        _: &genpdf::Context,
        area: genpdf::render::Area<'_>,
        style: genpdf::style::Style,
    ) -> Result<genpdf::RenderResult, genpdf::error::Error> {
        area.draw_line(
            vec![
                genpdf::Position {
                    x: 0.into(),
                    y: 1.into(),
                },
                genpdf::Position {
                    x: self.width.into(),
                    y: 1.into(),
                },
            ],
            style.with_color(self.color),
        );

        Ok(genpdf::RenderResult {
            size: genpdf::Size {
                width: area.size().width,
                height: 1.into(),
            },
            has_more: false,
        })
    }
}