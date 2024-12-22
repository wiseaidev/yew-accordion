/// Accordion Size
#[derive(Clone, PartialEq, Default)]
pub enum Size {
    Small,
    Medium,
    Large,
    XSmall,
    XLarge,
    #[default]
    XXLarge,
    Custom(&'static str),
}

impl Size {
    pub fn to_style(&self) -> String {
        match self {
            Size::Small => "width: 7rem;".to_string(),
            Size::Medium => "width: 10rem;".to_string(),
            Size::Large => "width: 20rem;".to_string(),
            Size::XSmall => "width: 5rem;".to_string(),
            Size::XLarge => "width: 25rem;".to_string(),
            Size::XXLarge => "width: 30rem;".to_string(),
            Size::Custom(custom_size) => format!("width: {}; ", custom_size),
        }
    }
}

/// Accordion Item Alignment
#[derive(Clone, PartialEq, Default)]
pub enum Align {
    #[default]
    Left,
    Center,
    Right,
    Justify,
    Start,
    End,
    MatchParent,
    Custom(&'static str),
}

impl Align {
    pub fn to_style(&self) -> String {
        match self {
            Align::Left => "text-align: left;".to_string(),
            Align::Center => "text-align: center;".to_string(),
            Align::Right => "text-align: right;".to_string(),
            Align::Justify => "text-align: justify;".to_string(),
            Align::Start => "text-align: start;".to_string(),
            Align::End => "text-align: end;".to_string(),
            Align::MatchParent => "text-align: match-parent;".to_string(),
            Align::Custom(custom_alignment) => {
                format!("text-align: {}; ", custom_alignment)
            }
        }
    }
}
