use markup5ever::LocalName;
use style::properties::PropertyDeclaration;
use style::values::specified::SVGOpacity;
use style::values::specified::svg::SVGPaint;

/// Convert SVG presentation attribute to a CSS PropertyDeclaration.
/// Returns None if the attribute isn't a presentation attr or value is invalid.
pub fn svg_presentation_hint(local: &LocalName, value: &str) -> Option<PropertyDeclaration> {
    match local.as_ref() {
        "fill" => parse_svg_paint(value).map(|p| PropertyDeclaration::Fill(Box::new(p))),
        "stroke" => parse_svg_paint(value).map(|p| PropertyDeclaration::Stroke(Box::new(p))),
        "fill-opacity" => parse_opacity(value).map(PropertyDeclaration::FillOpacity),
        "stroke-opacity" => parse_opacity(value).map(PropertyDeclaration::StrokeOpacity),
        // TODO: Add more properties as needed
        _ => None,
    }
}

fn parse_svg_paint(_value: &str) -> Option<SVGPaint> {
    // TODO: proper SVG paint parsing
    None
}

fn parse_opacity(_value: &str) -> Option<SVGOpacity> {
    // TODO: proper opacity parsing
    None
}
