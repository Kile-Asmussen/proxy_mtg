trait GeneralProxyBuilder {
    fn name(&mut self, name: &str) -> &mut Self;
    fn types(&mut self, type_line: &str) -> &mut Self;

    fn transform<F>(&mut self, notice: Option<&str>, back: F) -> &mut Self
    where
        F: FnOnce(&mut Self);

    fn double_sided<F>(&mut self, back: F) -> &mut Self
    where
        F: FnOnce(&mut Self);

    fn format(&mut self, layout: ArtLayout) -> &mut Self;
}

enum ArtLayout {
    Ordinary,
    VerticalArtLeft,
    VeritcalArtRight,
}
