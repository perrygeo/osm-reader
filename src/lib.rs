use osmpbfreader;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn count(pbf_filename: String) -> i64 {
    let path = std::path::Path::new(&pbf_filename);
    let fh = std::fs::File::open(&path).unwrap();
    let mut pbf_reader = osmpbfreader::OsmPbfReader::new(fh);
    let mut n_nodes: i64 = 0;
    let mut n_ways: i64 = 0;
    let mut n_way_nodes: i64 = 0;
    let mut n_rels: i64 = 0;
    let mut n_rel_refs: i64 = 0;

    // par_iter give us multiple threads
    for objres in pbf_reader.par_iter() {
        let obj = objres.unwrap();
        match obj {
            osmpbfreader::OsmObj::Node(_node) => {
                n_nodes += 1;
            }
            osmpbfreader::OsmObj::Way(way) => {
                n_ways += 1;
                n_way_nodes += way.nodes.len() as i64;
            }
            osmpbfreader::OsmObj::Relation(rel) => {
                n_rels += 1;
                n_rel_refs += rel.refs.len() as i64;
            }
        }
    }
    n_nodes + n_ways + n_rels // + n_rel_refs + n_way_nodes
}

////////////////////////////////////
// // If your objective is filtering by tag
// let objects = pbf_reader.get_objs_and_deps(|_| {...}).unwrap();
// for (_id, obj) in objects {
////////////////////////////////////

/// A Python module implemented in Rust.
#[pymodule]
fn _pyo3_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(count))?;

    Ok(())
}
