use std::sync::Arc;

pub(crate) fn map_to_arc_vec(list: Vec<&str>) -> Arc<Vec<String>> {
    let list = list
        .iter()
        .map(|entry| entry.to_string())
        .collect::<Vec<String>>();

    return Arc::new(list);
}
