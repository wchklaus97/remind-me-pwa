---
alwaysApply: false
description: Storage approach comparison - Flutter IndexedDB vs Dioxus localStorage
---
# Storage Approach Comparison

## 📊 Flutter PWA Storage Pattern (Reference)

### Architecture Overview

The Flutter project uses **IndexedDB** (via `sembast` package) with the following patterns:

1. **Singleton Service Manager**: `IndexedDBSer` manages all database connections
2. **Separated Database Services**: Each data type has its own DBSer class
3. **Reactive Updates**: Uses `ValueNotifier` for UI updates
4. **Initialization Pattern**: All databases initialized on app startup
5. **CRUD Operations**: Each DBSer provides `write()` and `read()` methods

### Flutter Pattern Example

```dart
// Singleton service manager
class IndexedDBSer {
  static final IndexedDBSer _ser = IndexedDBSer._internal();
  factory IndexedDBSer() => _ser;
  
  late ValidatedDBSer _validatedDBSer;
  
  Future<void> init() async {
    await _initDBClient(
      dbSerClient: _validatedDBSer, 
      name: 'ValidatedDB', 
      version: 2
    );
  }
}

// Specialized database service
class ValidatedDBSer extends DBSerClient {
  late ValueNotifier<List<ValidatedRecord?>?> _validatedIDNotifi;
  
  @override
  Future<void> init() async {
    List<ValidatedRecord?>? res = await read();
    if (res != null) {
      _validatedIDNotifi.value = res;
    }
  }
  
  Future<void> write({required String hkid, required bool isValid}) async {
    final record = ValidatedRecord(...);
    await dbSer!.dbstore.add(dbSer!.db, record.toJson());
    _validatedIDNotifi.value!.add(record);
  }
  
  Future<List<ValidatedRecord?>?> read() async {
    // Read from IndexedDB
  }
}
```

### Key Advantages of IndexedDB

- ✅ **Large Storage**: 100MB+ (vs 5-10MB for localStorage)
- ✅ **Indexed Queries**: Fast lookups by key
- ✅ **Transactions**: Atomic operations
- ✅ **Better Performance**: For large datasets
- ✅ **Structured Data**: Object stores vs key-value pairs

---

## 🦀 Dioxus Current Pattern (localStorage)

### Current Implementation

```rust
// Simple localStorage pattern
fn load_reminders() -> Vec<Reminder> {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(Some(data)) = storage.get_item("reminders") {
                if let Ok(reminders) = serde_json::from_str::<Vec<Reminder>>(&data) {
                    return reminders;
                }
            }
        }
    }
    Vec::new()
}

fn save_reminders(reminders: &[Reminder]) {
    if let Some(window) = web_sys::window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(json) = serde_json::to_string(reminders) {
                let _ = storage.set_item("reminders", &json);
            }
        }
    }
}
```

### Advantages of localStorage

- ✅ **Simple API**: Easy to use
- ✅ **Synchronous**: No async complexity
- ✅ **Sufficient for Small Data**: < 5MB works well
- ✅ **No Setup**: Works out of the box

### Limitations

- ❌ **Size Limit**: ~5-10MB per origin
- ❌ **No Indexing**: Full scan for queries
- ❌ **String-Only**: Must serialize everything
- ❌ **No Transactions**: No atomic operations

---

## 🎯 When to Use Each Approach

### Use localStorage (Current) When:
- ✅ Data size < 5MB
- ✅ Simple key-value storage
- ✅ No complex queries needed
- ✅ Quick prototyping
- ✅ Small to medium datasets

### Consider IndexedDB When:
- ✅ Data size > 5MB
- ✅ Need indexed queries
- ✅ Need transactions
- ✅ Large number of records (1000+)
- ✅ Need partial updates
- ✅ Complex data relationships

---

## 🔄 Potential Dioxus IndexedDB Pattern (Future)

If migrating to IndexedDB, consider this pattern:

```rust
// Service manager (similar to Flutter's IndexedDBSer)
pub struct StorageService {
    reminders_db: RemindersDB,
    settings_db: SettingsDB,
}

impl StorageService {
    pub async fn init() -> Result<Self, StorageError> {
        let reminders_db = RemindersDB::new("RemindersDB", 1).await?;
        let settings_db = SettingsDB::new("SettingsDB", 1).await?;
        
        Ok(Self {
            reminders_db,
            settings_db,
        })
    }
    
    pub fn reminders(&self) -> &RemindersDB {
        &self.reminders_db
    }
}

// Specialized database service
pub struct RemindersDB {
    db: IdbDatabase,
    store: ObjectStore,
}

impl RemindersDB {
    pub async fn new(name: &str, version: u32) -> Result<Self, StorageError> {
        // Initialize IndexedDB
    }
    
    pub async fn write(&self, reminder: &Reminder) -> Result<(), StorageError> {
        // Write to IndexedDB
    }
    
    pub async fn read_all(&self) -> Result<Vec<Reminder>, StorageError> {
        // Read all from IndexedDB
    }
    
    pub async fn read_by_id(&self, id: &str) -> Result<Option<Reminder>, StorageError> {
        // Read by ID (indexed query)
    }
}
```

### Rust IndexedDB Crates

- `idb` - IndexedDB bindings for Rust/WASM
- `wasm-bindgen` - Direct web-sys IndexedDB API
- `gloo-storage` - Higher-level storage abstractions

---

## 📋 Migration Checklist (If Needed)

If migrating from localStorage to IndexedDB:

- [ ] Evaluate data size and query needs
- [ ] Choose IndexedDB crate (`idb` or `gloo-storage`)
- [ ] Create storage service manager
- [ ] Create specialized DB services per data type
- [ ] Implement async read/write methods
- [ ] Add error handling
- [ ] Update component state management
- [ ] Test migration path (localStorage → IndexedDB)
- [ ] Update documentation

---

## 🎯 Recommendation for Current Project

**Current Status**: localStorage is **sufficient** for the reminder app

**Reasoning**:
- Reminder data is typically small (< 1MB for 1000+ reminders)
- No complex queries needed (just filter by status)
- Simple CRUD operations
- localStorage is simpler and works well

**Consider IndexedDB if**:
- Adding features that require large data (e.g., file attachments)
- Need to store 10,000+ reminders
- Need complex queries (e.g., search, date ranges)
- Need better performance for large datasets

---

**Reference**: Flutter PWA project uses IndexedDB for better scalability and query capabilities.
