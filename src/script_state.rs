use std::collections::HashMap;

use variable::Variable;

struct Scope 
{
    variables: HashMap<String, Variable>
}

impl Scope 
{
    pub fn new() -> Scope 
    {
        Scope {
            variables: HashMap::new()
        }
    }

    pub fn add_variable(var: Variable) 
    {
        
    }
}
