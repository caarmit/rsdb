#[derive(PartialEq, Eq, Debug, Clone)]
pub(crate) enum DbErr {
    Generic(String),
    TableAlreadyExists,
    TableNotExists,
}
