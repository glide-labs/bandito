use petgraph::Graph;
use petgraph::graph::NodeIndex;
// use petgraph::dot::{Dot, Config};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use petgraph::algo::astar;

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

        // println!("{:?}", dg.modules.len());

        Ok(dg)
}


fn create_graph_from_json(dg: DependencyGraph) -> Result<(Graph<String, ()>, HashMap<String, NodeIndex>, HashMap<NodeIndex, String>), Box<dyn Error>> {
    let mut graph = Graph::<String, ()>::new();
    // Use a set to check if node is already visited
    let mut visited = HashMap::new();
    let mut node_idx_to_string = HashMap::new();


     for module in dg.modules {
        let source_node: NodeIndex = if !visited.contains_key(&module.source) { 
            let foo = graph.add_node(module.source.clone());
            // println!("{:?}", foo);
            foo
        } else {
            visited[&module.source]
        }; 
        
        if !visited.contains_key(&module.source) { 
            visited.insert(module.source.clone(), source_node);
            node_idx_to_string.insert(source_node, module.source.clone());
        }

        for dependency in module.dependencies {
            if dependency.dynamic {
                // println!("dynamic");
                continue;
            } 
                
            let dependency_node: NodeIndex = if !visited.contains_key(&dependency.resolved) {
                let bar = graph.add_node(dependency.resolved.clone());
                // println!("{:?}", graph.node_weight(bar));
                bar
            } else {
                visited[&dependency.resolved]
            };
            if !visited.contains_key(&dependency.resolved) {
                visited.insert(dependency.resolved.clone(), dependency_node);
                node_idx_to_string.insert(dependency_node, dependency.resolved.clone());
            }

    // not add edges where dynamic == true;
        graph.add_edge(source_node, dependency_node, ());
        // graph.add_edge(dependency_node, source_node, ());
    }

     }
     Ok((graph, visited, node_idx_to_string))
    
}

fn shortest_path_from_a_to_b(a: String, b: String, visited: HashMap<String, NodeIndex>, graph: Graph<String, ()>) -> Option<(isize, Vec<NodeIndex>)>{
    let dep_a = visited[&a];
    let dep_b = visited[&b];

    let path = astar(
        &graph,
        dep_a,               // start
        |goal| goal == dep_b,      // is_goal
        |_| 1, // edge_cost
        |_| 1,           // estimate_cost
    );
    return path;
}

fn all_paths_from_a_to_b_bfs(a: String, b: String, string_to_nodeidx: HashMap<String, NodeIndex>, nodeidx_to_string: HashMap<NodeIndex, String>,  mut visited: HashSet<NodeIndex>, mut path: Vec<NodeIndex>, graph: Graph<String, ()>) -> () {
    visited.insert(string_to_nodeidx[&a.clone()]);
    path.push(string_to_nodeidx[&a.clone()]);
    // println!("hello");
    // println!("{:?}", a);
    // println!("{:?}", b);


    if a == b {
        let mut path_str: Vec<String> = Vec::new();
        for i in path.clone() {
            path_str.push(nodeidx_to_string[&i].clone()); 
        }
        println!("{:?}", path_str);
    } else {
        // println!("in else");
        for i in graph.neighbors(string_to_nodeidx[&a.clone()]) {
            // println!("{:?}", i);
            if !visited.contains(&i) {
                // println!("recur");
                all_paths_from_a_to_b_bfs(nodeidx_to_string[&i].clone(), b.clone(), string_to_nodeidx.clone(), nodeidx_to_string.clone(), visited.clone(), path.clone(), graph.clone());
            } 
         } 
    }

    path.pop();
    visited.remove(&string_to_nodeidx[&a.clone()]);
}

fn all_paths_from_a_to_b(a: String, b: String, string_to_nodeidx: HashMap<String, NodeIndex>, nodeidx_to_string: HashMap<NodeIndex, String>, graph: Graph<String, ()>) -> () {
    let visited = HashSet::new();
    let paths = Vec::new();
    all_paths_from_a_to_b_bfs(a, b, string_to_nodeidx, nodeidx_to_string, visited, paths, graph);
}

fn main() {
    let dg = create_dg_map("dependencies.json").unwrap();
    let result = create_graph_from_json(dg).unwrap();
    // println!("{:?}", Dot::with_config(&result.0, &[Config::EdgeNoLabel]));
    let path = shortest_path_from_a_to_b("src/common/react-plucked.tsx".to_string(), "src/common/generator/screens.ts".to_string(), result.1, result.0 );
    match path {
        Some((cost, path)) => {
            let mut path_str: Vec<String> = Vec::new();
            for i in path {
                path_str.push(result.2[&i].clone()); 
            }
            println!("Number of edges: {:?}; Path: {:?}", cost, path_str);
        }
        None => println!("No path from a -> b"), 
    }

    // all_paths_from_a_to_b("src/common/react-plucked.tsx".to_string(), "src/common/generator/screens.ts".to_string(), result.1, result.2, result.0);
        
}


//Glide app -> Support -> src/webapp/components/AuthScreen/username-id-screen.tsx -> src/webapp/components/UserCircleImage/index.tsx