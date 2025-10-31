use crate::customer::Customer;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub enum RelationType {
    FRIEND,
    FAMILY,
    BUSINESS,
    OTHER,
    UNDEFINED,
}


#[derive(Clone, Debug)]
pub struct Relation{
    pub from: Customer,
    pub to: Customer,
    pub relation_type: RelationType

}
