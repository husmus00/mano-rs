# Changes from C# to Rust Implementation

## Static HashMap Initialization

**Change:** Replaced static readonly Dictionary initialization with functions that create HashMaps.

**C# Version:**
```csharp
protected static readonly Dictionary<string, ushort> MRI = new Dictionary<string, ushort>
{
    { "AND", 0x0000 },
    { "ADD", 0x1000 },
    // ...
};
```

**Rust Version:**
```rust
fn get_mri_table() -> HashMap<&'static str, u16> {
    let mut mri = HashMap::new();
    mri.insert("AND", 0x0000);
    mri.insert("ADD", 0x1000);
    // ...
    mri
}
```

**Why:** Rust doesn't allow complex type initialization in const/static context without external crates. Rust's const evaluation is limited to operations that don't allocate memory, don't call arbitrary functions, and are deterministic. HashMap creation requires heap allocation and dynamic operations that can't happen at compile time.

**Trade-off:** This approach recreates the HashMap on each call, which is less efficient than static initialization. Better alternatives include:
- Using `lazy_static!` or `once_cell` crates for one-time initialization
- Using const arrays with `.collect()` 
- Using `phf` crate for compile-time perfect hash functions

---

## Message System Instead of Direct Console Output

**Change:** Replaced direct console printing with a message collection system.

**C# Version:**
```csharp
Logger.Print("Assembler", "ORG is at address " + origin);
Logger.PrintAndLog("Assembler", "Error: Invalid instruction");
```

**Rust Version:**
```rust
let mut messages = Messages::new();
messages.info("ORG is at address 100");  
messages.error("Invalid instruction");
// Return messages instead of printing
```

**Why:** The original C# code directly printed to console, making it unsuitable for different frontends (TUI, web, etc.). The new system collects messages in a buffer that can be handled differently by each frontend implementation.

**Benefits:**
- **Frontend Agnostic**: Same logic works for CLI, TUI, web interfaces
- **Testable**: Can verify assembler messages without console I/O
- **Library-Friendly**: No unwanted side effects when used as a library  
- **Flexible**: Different frontends can format/display messages as needed

**API Changes:**
- `pass_one()` now returns `Messages` instead of `(bool, Option<HashMap>)`
- `pass_two()` now returns `Messages` instead of `(bool, Option<HashMap>)` 
- All `println!` calls replaced with `messages.info()` or `messages.error()`

---

## Machine Struct Integration

**Change:** Modified Machine struct to store program data and updated assembler functions to work with Machine references.

**C# Version:**
```csharp
// Static fields in Assembler class
protected static List<string> program;
protected static uint origin = 0;

// Separate function calls
var (success, symbolTable) = Assembler.PassOne(program);
var (success2, binaryProgram) = Assembler.PassTwo(program, symbolTable);
```

**Rust Version:**
```rust
pub struct Machine {
    pub source_program: Vec<String>,
    pub assembled_program: Vec<String>,
    pub address_symbol_table: HashMap<String, u32>,
}

// Integrated workflow
machine.load_program(program);
let messages = machine.assemble(); // Does both passes internally
```

**Why:** The original C# used static state and required manual coordination between passes. The new design encapsulates all program state in the Machine struct, making the API cleaner and more stateful.

**Benefits:**
- **State Management**: All program data lives in one place
- **Cleaner API**: Single `assemble()` call handles both passes  
- **Thread Safety**: No static state to cause issues
- **Better Encapsulation**: Machine owns its data

**API Changes:**
- `pass_one()` now takes `&mut Machine` instead of `Vec<String>`
- `pass_two()` now takes `&mut Machine` instead of `Vec<String>` and `HashMap`
- Machine struct stores data that assembler reads from and writes to
- Added `Messages::combine()` method for cleaner message aggregation in multi-step operations

---

## Single-Pass Parsing Optimization

**Change:** Replaced multiple string processing functions with a single zero-allocation parser.

**C# Version:**
```csharp
// Multiple passes with string allocations
string line = program[i];
line = RemoveComment(line);       // New string allocation
line = RemoveLabel(line);         // Another string allocation  
string[] parts = line.Split();    // String array allocation
string instruction = parts[0];    // String indexing
```

**Rust Version:**
```rust
// Single pass with zero allocations
let line = &machine.source_program[i];                              // Just a reference
let (instruction, operand, indirect, has_content) = parse_line_fast(line);  // All &str slices
```

**Why:** The original C# approach processed each assembly line multiple times, creating new string objects at each step. The Rust version uses string slices (`&str`) and parses everything in a single pass, eliminating unnecessary memory allocations.

**Benefits:**
- **Performance**: Zero allocations vs 3-4 allocations per line
- **Memory Efficiency**: No intermediate string objects created
- **CPU Efficiency**: Single parsing pass instead of multiple string operations
- **Cleaner Code**: Direct access to parsed components without indexing

**Performance Impact:** For a 1000-line assembly program: ~4000 allocations → 0 allocations