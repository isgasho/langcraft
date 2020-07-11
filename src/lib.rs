use cir::Function;
use std::path::Path;
pub use interpreter::Interpreter;

pub mod cir;
pub mod compile_ir;
pub mod interpreter;
mod intrinsics;

pub fn compile_bc(path: &Path) -> Result<Vec<Function>, String> {
    Ok(compile_ir::compile_module(
        &llvm_ir::Module::from_bc_path(path)?,
        &Default::default(),
    ))
}
