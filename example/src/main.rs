use starlark::environment::{Module, Globals, FrozenModule};
use starlark::{starlark_module, starlark_simple_value};
use starlark::values::{Value, StarlarkValue, Heap};
use starlark::syntax::{AstModule, Dialect, DialectTypes};
use starlark::eval::{Evaluator, ReturnFileLoader};
use starlark::values::none::NoneType;
use std::fmt::{Display, self};
use std::fs;
use std::path::Path;
use starlark::environment::GlobalsBuilder;
use starlark::any::ProvidesStaticType;
use std::cell::RefCell;


//////// Reading the star file and executing in rust /////////////

// fn main() {

// // let script_content = fs::read_to_string("src/add.star").expect("Failed to read script file");

// let ast: AstModule = AstModule::parse_file(Path::new("src/add.star"), &Dialect::Standard).unwrap();

// let globals: Globals = Globals::standard();

// let module: Module = Module::new();

// let mut eval: Evaluator = Evaluator::new(&module);

// let res: Value = eval.eval_module(ast, &globals).unwrap();

// println!("{res}");

// assert_eq!(res.unpack_i32(), Some(4));

// }

//////////////// Call a starlark function frm rust by reading a starlark file ///////////////

// fn main(){

//     let ast: AstModule = AstModule::parse_file(Path::new("src/quad.star"), &Dialect::Standard).unwrap();

//     let globals = Globals::standard();
//     let module = Module::new();
//     let mut eval: Evaluator = Evaluator::new(&module);

//     let sub = eval.eval_module(ast, &globals).unwrap();

//     let heap = module.heap();

//     let res = eval.eval_function(sub, &[heap.alloc(2), heap.alloc(2), heap.alloc(1)], &[("x",heap.alloc(1))],).unwrap();

//     println!("{res}");

//     assert_eq!(res.unpack_i32(), Some(5));
// }

//////////////// call a Rust function from starlark //////////////

// fn main(){

// #[starlark_module]
// fn starlark_helloo(builder: &mut GlobalsBuilder) {
//     fn helloo(name: String) -> anyhow::Result<String> {
//         Ok(name)
//     }
// }

// let globals = GlobalsBuilder::new().with(starlark_helloo).build();
// let module = Module::new();
// let mut eval = Evaluator::new(&module);
// let ast : AstModule = AstModule::parse_file(Path::new("src/hello.star"), &Dialect::Standard).unwrap();
// let res = eval.eval_module(ast, &globals).unwrap();
// println!("{res}");
// assert_eq!(res.unpack_str(), Some("hello world"));
// }

//////////////// Starlark has an enhanced JSON /////////////////

// fn main(){

// #[derive(Debug, ProvidesStaticType, Default)]
// struct Store(RefCell<Vec<String>>);

// impl Store {
//     fn add(&self, x: String) {
//          self.0.borrow_mut().push(x)
//     }
// }

// #[starlark_module]
// fn starlark_emit(builder: &mut GlobalsBuilder) {
//     fn emit(x: Value, eval: &mut Evaluator) -> anyhow::Result<NoneType> {
//         // We modify extra (which we know is a Store) and add the JSON of the
//         // value the user gave.
//         eval.extra
//             .unwrap()
//             .downcast_ref::<Store>()
//             .unwrap()
//             .add(x.to_json()?);
//         Ok(NoneType)
//     }
// }

// let ast : AstModule = AstModule::parse_file(Path::new("src/json.star"), &Dialect::Standard).unwrap();

// let globals = GlobalsBuilder::new().with(starlark_emit).build();

// let module = Module::new();
// let store = Store::default();
// {
//     let mut eval = Evaluator::new(&module);
//     // We add a reference to our store
//     eval.extra = Some(&store);
//     eval.eval_module(ast, &globals).unwrap();
// }
// assert_eq!(&*store.0.borrow(), &["1", "[\"test\"]", "{\"x\":\"y\"}"]);

// }


//////////////// Enabling starlark extensions /////////////////

// fn main(){
// let dialect = Dialect {enable_types: DialectTypes::Enable, ..Dialect::Standard};

// let ast = AstModule::parse_file(Path::new("src/json.star"), &dialect).unwrap();
// let globals = Globals::standard();
// let module = Module::new();
// let mut eval = Evaluator::new(&module);
// let res = eval.eval_module(ast, &globals);

// assert!(res.unwrap_err().to_string().contains("Value `test` of type `string` does not match the type annotation `int`"));
// }

//////////////// Enabling the load statement[Starlark load files imported by the user] /////////////////


fn get_module(file: &str) -> anyhow::Result<FrozenModule> {
    let ast = AstModule::parse_file(Path::new(file), &Dialect::Standard).unwrap();

    let mut loads = Vec::new();
    for load in ast.loads() {
        loads.push((load.module_id.to_owned(), get_module(load.module_id).unwrap()));
    }
    let modules = loads.iter().map(|(a, b)| (a.as_str(), b)).collect();
    let mut loader = ReturnFileLoader { modules: &modules };
    let globals = Globals::standard();
    let module = Module::new();
    {
       let mut eval = Evaluator::new(&module);
       eval.set_loader(&mut loader);
       eval.eval_module(ast, &globals).unwrap();
    }
    Ok(module.freeze().unwrap())
}
fn main(){
let ab = get_module("/Users/prathiksha/Documents/Hugobyte/Practice/practice_starlark-rust/exampls/example-4/loade.star").unwrap();
println!("{:?}", ab);
assert_eq!(ab.get("ab").unwrap().unpack_i32(), Some(14));
}
