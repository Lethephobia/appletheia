use appletheia::read_model_attribute_key;

#[read_model_attribute_key]
pub enum UserReadModelAttributeKey {
    Username,
    DisplayName,
    Bio,
}
