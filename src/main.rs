use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::dot::{Dot, Config};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;

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

fn create_dg_map<P: AsRef<Path>>(path: P) -> Result<DependencyGraph, Box<dyn Error>>{
        // Open file in read only mode
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read json and define as instance DependencyGraph
        let dg : DependencyGraph = serde_json::from_reader(reader)?;

        Ok(dg)
}


fn create_graph_from_json(dg: DependencyGraph) -> Result<Graph<String, ()>, Box<dyn Error>> {
    let mut graph = Graph::<String, ()>::new();
    // Use a set to check if node is already visited
    let mut visited = HashMap::new();

     for module in dg.modules {
        let source_node: NodeIndex = if !visited.contains_key(&module.source) { 
            graph.add_node(module.source.clone())
        } else {
            visited[&module.source]
        }; 
        
        if !visited.contains_key(&module.source) { 
            visited.insert(module.source.clone(), source_node);
        }

        for dependency in module.dependencies {
                
            let dependency_node: NodeIndex = if !visited.contains_key(&dependency.resolved) {
                graph.add_node(dependency.resolved.clone())
            } else {
                visited[&dependency.resolved]
            };
            if !visited.contains_key(&dependency.resolved) {
                visited.insert(dependency.resolved.clone(), dependency_node);
            }

        graph.add_edge(source_node, dependency_node, ());
    }

     }
     Ok(graph)
    
}

// fn all_paths_from_a_to_b(a: Dependency, b: Dependency) -> [[Dependency]] {
//     return []
// }

fn main() {
    let dg = create_dg_map("dependencies.json").unwrap();
let result = create_graph_from_json(dg).unwrap();
println!("{:?}", Dot::with_config(&result, &[Config::EdgeNoLabel]));

}


