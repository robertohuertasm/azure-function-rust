mod hello;

pub const FUNCTIONS: &[&azure_functions::codegen::Function] = azure_functions::export!{
  hello::hello,
};
