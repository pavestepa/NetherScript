use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, OnceLock, RwLock},
};

static IDENT_POOL: OnceLock<RwLock<Interner>> = OnceLock::new();

struct Interner {
    strs: Vec<Arc<str>>,
    map: HashMap<String, usize>,
}
impl Interner {
    pub fn new() -> Self {
        Self {
            strs: Vec::new(),
            map: HashMap::new(),
        }
    }

    /// add str to pool
    fn intern(&mut self, s: &str) -> usize {
        if let Some(&i) = self.map.get(s) {
            return i;
        }

        let idx = self.strs.len();
        let arc_str: Arc<str> = Arc::from(s);
        self.strs.push(arc_str.clone());
        self.map.insert(s.to_string(), idx);
        idx
    }

    /// get str from pool
    fn get(&self, i: usize) -> Arc<str> {
        self.strs[i].clone()
    }
}
fn ident_pool() -> &'static RwLock<Interner> {
    IDENT_POOL.get_or_init(|| RwLock::new(Interner::new()))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Atom {
    idx: usize,
}
impl Atom {
    /// create
    pub fn from(s: &str) -> Self {
        let mut ident_pool = ident_pool().write().unwrap();
        let idx = ident_pool.intern(s);
        Self { idx }
    }

    // get
    pub fn as_str(&self) -> Arc<str> {
        let ident_pool = ident_pool().read().unwrap();
        ident_pool.get(self.idx)
    }

    /// comparision with str
    pub fn eq_str(&self, s: &str) -> bool {
        self.as_str().as_ref() == s
    }
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ident({:?})", self.as_str())
    }
}

pub fn atom(a: impl Into<String>) -> Atom {
    Atom::from(a.into().as_str())
}
