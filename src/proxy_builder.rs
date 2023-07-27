use std::path::Path;

pub trait ProxyBuilder {
    type Output;

    fn build(&mut self) -> Self::Output;
    fn name(&mut self, name: &str) -> &mut Self;
    fn type_line(&mut self, type_line: &str) -> &mut Self;
    fn color_indicator(&mut self, colors: &[String]) -> &mut Self;
    fn color_identity(&mut self, colors: &[String]) -> &mut Self;
    fn mana_cost(&mut self, mana_cost: &str) -> &mut Self;
    fn art_filename(&mut self, art_filename: &Path) -> &mut Self;
    fn art_credits(&mut self, artist: &str) -> &mut Self;
    fn set_legendary(&mut self, is_legendary: bool) -> &mut Self;
}

pub trait ProxyBuilderNormal {
    fn rules_text(&mut self, rules_text: &str) -> &mut Self;
    fn flavor_text(&mut self, flavor_text: &str) -> &mut Self;
    fn corner_bubble(&mut self, corner_bubble: &str) -> &mut Self;
}

pub trait ProxyBuilderReversible: ProxyBuilder {
    type Back: ProxyBuilder<Output = <Self as ProxyBuilder>::Output>;
    fn back(&mut self) -> &mut Self::Back;
}

pub trait ProxyBuilderSaga: ProxyBuilder {
    fn step_text(&mut self, step_text: &str, steps: &[i32]) -> &mut Self;
}

// pub trait ProxyBuilderAdventure: ProxyBuilder {
//     type Adventure: ProxyBuilder<Output = <Self as ProxyBuilder>::Output>;
//     fn adventure(&mut self) -> &mut Self::Adventure;
// }

// pub trait ProxyBuilderSplit: ProxyBuilder {
//     type RightSide: ProxyBuilder<Output = <Self as ProxyBuilder>::Output>;
//     fn right_side(&mut self) -> &mut Self::RightSide;
// }

// pub trait ProxyBuilderFlip: ProxyBuilder {
//     type FlipSide: ProxyBuilder<Output = <Self as ProxyBuilder>::Output>;
//     fn flip_side(&mut self) -> &mut Self::FlipSide;
// }

pub trait BasicLandProxyBuilder {
    type Output;

    fn art_filename(&mut self, art_filename: &str) -> &mut Self;
    fn art_credits(&mut self, artist: &str) -> &mut Self;

    fn build(&self, land: BasicLand) -> Self::Output;
}

pub enum BasicLand {
    Wastes,
    Snow(CoreLand),
    Base(CoreLand),
}

pub enum CoreLand {
    Plains,
    Island,
    Swamp,
    Mountain,
    Forest,
}
