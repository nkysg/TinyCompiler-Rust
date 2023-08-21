use std::collections::{HashMap, LinkedList};

#[derive(Debug, Clone)]
pub struct SymInfo {
    name: String,
    mem_loc: i32,
    lines: LinkedList<i32>,
}

impl SymInfo {
    fn new(name: String, mem_loc: i32, lines: LinkedList<i32>) -> Self {
        Self {
            name,
            mem_loc,
            lines,
        }
    }

    fn append_line(&mut self, line: i32) {
        self.lines.push_back(line)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Default)]
pub struct SymTable {
    bucket_list: HashMap<String, SymInfo>,
}

impl SymTable {
    pub fn new() -> Self {
        Self {
            bucket_list: HashMap::new(),
        }
    }

    pub fn st_insert(&mut self, name: &str, line_no: i32, loc: i32) -> Option<SymInfo> {
        if self.bucket_list.contains_key(name) {
            let sym_info = self.bucket_list.get(name);
            let v = sym_info.cloned();
            self.bucket_list.get_mut(name).unwrap().append_line(line_no);
            v
        } else {
            let mut list = LinkedList::new();
            list.push_back(line_no);
            let sym_info = SymInfo::new(name.into(), loc, list);
            self.bucket_list.insert(name.into(), sym_info);
            None
        }
    }

    pub fn st_lookup(&self, name: &str) -> Option<i32> {
        let sym_info = self.bucket_list.get(name);
        sym_info.map(|info| info.mem_loc)
    }
}
