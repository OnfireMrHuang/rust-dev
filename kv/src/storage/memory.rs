use crate::{KvError, Kvpair, Storage, StorageIter, Value};
use dashmap::{mapref::one::Ref, DashMap};

/// 使用DashMap构建的MemTable，实现了Storage trait
#[derive(Clone, Debug, Default)]
pub struct MemTable {
    tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    // 创建一个缺省的MemTable
    pub fn new() -> Self {
        Self::default()
    }

    /// 如果名为name的hash talbe不存在，则创建，否则返回
    fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    // 实现get trait
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.get(key).map(|v| v.value().clone()))
    }

    // 实现set trait
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.insert(key, value))
    }

    // 实现contains trait
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.contains_key(key))
    }

    // 实现del trait
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.remove(key).map(|(_k, v)| v))
    }

    // 实现get_all trait
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table
            .iter()
            .map(|v| Kvpair::new(v.key(), v.value().clone()))
            .collect())
    }

    // 实现get iter trait
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError> {
        // 首先获取表
        let table = self.get_or_create_table(table).clone();
        let iter = StorageIter::new(table.into_iter());
        Ok(Box::new(iter))
    }
}

impl From<(String, Value)> for Kvpair {
    fn from(data: (String, Value)) -> Self {
        Kvpair::new(data.0, data.1)
    }
}

// 以后想为更多的data store实现Storage trait,都会怎样处理get_iter方法
// 1、拿到一个关于某个table下拥有所有权的Iterator
// 2、对Iterator做map
// 3、将map出来的每个item都转换为KvPair
