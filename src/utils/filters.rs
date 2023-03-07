use serde::Deserialize;


#[derive(Debug, Deserialize, Clone)]
pub enum LogicalWrapper {
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Not(Filter),
}

#[derive(Debug, Deserialize, Clone)]
pub enum FieldFilter {
    In(Vec<String>),
    IsExact(String),
    Contains(String),
}

#[derive(Debug, Deserialize, Clone)]
pub enum Filter {
    Operator(Box<LogicalWrapper>),
    Filter(FieldFilter),
}

pub fn execute_filter(value: &String, filter: &Filter) -> bool {
    match filter {
        Filter::Operator(o) => {
            match &**o {
                LogicalWrapper::And(wrapper) => wrapper.iter().fold(
                    true,
                    |state, f| if state == true {execute_filter(value, &f)} else {false}
                ),
                LogicalWrapper::Or(wrapper) => wrapper.iter().fold(
                    false,
                    |state, f| if state == false {execute_filter(value, &f)} else {true}
                ),
                LogicalWrapper::Not(f) => !execute_filter(value, &f),
            }
        },
        Filter::Filter(f) => {
            match f {
                FieldFilter::In(cases) => if cases.contains(value) {true} else {false},
                FieldFilter::IsExact(case) => if case == value {true} else {false},
                FieldFilter::Contains(cases) => if cases.contains(value) {true} else {false},
            }
        },
    }
}
