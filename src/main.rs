use petgraph::Graph;
// use petgraph::dot::{Dot, Config};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde_json::value::Map;
use serde_json::value::Value;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


#[derive(Serialize, Deserialize, Debug)]
pub struct DependencyGraph {
    modules: Vec<Module>,
    summary: Summary,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    source: String,
    dependencies: Vec<Dependency>,
    valid: bool,
    followable: Option<bool>,
    #[serde(rename = "coreModule")]
    core_module: Option<bool>,
    #[serde(rename = "couldNotResolve")]
    could_not_resolve: Option<bool>,
    #[serde(rename = "matchesDoNotFollow")]
    matches_do_not_follow: Option<bool>,
    #[serde(rename = "dependencyTypes")]
    dependency_types: Option<Vec<DependencyType>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Dependency {
    resolved: String,
    #[serde(rename = "coreModule")]
    core_module: bool,
    followable: bool,
    #[serde(rename = "couldNotResolve")]
    could_not_resolve: bool,
    #[serde(rename = "dependencyTypes")]
    dependency_types: Vec<DependencyType>,
    module: String,
    #[serde(rename = "moduleSystem")]
    module_system: ModuleSystem,
    dynamic: bool,
    #[serde(rename = "matchesDoNotFollow")]
    matches_do_not_follow: bool,
    valid: bool,
    license: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    violations: Vec<Option<serde_json::Value>>,
    error: i64,
    warn: i64,
    info: i64,
    #[serde(rename = "totalCruised")]
    total_cruised: i64,
    #[serde(rename = "totalDependenciesCruised")]
    total_dependencies_cruised: i64,
    #[serde(rename = "optionsUsed")]
    options_used: OptionsUsed,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OptionsUsed {
    #[serde(rename = "combinedDependencies")]
    combined_dependencies: bool,
    exclude: Exclude,
    #[serde(rename = "externalModuleResolutionStrategy")]
    external_module_resolution_strategy: String,
    #[serde(rename = "moduleSystems")]
    module_systems: Vec<String>,
    #[serde(rename = "outputTo")]
    output_to: String,
    #[serde(rename = "outputType")]
    output_type: String,
    #[serde(rename = "preserveSymlinks")]
    preserve_symlinks: bool,
    #[serde(rename = "tsPreCompilationDeps")]
    ts_pre_compilation_deps: bool,
    args: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Exclude {
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DependencyType {
    #[serde(rename = "core")]
    Core,
    #[serde(rename = "local")]
    Local,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ModuleSystem {
    #[serde(rename = "es6")]
    Es6,
}


// fn read_dependencies_from_json<P: AsRef<Path>>(path: P) -> Result<DependencyGraph, Box<dyn Error>> {
//     // Open file in read only mode
//     let file = File::open(path)?;
//     let reader = BufReader::new(file);

//     // Read json and define as instance DependencyGraph
//     let dg = serde_json::from_reader(reader)?;
    

//     Ok(dg)

// }

fn create_graph_from_json<P: AsRef<Path>>(path: P) -> Result<Graph<&'static str, ()>, Box<dyn Error>> {
    let mut graph = Graph::<_, ()>::new();
    graph.add_node("A");
    graph.add_node("B");
    graph.add_node("C");
    graph.add_node("D");
    graph.extend_with_edges(&[ 
        (0, 1), (0, 2), (0, 3),
        (1, 2), (1, 3),
        (2, 3),
    ]);
     // Open file in read only mode
     let file = File::open(path)?;
     let reader = BufReader::new(file);
 
     // Read json and define as instance DependencyGraph
     let dg : DependencyGraph = serde_json::from_reader(reader)?;
    //  let issues = serde_json::from_str::<DependencyGraph>(&dg).unwrap();

     println!("{:#?}", dg.modules);
     
    // for element in &dg[0].iter() {
    //     println!("the value is: {:#?}", element);
    //     for e in element.iter() {
    //         println!("HO HO HO: {:#?}", e);
    //     }
    // }

    Ok(graph)
    
}

// fn all_paths_from_a_to_b(a: Dependency, b: Dependency) -> [[Dependency]] {
//     return []
// }

fn main() {
//     let mut graph = Graph::<_, ()>::new();
//     graph.add_node("A");
//     graph.add_node("B");
//     graph.add_node("C");
//     graph.add_node("D");
//     graph.extend_with_edges(&[
//         (0, 1), (0, 2), (0, 3),
//         (1, 2), (1, 3),
//         (2, 3),
//     ]);

// println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

let result = create_graph_from_json("dependencies.json");

// let dg = read_dependencies_from_json("dependencies.json").unwrap();
println!("{:#?}", result);

}
