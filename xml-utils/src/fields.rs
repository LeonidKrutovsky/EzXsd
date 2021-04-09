use proc_macro2::Ident;

pub struct Fields{
    pub attributes: Vec<Ident>,
    pub elements: Vec<Ident>,
    pub groups: Vec<Ident>,
    pub any_text: Option<Ident>,
    pub any_attributes: Option<Ident>,
    pub any_elements: Option<Ident>,
}