//! fabric2seer compat helpers. Generated; safe to edit.

/// Detaches a callback argument from `ci`, so the handler can use the object
/// and still call `ci.cancel()` / `ci.set_return_value(..)`.
///
/// SAFETY (the same rule Mixin has for injector arguments): the reference is
/// valid only during the callback that produced it. Never store it.
pub fn detach<'a, T: ?Sized>(r: Option<&mut T>) -> Option<&'a mut T> {
    r.map(|p| unsafe { &mut *(p as *mut T) })
}

/// `detach` for a shared reference.
pub fn detach_ref<'a, T: ?Sized>(r: Option<&T>) -> Option<&'a T> {
    r.map(|p| unsafe { &*(p as *const T) })
}

/// Reads a ported static / @Unique field. The lock is released before this
/// returns, so one statement may read and write the same field.
pub fn get_state<T: Clone>(m: &std::sync::Mutex<T>) -> T {
    m.lock().unwrap_or_else(|p| p.into_inner()).clone()
}

/// Writes a ported static / @Unique field.
pub fn set_state<T>(m: &std::sync::Mutex<T>, value: T) {
    *m.lock().unwrap_or_else(|p| p.into_inner()) = value;
}

/// `System.currentTimeMillis()`.
pub fn millis() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// `System.nanoTime()` (monotonic, arbitrary origin).
pub fn nanos() -> i64 {
    use std::sync::OnceLock;
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    START.get_or_init(std::time::Instant::now).elapsed().as_nanos() as i64
}

/// `Math.random()`.
pub fn random() -> f64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static STATE: AtomicU64 = AtomicU64::new(0x9E37_79B9_7F4A_7C15);
    let mut x = STATE.load(Ordering::Relaxed);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    STATE.store(x, Ordering::Relaxed);
    (x >> 11) as f64 / (1u64 << 53) as f64
}

// ===========================================================================
// Java semantics for translated classes (fabric2seer chunk 368)
// ===========================================================================

/// A Java `static` holding a game object (the port's render / GUI types hold
/// `Rc`, `RefCell` and non-`Send` trait objects, so neither `Mutex` nor
/// `LazyLock` accepts them). Java only touches such statics on the render
/// thread; this cell makes that the rule: it is created lazily on first use,
/// remembers that thread, and PANICS (naming the static's type) if any other
/// thread reaches it. `get` / `get_mut` hand out the value; as with the Java
/// field, a caller must not hold the `&mut` across a call that reads the same
/// static again.
pub struct MainThread<T> {
    init: fn() -> T,
    value: std::sync::OnceLock<SyncCell<T>>,
}

/// The slot inside a `MainThread`: Java statics are shared by every thread, so
/// the cell is reachable from any of them (a Java data race is a data race here
/// too; the generated code never holds the `&mut` across statements).
struct SyncCell<T>(std::cell::UnsafeCell<T>);
// SAFETY: see `MainThread` - a Java static is visible to every thread.
unsafe impl<T> Sync for SyncCell<T> {}
unsafe impl<T> Send for SyncCell<T> {}

// SAFETY: a Java `static` is shared across threads; initialisation is
// synchronised by the `OnceLock` (Java's class-init lock), later accesses are
// exactly as racy as the Java field they port.
unsafe impl<T> Sync for MainThread<T> {}
unsafe impl<T> Send for MainThread<T> {}

impl<T> MainThread<T> {
    pub const fn new(init: fn() -> T) -> MainThread<T> {
        MainThread { init, value: std::sync::OnceLock::new() }
    }

    /// The value (created on first use).
    pub fn get(&self) -> &T {
        self.get_mut()
    }

    /// The value, mutably (a Java field write / mutating call).
    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        let cell = self.value.get_or_init(|| SyncCell(std::cell::UnsafeCell::new((self.init)())));
        // SAFETY: no reference outlives the statement that asked for it in the generated code.
        unsafe { &mut *cell.0.get() }
    }

    /// `with(|v| ..)`: read through a closure.
    pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        f(self.get())
    }

    /// `with_mut(|v| ..)`: change through a closure.
    pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        f(self.get_mut())
    }

    /// A clone of the value.
    pub fn get_clone(&self) -> T
    where
        T: Clone,
    {
        self.get().clone()
    }
}

/// `X.register(..)` on a main-thread static (a mod's own event): the value, on its thread.
impl<T> std::ops::Deref for MainThread<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.get()
    }
}

/// `java.lang.Object` held by value: any shared value.
pub type Object = std::sync::Arc<dyn std::any::Any + Send + Sync>;

/// `new Object()` (a lock or a sentinel).
pub fn new_object() -> Object {
    std::sync::Arc::new(())
}

/// Locks a ported static (`static X: LazyLock<Mutex<T>>`) for one call:
/// `compat::lock(&X).insert(k, v)`. The guard is dropped at the end of the
/// statement - never hold it across a call that reads `X` again.
pub fn lock<T>(m: &std::sync::Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|p| p.into_inner())
}

/// `String.valueOf(obj)` for a value with a `Display`.
pub fn jstring<T: std::fmt::Display + ?Sized>(v: &T) -> String {
    v.to_string()
}

/// `String.valueOf(x)` of a nullable value: "null" when absent.
pub fn opt_str<T: std::fmt::Display>(v: &Option<T>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "null".to_string(),
    }
}

/// `Double.toString(d)` (and `Float`): Java always prints a fraction
/// (`1.0`), uses `E` notation outside 1e-3..1e7, and spells NaN / Infinity.
pub fn fstr(d: f64) -> String {
    if d.is_nan() {
        return "NaN".to_string();
    }
    if d.is_infinite() {
        return if d > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }
    let a = d.abs();
    if a != 0.0 && (a < 1e-3 || a >= 1e7) {
        let s = format!("{:E}", d);
        let (m, e) = s.split_once('E').unwrap_or((&s, "0"));
        let m = if m.contains('.') { m.to_string() } else { format!("{m}.0") };
        return format!("{m}E{e}");
    }
    if d.fract() == 0.0 {
        format!("{:.1}", d)
    } else {
        format!("{}", d)
    }
}

/// `String.length()` (chars; see the module note on UTF-16).
pub fn str_len(s: &str) -> i32 {
    s.chars().count() as i32
}

/// `Util.offsetByCodepoints(s, cursor, n)` over the mod's char index: `cursor` moved by `n` code points, kept within
/// `0..=len`.
pub fn offset_by_codepoints(s: &str, cursor: i32, n: i32) -> i32 {
    (cursor as i64 + n as i64).clamp(0, s.chars().count() as i64) as i32
}

/// `String.isBlank()`: only `Character.isWhitespace` characters.
pub fn is_blank(s: &str) -> bool {
    s.chars().all(java_is_whitespace)
}

/// `Character.isWhitespace(char)`: a Unicode space separator (Zs / Zl / Zp) that is not a non-breaking space
/// (U+00A0, U+2007, U+202F), or one of U+0009-U+000D and U+001C-U+001F. Rust's `char::is_whitespace` (Unicode
/// `White_Space`) differs on both lists, so it is spelled out.
pub fn java_is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{0009}'..='\u{000D}'
            | '\u{001C}'..='\u{001F}'
            | ' '
            | '\u{1680}'
            | '\u{2000}'..='\u{2006}'
            | '\u{2008}'..='\u{200A}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{205F}'
            | '\u{3000}'
    )
}

/// `String.trim()`: strips every char <= ' ' (not Unicode whitespace).
pub fn trim(s: &str) -> String {
    s.trim_matches(|c: char| c <= ' ').to_string()
}

/// `String.equalsIgnoreCase`.
pub fn equals_ignore_case(a: &str, b: &str) -> bool {
    a.chars().count() == b.chars().count()
        && a.chars().zip(b.chars()).all(|(x, y)| {
            x == y || x.to_uppercase().eq(y.to_uppercase()) || x.to_lowercase().eq(y.to_lowercase())
        })
}

fn char_to_byte(s: &str, i: i32) -> usize {
    if i <= 0 {
        return 0;
    }
    s.char_indices().nth(i as usize).map(|(b, _)| b).unwrap_or_else(|| {
        if s.chars().count() == i as usize {
            s.len()
        } else {
            panic!("StringIndexOutOfBoundsException: index {i}, length {}", s.chars().count())
        }
    })
}

fn byte_to_char(s: &str, b: usize) -> i32 {
    s[..b].chars().count() as i32
}

/// `String.substring(begin)`.
pub fn substring(s: &str, begin: i32) -> String {
    s[char_to_byte(s, begin)..].to_string()
}

/// `String.substring(begin, end)`.
pub fn substring2(s: &str, begin: i32, end: i32) -> String {
    if begin > end {
        panic!("StringIndexOutOfBoundsException: begin {begin}, end {end}");
    }
    s[char_to_byte(s, begin)..char_to_byte(s, end)].to_string()
}

/// `String.charAt(i)`.
pub fn char_at(s: &str, i: i32) -> char {
    s.chars().nth(i as usize).unwrap_or_else(|| panic!("StringIndexOutOfBoundsException: index {i}"))
}

/// `String.startsWith(prefix, offset)`.
pub fn starts_with_at(s: &str, prefix: &str, offset: i32) -> bool {
    offset >= 0 && offset as usize <= s.chars().count() && s[char_to_byte(s, offset)..].starts_with(prefix)
}

/// What `indexOf` / `contains` accept: a `&str` / `&String` or a `char`.
pub trait Needle {
    fn find_in(&self, hay: &str) -> Option<usize>;
    fn rfind_in(&self, hay: &str) -> Option<usize>;
}
impl Needle for &str {
    fn find_in(&self, hay: &str) -> Option<usize> {
        hay.find(*self)
    }
    fn rfind_in(&self, hay: &str) -> Option<usize> {
        hay.rfind(*self)
    }
}
impl Needle for &String {
    fn find_in(&self, hay: &str) -> Option<usize> {
        hay.find(self.as_str())
    }
    fn rfind_in(&self, hay: &str) -> Option<usize> {
        hay.rfind(self.as_str())
    }
}
impl Needle for char {
    fn find_in(&self, hay: &str) -> Option<usize> {
        hay.find(*self)
    }
    fn rfind_in(&self, hay: &str) -> Option<usize> {
        hay.rfind(*self)
    }
}

/// `String.indexOf(x)`: -1 when absent.
pub fn index_of(s: &str, n: impl Needle) -> i32 {
    n.find_in(s).map_or(-1, |b| byte_to_char(s, b))
}

/// `String.indexOf(x, from)`.
pub fn index_of_from(s: &str, n: impl Needle, from: i32) -> i32 {
    let len = s.chars().count() as i32;
    if from >= len {
        return -1;
    }
    let start = char_to_byte(s, from.max(0));
    n.find_in(&s[start..]).map_or(-1, |b| byte_to_char(s, start + b))
}

/// `String.lastIndexOf(x)`.
pub fn last_index_of(s: &str, n: impl Needle) -> i32 {
    n.rfind_in(s).map_or(-1, |b| byte_to_char(s, b))
}

/// `String.lastIndexOf(x, from)`.
pub fn last_index_of_from(s: &str, n: impl Needle, from: i32) -> i32 {
    if from < 0 {
        return -1;
    }
    let end = s.char_indices().nth(from as usize + 1).map_or(s.len(), |(b, _)| b);
    n.rfind_in(&s[..end]).map_or(-1, |b| byte_to_char(s, b))
}

/// `String.compareTo`: UTF-16 code unit order (not UTF-8 byte order).
pub fn compare_to(a: &str, b: &str) -> i32 {
    let mut x = a.encode_utf16();
    let mut y = b.encode_utf16();
    loop {
        match (x.next(), y.next()) {
            (Some(p), Some(q)) if p != q => return p as i32 - q as i32,
            (Some(_), Some(_)) => {}
            (None, None) => return 0,
            (None, Some(_)) => return a.encode_utf16().count() as i32 - b.encode_utf16().count() as i32,
            (Some(_), None) => return a.encode_utf16().count() as i32 - b.encode_utf16().count() as i32,
        }
    }
}

/// `String.hashCode()` - Java's `s[0]*31^(n-1) + ...` over UTF-16 units.
pub fn string_hash_code(s: &str) -> i32 {
    s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
}

/// `Object.hashCode()` of a value with `Hash` (NOT Java's numbers: only equal-for-equal holds).
pub fn hash_code<T: std::hash::Hash + ?Sized>(v: &T) -> i32 {
    use std::hash::Hasher;
    let mut h = std::collections::hash_map::DefaultHasher::new();
    v.hash(&mut h);
    h.finish() as i32
}

/// A comparison as Java's int (`compareTo` / `Comparator.compare`).
pub fn ord_int(o: std::cmp::Ordering) -> i32 {
    o as i32
}

/// `Math.round(float)`: floor(x + 0.5).
pub fn java_round_f32(x: f32) -> i32 {
    (x + 0.5).floor() as i32
}

/// `Math.round(double)`: floor(x + 0.5).
pub fn java_round_f64(x: f64) -> i64 {
    (x + 0.5).floor() as i64
}

/// `Math.floorDiv` / `floorMod` (rounding toward negative infinity).
pub fn floor_div_i32(a: i32, b: i32) -> i32 {
    let q = a.wrapping_div(b);
    if (a % b != 0) && ((a < 0) != (b < 0)) { q - 1 } else { q }
}
pub fn floor_mod_i32(a: i32, b: i32) -> i32 {
    a.wrapping_sub(floor_div_i32(a, b).wrapping_mul(b))
}
pub fn floor_div_i64(a: i64, b: i64) -> i64 {
    let q = a.wrapping_div(b);
    if (a % b != 0) && ((a < 0) != (b < 0)) { q - 1 } else { q }
}
pub fn floor_mod_i64(a: i64, b: i64) -> i64 {
    a.wrapping_sub(floor_div_i64(a, b).wrapping_mul(b))
}

/// `Mth.lerp` / `Math` helpers.
pub fn lerp(delta: f64, start: f64, end: f64) -> f64 {
    start + delta * (end - start)
}

/// `Character.toUpperCase(c)` / `toLowerCase(c)` (one char in, one out).
pub fn char_upper(c: char) -> char {
    c.to_uppercase().next().unwrap_or(c)
}
pub fn char_lower(c: char) -> char {
    c.to_lowercase().next().unwrap_or(c)
}

/// `List.remove(Object)`: removes the first equal element.
pub fn remove_item<T: PartialEq>(v: &mut Vec<T>, x: &T) -> bool {
    match v.iter().position(|e| e == x) {
        Some(i) => {
            v.remove(i);
            true
        }
        None => false,
    }
}

/// `StringBuilder.setLength(n)`.
pub fn set_length(s: &mut String, n: i32) {
    let b = char_to_byte(s, n);
    if b <= s.len() {
        s.truncate(b);
    }
    while (s.chars().count() as i32) < n {
        s.push('\0');
    }
}

/// `StringBuilder.insert(i, str)`.
pub fn insert_str(s: &mut String, i: i32, x: &str) {
    let b = char_to_byte(s, i);
    s.insert_str(b, x);
}

/// `StringBuilder.deleteCharAt(i)`.
pub fn delete_char_at(s: &mut String, i: i32) {
    let b = char_to_byte(s, i);
    s.remove(b);
}

/// `Arrays.copyOf(a, n)`.
pub fn copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
    let mut v: Vec<T> = a.iter().take(n).cloned().collect();
    v.resize(n, T::default());
    v
}

/// `Collections.shuffle(list)`.
pub fn shuffle<T>(v: &mut Vec<T>) {
    for i in (1..v.len()).rev() {
        let j = (random() * (i + 1) as f64) as usize;
        v.swap(i, j.min(i));
    }
}

/// `Stream.sorted()`.
pub fn sorted<T: Ord>(it: impl Iterator<Item = T>) -> std::vec::IntoIter<T> {
    let mut v: Vec<T> = it.collect();
    v.sort();
    v.into_iter()
}

/// `Stream.sorted(comparator)` (stable, as Java's).
pub fn sorted_by<T>(it: impl Iterator<Item = T>, cmp: impl FnMut(&T, &T) -> std::cmp::Ordering) -> std::vec::IntoIter<T> {
    let mut v: Vec<T> = it.collect();
    v.sort_by(cmp);
    v.into_iter()
}

/// `Stream.distinct()` (first occurrence kept, order kept).
pub fn distinct<T: PartialEq + Clone>(it: impl Iterator<Item = T>) -> std::vec::IntoIter<T> {
    let mut v: Vec<T> = Vec::new();
    for x in it {
        if !v.contains(&x) {
            v.push(x);
        }
    }
    v.into_iter()
}

/// `IntStream.average()`.
pub fn average(it: impl Iterator<Item = f64>) -> Option<f64> {
    let (mut n, mut s) = (0usize, 0.0);
    for x in it {
        n += 1;
        s += x;
    }
    if n == 0 { None } else { Some(s / n as f64) }
}

/// `Collectors.groupingBy(key)`.
pub fn grouping_by<T, K: std::hash::Hash + Eq>(
    it: impl Iterator<Item = T>,
    key: impl Fn(&T) -> K,
) -> std::collections::HashMap<K, Vec<T>> {
    let mut m: std::collections::HashMap<K, Vec<T>> = std::collections::HashMap::new();
    for x in it {
        m.entry(key(&x)).or_default().push(x);
    }
    m
}

/// `Collectors.toMap(k, v, merge)`.
pub fn to_map_merge<K: std::hash::Hash + Eq, V>(
    it: impl Iterator<Item = (K, V)>,
    merge: impl Fn(V, V) -> V,
) -> std::collections::HashMap<K, V> {
    let mut m = std::collections::HashMap::new();
    for (k, v) in it {
        let nv = match m.remove(&k) {
            Some(old) => merge(old, v),
            None => v,
        };
        m.insert(k, nv);
    }
    m
}

/// `Map.merge(k, v, f)`.
pub fn map_merge<K: std::hash::Hash + Eq, V: Clone>(
    m: &mut std::collections::HashMap<K, V>,
    k: K,
    v: V,
    f: impl Fn(V, V) -> V,
) -> V {
    let nv = match m.remove(&k) {
        Some(old) => f(old, v),
        None => v,
    };
    m.insert(k, nv.clone());
    nv
}

/// One argument of `String.format` (`java.util.Formatter`): an integral, a floating-point or any other value (its
/// `toString`). A `float` widens to `double`, as `Formatter.print(float, Locale)` does.
#[derive(Clone, Debug)]
pub enum FormatArg {
    Int(i64),
    Float(f64),
    Str(String),
}

impl From<i32> for FormatArg {
    fn from(v: i32) -> Self {
        FormatArg::Int(v as i64)
    }
}
impl From<i64> for FormatArg {
    fn from(v: i64) -> Self {
        FormatArg::Int(v)
    }
}
impl From<f32> for FormatArg {
    fn from(v: f32) -> Self {
        FormatArg::Float(v as f64)
    }
}
impl From<f64> for FormatArg {
    fn from(v: f64) -> Self {
        FormatArg::Float(v)
    }
}
impl From<&str> for FormatArg {
    fn from(v: &str) -> Self {
        FormatArg::Str(v.to_string())
    }
}
impl From<String> for FormatArg {
    fn from(v: String) -> Self {
        FormatArg::Str(v)
    }
}
impl From<bool> for FormatArg {
    fn from(v: bool) -> Self {
        FormatArg::Str(v.to_string())
    }
}
impl From<char> for FormatArg {
    fn from(v: char) -> Self {
        FormatArg::Str(v.to_string())
    }
}

/// `String.format(Locale.ENGLISH / Locale.US / Locale.ROOT, format, args)` - `java.util.Formatter`'s
/// `%[argument_index$][flags][width][.precision]conversion` for the conversions `d s f e g % n` and the flags
/// `- + space 0 ,`. A floating-point conversion rounds the value's shortest decimal digits (`Double.toString`'s)
/// HALF_UP, as `sun.misc.FormattedFloatingDecimal` does - not the binary value, as C's `printf` would.
pub fn string_format(format: &str, args: &[FormatArg]) -> String {
    let chars: Vec<char> = format.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut ordinary = 0usize;
    while i < chars.len() {
        if chars[i] != '%' {
            out.push(chars[i]);
            i += 1;
            continue;
        }
        i += 1;
        // argument_index$
        let mut j = i;
        while j < chars.len() && chars[j].is_ascii_digit() {
            j += 1;
        }
        let mut index: Option<usize> = None;
        if j > i && j < chars.len() && chars[j] == '$' {
            index = chars[i..j].iter().collect::<String>().parse::<usize>().ok().map(|n| n - 1);
            i = j + 1;
        }
        let mut flags = String::new();
        while i < chars.len() && "-+ 0,#(".contains(chars[i]) {
            flags.push(chars[i]);
            i += 1;
        }
        let mut width: Option<usize> = None;
        let start = i;
        while i < chars.len() && chars[i].is_ascii_digit() {
            i += 1;
        }
        if i > start {
            width = chars[start..i].iter().collect::<String>().parse().ok();
        }
        let mut precision: Option<usize> = None;
        if i < chars.len() && chars[i] == '.' {
            i += 1;
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            precision = chars[start..i].iter().collect::<String>().parse().ok();
        }
        let conversion = chars.get(i).copied().expect("UnknownFormatConversionException");
        i += 1;
        if conversion == '%' {
            out.push('%');
            continue;
        }
        if conversion == 'n' {
            out.push('\n');
            continue;
        }
        let arg = match index {
            Some(n) => args.get(n),
            None => {
                ordinary += 1;
                args.get(ordinary - 1)
            }
        }
        .expect("MissingFormatArgumentException");
        let grouping = flags.contains(',');
        let body = match (conversion.to_ascii_lowercase(), arg) {
            ('d', FormatArg::Int(v)) => format_sign(&group_digits(&v.unsigned_abs().to_string(), grouping), *v < 0, &flags),
            ('f', FormatArg::Float(v)) => format_float(*v, &flags, |d, neg| format_sign(&group_digits_fixed(&fixed_digits(d, precision.unwrap_or(6)), grouping), neg, &flags)),
            ('e', FormatArg::Float(v)) => format_float(*v, &flags, |d, neg| format_sign(&scientific_digits(d, precision.unwrap_or(6)), neg, &flags)),
            ('g', FormatArg::Float(v)) => format_float(*v, &flags, |d, neg| {
                let p = match precision {
                    None => 6,
                    Some(0) => 1,
                    Some(p) => p,
                };
                format_sign(&general_digits(d, p, grouping), neg, &flags)
            }),
            ('s', FormatArg::Str(s)) | ('b', FormatArg::Str(s)) | ('c', FormatArg::Str(s)) => s.clone(),
            ('x', FormatArg::Int(v)) => format!("{:x}", v),
            ('o', FormatArg::Int(v)) => format!("{:o}", v),
            ('s', FormatArg::Int(v)) => v.to_string(),
            ('s', FormatArg::Float(v)) => mcrust::javacompat::java_double_to_string(*v),
            _ => panic!("IllegalFormatConversionException: {} != {:?}", conversion, arg),
        };
        let body = if conversion.is_ascii_uppercase() { body.to_uppercase() } else { body };
        let body: String = match precision {
            Some(p) if conversion.to_ascii_lowercase() == 's' => body.chars().take(p).collect(),
            _ => body,
        };
        match width {
            Some(w) if body.chars().count() < w => {
                let pad = w - body.chars().count();
                if flags.contains('-') {
                    out.push_str(&body);
                    out.push_str(&" ".repeat(pad));
                } else if flags.contains('0') {
                    let sign_len = if body.starts_with(['+', '-', ' ']) { 1 } else { 0 };
                    out.push_str(&body[..sign_len]);
                    out.push_str(&"0".repeat(pad));
                    out.push_str(&body[sign_len..]);
                } else {
                    out.push_str(&" ".repeat(pad));
                    out.push_str(&body);
                }
            }
            _ => out.push_str(&body),
        }
    }
    out
}

/// The shortest decimal digits of `|v|` (`Double.toString`'s) and the decimal exponent of the first:
/// `|v| = d1.d2d3.. * 10^exp`.
fn shortest_digits(v: f64) -> (Vec<u8>, i32) {
    let s = format!("{:e}", v.abs());
    let (mantissa, exp) = s.split_once('e').unwrap();
    let digits: Vec<u8> = mantissa.bytes().filter(|b| b.is_ascii_digit()).map(|b| b - b'0').collect();
    (digits, exp.parse::<i32>().unwrap())
}

/// Rounds `digits` HALF_UP to `keep` digits; a carry out of the first digit raises `exp`. `keep < 0` is zero.
fn round_half_up(mut digits: Vec<u8>, mut exp: i32, keep: i32) -> (Vec<u8>, i32) {
    if keep < 0 {
        return (vec![0], exp);
    }
    let keep = keep as usize;
    if digits.len() > keep {
        let round_up = digits[keep] >= 5;
        digits.truncate(keep);
        if round_up {
            let mut k = keep;
            loop {
                if k == 0 {
                    digits.insert(0, 1);
                    exp += 1;
                    break;
                }
                k -= 1;
                if digits[k] == 9 {
                    digits[k] = 0;
                } else {
                    digits[k] += 1;
                    break;
                }
            }
        }
    }
    (digits, exp)
}

fn format_float(v: f64, flags: &str, finite: impl Fn(f64, bool) -> String) -> String {
    if v.is_nan() {
        return "NaN".to_string();
    }
    if v.is_infinite() {
        return format_sign("Infinity", v < 0.0, flags);
    }
    finite(v, v.is_sign_negative() && v != 0.0)
}

fn format_sign(body: &str, negative: bool, flags: &str) -> String {
    if negative {
        format!("-{}", body)
    } else if flags.contains('+') {
        format!("+{}", body)
    } else if flags.contains(' ') {
        format!(" {}", body)
    } else {
        body.to_string()
    }
}

/// `%.{precision}f` of `|v|`.
fn fixed_digits(v: f64, precision: usize) -> String {
    if v == 0.0 {
        return if precision == 0 { "0".to_string() } else { format!("0.{}", "0".repeat(precision)) };
    }
    let (digits, exp) = shortest_digits(v);
    // exp + 1 digits sit before the point; keep those and `precision` more
    let (digits, exp) = round_half_up(digits, exp, exp + 1 + precision as i32);
    let mut int_part = String::new();
    for k in 0..(exp + 1).max(0) {
        int_part.push((b'0' + digits.get(k as usize).copied().unwrap_or(0)) as char);
    }
    if int_part.is_empty() {
        int_part.push('0');
    }
    let mut frac_part = String::new();
    for k in 0..precision as i32 {
        let pos = exp + 1 + k;
        let d = if pos >= 0 { digits.get(pos as usize).copied().unwrap_or(0) } else { 0 };
        frac_part.push((b'0' + d) as char);
    }
    if precision == 0 { int_part } else { format!("{}.{}", int_part, frac_part) }
}

/// `%.{precision}e` of `|v|`: `d.ddde+xx`.
fn scientific_digits(v: f64, precision: usize) -> String {
    let (digits, exp) = if v == 0.0 {
        (vec![0], 0)
    } else {
        let (d, e) = shortest_digits(v);
        round_half_up(d, e, precision as i32 + 1)
    };
    let mut s = String::new();
    s.push((b'0' + digits[0]) as char);
    if precision > 0 {
        s.push('.');
        for k in 1..=precision {
            s.push((b'0' + digits.get(k).copied().unwrap_or(0)) as char);
        }
    }
    s.push('e');
    s.push(if exp < 0 { '-' } else { '+' });
    s.push_str(&format!("{:02}", exp.abs()));
    s
}

/// `%.{precision}g` of `|v|`: rounded to `precision` significant digits, then fixed when
/// `10^-4 <= x < 10^precision`, scientific otherwise.
fn general_digits(v: f64, precision: usize, grouping: bool) -> String {
    if v == 0.0 {
        return fixed_digits(0.0, precision - 1);
    }
    let (digits, exp) = shortest_digits(v);
    let (_, rounded_exp) = round_half_up(digits, exp, precision as i32);
    if rounded_exp >= -4 && rounded_exp < precision as i32 {
        group_digits_fixed(&fixed_digits(v, (precision as i32 - 1 - rounded_exp).max(0) as usize), grouping)
    } else {
        scientific_digits(v, precision - 1)
    }
}

/// The `,` flag: the integer digits in groups of three (`Locale.ENGLISH`'s separator).
fn group_digits(int_part: &str, grouping: bool) -> String {
    if !grouping {
        return int_part.to_string();
    }
    let chars: Vec<char> = int_part.chars().collect();
    let mut out = String::new();
    for (k, c) in chars.iter().enumerate() {
        if k > 0 && (chars.len() - k) % 3 == 0 {
            out.push(',');
        }
        out.push(*c);
    }
    out
}

fn group_digits_fixed(fixed: &str, grouping: bool) -> String {
    match fixed.split_once('.') {
        Some((int_part, frac)) => format!("{}.{}", group_digits(int_part, grouping), frac),
        None => group_digits(fixed, grouping),
    }
}

/// `Integer.decode(nm)`: an optional sign, then `0x` / `0X` / `#` (hexadecimal), a leading `0` (octal) or decimal
/// digits. A malformed string is Java's `NumberFormatException`.
pub fn integer_decode(nm: &str) -> i32 {
    let (negative, rest) = match nm.as_bytes().first() {
        Some(b'-') => (true, &nm[1..]),
        Some(b'+') => (false, &nm[1..]),
        _ => (false, nm),
    };
    let (radix, digits) = if let Some(d) = rest.strip_prefix("0x").or_else(|| rest.strip_prefix("0X")).or_else(|| rest.strip_prefix('#')) {
        (16, d)
    } else if rest.len() > 1 && rest.starts_with('0') {
        (8, &rest[1..])
    } else {
        (10, rest)
    };
    if digits.starts_with(['-', '+']) {
        panic!("NumberFormatException: Sign character in wrong position");
    }
    let signed = if negative { format!("-{}", digits) } else { digits.to_string() };
    i32::from_str_radix(&signed, radix).unwrap_or_else(|_| panic!("NumberFormatException: For input string: \"{}\"", nm))
}

/// `Files.createTempFile(prefix, suffix)`: a new empty file in the temp directory (`suffix` null is ".tmp").
pub fn create_temp_file(prefix: &str, suffix: Option<&str>) -> std::io::Result<std::path::PathBuf> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir();
    loop {
        let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
        let name = format!("{}{}{}{}", prefix, nanos, N.fetch_add(1, Ordering::Relaxed), suffix.unwrap_or(".tmp"));
        let path = dir.join(name);
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(_) => return Ok(path),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
}

/// `Files.move(a, b, REPLACE_EXISTING)` without ATOMIC_MOVE: a rename, else (another volume) a copy and a delete.
pub fn files_move(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    match std::fs::rename(from, to) {
        Ok(()) => Ok(()),
        Err(_) => {
            std::fs::copy(from, to)?;
            std::fs::remove_file(from)
        }
    }
}

/// A caught panic's message (`e.getMessage()` in a translated `catch`).
pub fn panic_message(e: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = e.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = e.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "panic".to_string()
    }
}

/// `obj.getClass()` as a name.
pub fn type_name_of<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

/// `UUID.fromString` (the 128-bit value; the port's UUIDs are `u128` where it has them).
pub fn uuid_from_string(s: &str) -> u128 {
    u128::from_str_radix(&s.replace('-', ""), 16).expect("IllegalArgumentException: Invalid UUID string")
}

/// `UUID.randomUUID()` (version 4).
pub fn random_uuid() -> u128 {
    let a = (random() * u64::MAX as f64) as u128;
    let b = (random() * u64::MAX as f64) as u128;
    ((a << 64 | b) & !(0xF000u128 << 64) | (0x4000u128 << 64)) & !(0xC000u128 << 48) | (0x8000u128 << 48)
}

/// `TimeUnit`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
}

/// `java.util.Random` (an xorshift generator; not Java's LCG sequence).
pub struct Random(u64);
impl Random {
    pub fn new() -> Random {
        Random((random() * u64::MAX as f64) as u64 | 1)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    pub fn next_int(&mut self, bound: i32) -> i32 {
        (self.next() % bound.max(1) as u64) as i32
    }
    pub fn next_double(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn next_float(&mut self) -> f32 {
        self.next_double() as f32
    }
    pub fn next_boolean(&mut self) -> bool {
        self.next() & 1 == 1
    }
}

/// A Java `UUID` the port's entity code keeps as a `u128` (`Entity.get_uuid`), as the port's
/// `java::util::uuid::Uuid` (two signed halves, as Java's).
pub fn uuid_of(bits: u128) -> mcrust::java::util::uuid::Uuid {
    mcrust::java::util::uuid::Uuid::new((bits >> 64) as i64, bits as u64 as i64)
}

/// The other way: a `Uuid` for a port parameter typed `u128`.
pub fn uuid_bits(uuid: &mcrust::java::util::uuid::Uuid) -> u128 {
    ((uuid.get_most_significant_bits() as u64 as u128) << 64) | (uuid.get_least_significant_bits() as u64 as u128)
}

/// The executor `CompletableFuture.supplyAsync(.., executor)` / `runAsync` hand
/// their task to: a new thread per task (Java's virtual-thread executors).
pub fn executor(task: Box<dyn FnOnce() + Send>) {
    std::thread::spawn(task);
}

/// `Screen.font` / `Minecraft.font` as Java has it: a `Font` over the game's font set.
pub fn font() -> mcrust::net::minecraft::client::gui::font::Font<'static> {
    // The game REPLACES its `Arc<FontSet>` on a resource reload, so a `Font`
    // borrowing the field dangles once the old set is dropped (a static
    // `Font` taken before the boot reload read freed memory). Keep one leaked
    // clone per font set, so every `Font` handed out stays valid for ever.
    use mcrust::net::minecraft::client::gui::font::font_set::FontSet;
    static KEPT: std::sync::Mutex<Vec<&'static std::sync::Arc<FontSet>>> = std::sync::Mutex::new(Vec::new());
    let current = &minecraft().font;
    let mut kept = KEPT.lock().unwrap_or_else(|p| p.into_inner());
    let set: &'static std::sync::Arc<FontSet> = match kept.iter().find(|k| std::sync::Arc::ptr_eq(k, current)) {
        Some(k) => k,
        None => {
            let k: &'static std::sync::Arc<FontSet> = Box::leak(Box::new(std::sync::Arc::clone(current)));
            kept.push(k);
            k
        }
    };
    mcrust::net::minecraft::client::gui::font::Font { set }
}

/// A Component the port hands back as the `FormattedText` it is (`Component extends FormattedText`):
/// the component itself when it is one, else the same text as a literal component with its style.
pub fn component_of(
    text: mcrust::net::minecraft::network::chat::formatted_text::FormattedText,
) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
    use mcrust::net::minecraft::network::chat::component;
    use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
    match text {
        FormattedText::Component(c) => *c,
        FormattedText::Empty => component::empty(),
        FormattedText::Plain(s) => component::literal(&s),
        FormattedText::Styled(s, style) => component::literal(&s).set_style(style),
        FormattedText::Composite(parts) => parts
            .into_iter()
            .fold(component::empty(), |acc, part| acc.append(component_of(part))),
    }
}

/// A Java `Component` the port holds as text in places (`Screen.title` is a `String`): either one, as a
/// component.
pub trait IntoComponent {
    fn into_component(self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
}

impl IntoComponent for mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
    fn into_component(self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        self
    }
}

impl IntoComponent for String {
    fn into_component(self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        mcrust::net::minecraft::network::chat::component::literal(&self)
    }
}

impl IntoComponent for &str {
    fn into_component(self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        mcrust::net::minecraft::network::chat::component::literal(self)
    }
}

impl IntoComponent for mcrust::net::minecraft::network::chat::formatted_text::FormattedText {
    fn into_component(self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        component_of(self)
    }
}

/// `new StringWidget(..)` as the port's own screens build one (confirm_link_screen.rs): an inactive
/// `AbstractWidget` of kind `StringWidget` whose message text is the component. A width of -1 is
/// Java's `font.width(message)`.
pub fn string_widget(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    message: impl IntoComponent,
) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget {
    use mcrust::net::minecraft::client::gui::components::abstract_widget::{AbstractWidget, OnPress, WidgetKind};
    let message = message.into_component();
    use mcrust::net::minecraft::network::chat::component::Component as _;
    use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
    let text = FormattedText::from(message.clone());
    let w = if width < 0 { font().width_of(&text) } else { width };
    let mut widget = AbstractWidget::new(x, y, w, height, message.get_string(), WidgetKind::StringWidget, OnPress::Close);
    widget.set_message_text(text);
    widget.active = false;
    widget
}

static MINECRAFT: std::sync::atomic::AtomicPtr<mcrust::net::minecraft::client::minecraft::Minecraft> =
    std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

/// Records the game's `Minecraft` on every client tick, for `minecraft()`.
/// The generated entrypoints call it first; it registers once.
pub fn install() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        mcrust::mcrust::modding::events::client_tick_events::START_CLIENT_TICK.register(std::sync::Arc::new(
            |mc: &mut mcrust::net::minecraft::client::minecraft::Minecraft| {
                MINECRAFT.store(mc as *mut _, std::sync::atomic::Ordering::Release);
            },
        ));
    });
}

/// `Minecraft.getInstance()`: the game's one Minecraft, as the tick events
/// hand it over (see `install`).
///
/// SAFETY (the rule Java's global has): the Minecraft lives as long as the
/// game; this is the same object the game is using, so do not hold the
/// reference across a call that hands you the Minecraft again.
pub fn minecraft() -> &'static mut mcrust::net::minecraft::client::minecraft::Minecraft {
    let p = MINECRAFT.load(std::sync::atomic::Ordering::Acquire);
    if p.is_null() {
        panic!("Minecraft.getInstance() before the first client tick (fabric2seer compat::minecraft)");
    }
    unsafe { &mut *p }
}


/// `java.util.regex` on `fancy-regex` (which has Java's look-around and
/// back-references). Indices are chars, as `compat::substring` expects.
pub mod regex {
    fn translate(java: &str) -> String {
        // Java's possessive quantifiers and atomic groups are fancy-regex syntax too; `\Q..\E` is not.
        if let (Some(a), Some(b)) = (java.find("\\Q"), java.find("\\E")) {
            if a < b {
                let lit = fancy_regex::escape(&java[a + 2..b]).into_owned();
                return format!("{}{}{}", &java[..a], lit, translate(&java[b + 2..]));
            }
        }
        java.to_string()
    }

    fn java_replacement(rep: &str) -> String {
        // Java: `$1` / `${name}` refer to groups, `\$` and `\\` are literal; fancy-regex: `$$` is a literal `$`
        let mut out = String::new();
        let mut it = rep.chars().peekable();
        while let Some(c) = it.next() {
            if c == '\\' {
                if let Some(n) = it.next() {
                    if n == '$' {
                        out.push_str("$$");
                    } else {
                        out.push(n);
                    }
                }
            } else if c == '$' && it.peek().is_some_and(|d| d.is_ascii_digit()) {
                out.push_str("${");
                while it.peek().is_some_and(|d| d.is_ascii_digit()) {
                    out.push(it.next().unwrap());
                }
                out.push('}');
            } else {
                out.push(c);
            }
        }
        out
    }

    /// `java.util.regex.Pattern`.
    #[derive(Clone, Debug)]
    pub struct Pattern {
        source: String,
        re: fancy_regex::Regex,
        full: fancy_regex::Regex,
        flags: i32,
    }

    pub const CASE_INSENSITIVE: i32 = 2;
    pub const MULTILINE: i32 = 8;
    pub const DOTALL: i32 = 32;

    impl Pattern {
        /// `Pattern.compile(regex)`; a bad pattern panics (Java throws PatternSyntaxException).
        pub fn compile(regex: &str) -> Pattern {
            Pattern::compile_with_flags(regex, 0)
        }

        pub fn compile_with_flags(regex: &str, flags: i32) -> Pattern {
            let mut pre = String::new();
            if flags & CASE_INSENSITIVE != 0 {
                pre.push('i');
            }
            if flags & MULTILINE != 0 {
                pre.push('m');
            }
            if flags & DOTALL != 0 {
                pre.push('s');
            }
            let body = translate(regex);
            let body = if pre.is_empty() { body } else { format!("(?{pre}){body}") };
            let re = fancy_regex::Regex::new(&body)
                .unwrap_or_else(|e| panic!("PatternSyntaxException: {e} in {regex:?}"));
            let full = fancy_regex::Regex::new(&format!("^(?:{body})$"))
                .unwrap_or_else(|e| panic!("PatternSyntaxException: {e} in {regex:?}"));
            Pattern { source: regex.to_string(), re, full, flags }
        }

        /// `Pattern.matches(regex, input)`.
        pub fn matches(regex: &str, input: &str) -> bool {
            Pattern::compile(regex).full.is_match(input).unwrap_or(false)
        }

        /// `Pattern.quote(s)`.
        pub fn quote(s: &str) -> String {
            fancy_regex::escape(s).into_owned()
        }

        pub fn pattern(&self) -> &str {
            &self.source
        }

        pub fn flags(&self) -> i32 {
            self.flags
        }

        /// `pattern.matcher(input)`.
        pub fn matcher(&self, input: &str) -> Matcher {
            Matcher { pattern: self.clone(), input: input.to_string(), caps: None, pos: 0 }
        }

        /// `asPredicate()`'s test: a match anywhere.
        pub fn find(&self, s: &str) -> bool {
            self.re.is_match(s).unwrap_or(false)
        }

        /// `asMatchPredicate()`'s test: the whole input.
        pub fn matches_all(&self, s: &str) -> bool {
            self.full.is_match(s).unwrap_or(false)
        }

        /// `pattern.split(input)` (trailing empty strings removed, as Java).
        pub fn split(&self, input: &str) -> Vec<String> {
            split_with(&self.re, input, 0)
        }
    }

    fn split_with(re: &fancy_regex::Regex, input: &str, limit: i32) -> Vec<String> {
        let mut out = Vec::new();
        let mut last = 0;
        for m in re.find_iter(input).flatten() {
            if limit > 0 && out.len() as i32 == limit - 1 {
                break;
            }
            if m.end() == 0 {
                continue; // a zero-width match at the start yields no leading empty string (Java 8+)
            }
            out.push(input[last..m.start()].to_string());
            last = m.end();
        }
        out.push(input[last..].to_string());
        if limit == 0 {
            while out.len() > 1 && out.last().is_some_and(|s| s.is_empty()) {
                out.pop();
            }
            if out.len() == 1 && out[0].is_empty() && !input.is_empty() {
                out.pop();
            }
        }
        out
    }

    /// `String.split(regex)`.
    pub fn split(input: &str, regex: &str) -> Vec<String> {
        split_with(&Pattern::compile(regex).re, input, 0)
    }

    /// `String.split(regex, limit)`.
    pub fn split_limit(input: &str, regex: &str, limit: i32) -> Vec<String> {
        split_with(&Pattern::compile(regex).re, input, limit)
    }

    /// `String.replaceAll(regex, replacement)`.
    pub fn replace_all(input: &str, regex: &str, rep: &str) -> String {
        Pattern::compile(regex).re.replace_all(input, java_replacement(rep).as_str()).into_owned()
    }

    /// `String.replaceFirst(regex, replacement)`.
    pub fn replace_first(input: &str, regex: &str, rep: &str) -> String {
        Pattern::compile(regex).re.replace(input, java_replacement(rep).as_str()).into_owned()
    }

    /// One match (`MatchResult`): the groups as found.
    #[derive(Clone, Debug)]
    pub struct MatchResult {
        groups: Vec<Option<(usize, usize)>>,
        names: Vec<Option<String>>,
        input: String,
    }

    impl MatchResult {
        pub fn group(&self, i: i32) -> Option<String> {
            self.groups.get(i as usize).copied().flatten().map(|(a, b)| self.input[a..b].to_string())
        }
        pub fn group_named(&self, name: &str) -> Option<String> {
            let i = self.names.iter().position(|n| n.as_deref() == Some(name))?;
            self.group(i as i32)
        }
        pub fn start(&self, i: i32) -> i32 {
            self.groups.get(i as usize).copied().flatten().map_or(-1, |(a, _)| self.input[..a].chars().count() as i32)
        }
        pub fn end(&self, i: i32) -> i32 {
            self.groups.get(i as usize).copied().flatten().map_or(-1, |(_, b)| self.input[..b].chars().count() as i32)
        }
        pub fn group_count(&self) -> i32 {
            self.groups.len() as i32 - 1
        }
    }

    /// `java.util.regex.Matcher`: stateful like Java's (`find` advances).
    #[derive(Clone, Debug)]
    pub struct Matcher {
        pattern: Pattern,
        input: String,
        caps: Option<MatchResult>,
        pos: usize,
    }

    impl Matcher {
        fn record(&mut self, full: bool, from: usize) -> bool {
            let re = if full { &self.pattern.full } else { &self.pattern.re };
            let names: Vec<Option<String>> = re.capture_names().map(|n| n.map(str::to_string)).collect();
            match re.captures_from_pos(&self.input, from) {
                Ok(Some(c)) => {
                    let groups: Vec<Option<(usize, usize)>> =
                        (0..c.len()).map(|i| c.get(i).map(|m| (m.start(), m.end()))).collect();
                    let end = groups[0].map_or(from, |(_, b)| b);
                    let start = groups[0].map_or(from, |(a, _)| a);
                    self.pos = if end == start { end + self.input[end..].chars().next().map_or(1, |c| c.len_utf8()) } else { end };
                    self.caps = Some(MatchResult { groups, names, input: self.input.clone() });
                    true
                }
                _ => {
                    self.caps = None;
                    false
                }
            }
        }

        /// `matches()`: the WHOLE input.
        pub fn matches(&mut self) -> bool {
            self.record(true, 0)
        }

        /// `lookingAt()`: a match at the start.
        pub fn looking_at(&mut self) -> bool {
            if self.record(false, 0) {
                if self.caps.as_ref().and_then(|c| c.groups[0]).is_some_and(|(a, _)| a == 0) {
                    return true;
                }
                self.caps = None;
            }
            false
        }

        /// `find()`: the next match.
        pub fn find(&mut self) -> bool {
            if self.pos > self.input.len() {
                self.caps = None;
                return false;
            }
            let from = self.pos;
            self.record(false, from)
        }

        /// `find(start)`.
        pub fn find_from(&mut self, start: i32) -> bool {
            let b = self.input.char_indices().nth(start.max(0) as usize).map_or(self.input.len(), |(b, _)| b);
            self.record(false, b)
        }

        /// `hitEnd()` (approximation: whether the last search reached the end).
        pub fn hit_end(&self) -> bool {
            self.pos >= self.input.len()
        }

        fn caps(&self) -> &MatchResult {
            self.caps.as_ref().expect("IllegalStateException: No match found")
        }

        /// `group(i)`: `None` for a group that did not take part (Java: null).
        pub fn group(&self, i: i32) -> Option<String> {
            self.caps().group(i)
        }

        /// `group(name)`.
        pub fn group_named(&self, name: &str) -> Option<String> {
            self.caps().group_named(name)
        }

        pub fn start(&self, i: i32) -> i32 {
            self.caps().start(i)
        }

        pub fn end(&self, i: i32) -> i32 {
            self.caps().end(i)
        }

        pub fn start_named(&self, name: &str) -> i32 {
            let c = self.caps();
            let i = c.names.iter().position(|n| n.as_deref() == Some(name)).map_or(-1, |i| i as i32);
            c.start(i)
        }

        pub fn end_named(&self, name: &str) -> i32 {
            let c = self.caps();
            let i = c.names.iter().position(|n| n.as_deref() == Some(name)).map_or(-1, |i| i as i32);
            c.end(i)
        }

        pub fn group_count(&self) -> i32 {
            self.pattern.re.captures_len() as i32 - 1
        }

        /// `reset()`.
        pub fn reset(&mut self) -> &mut Matcher {
            self.pos = 0;
            self.caps = None;
            self
        }

        /// `reset(input)`.
        pub fn reset_with(&mut self, input: &str) -> &mut Matcher {
            self.input = input.to_string();
            self.reset()
        }

        /// `usePattern(p)`.
        pub fn use_pattern(&mut self, p: &Pattern) -> &mut Matcher {
            self.pattern = p.clone();
            self
        }

        /// `replaceAll(replacement)`.
        pub fn replace_all(&self, rep: &str) -> String {
            self.pattern.re.replace_all(&self.input, java_replacement(rep).as_str()).into_owned()
        }

        /// `replaceFirst(replacement)`.
        pub fn replace_first(&self, rep: &str) -> String {
            self.pattern.re.replace(&self.input, java_replacement(rep).as_str()).into_owned()
        }

        /// `results()`: every match.
        pub fn results(&mut self) -> Vec<MatchResult> {
            self.reset();
            let mut out = Vec::new();
            while self.find() {
                out.push(self.caps().clone());
            }
            out
        }

        /// `Matcher.quoteReplacement(s)`.
        pub fn quote_replacement(s: &str) -> String {
            s.replace('\\', "\\\\").replace('$', "\\$")
        }
    }
}


/// Gson's `JsonObject` / `JsonElement` / `JsonArray` API on `serde_json`.
/// A missing member or a wrong type panics where Gson throws.
pub mod json {
    pub use serde_json::{Map, Value};

    /// Anything Gson calls a `JsonElement`.
    pub trait JsonRef {
        fn value(&self) -> Value;
    }
    impl JsonRef for Value {
        fn value(&self) -> Value {
            self.clone()
        }
    }
    impl JsonRef for Map<String, Value> {
        fn value(&self) -> Value {
            Value::Object(self.clone())
        }
    }
    impl JsonRef for Vec<Value> {
        fn value(&self) -> Value {
            Value::Array(self.clone())
        }
    }
    impl<T: JsonRef + ?Sized> JsonRef for &T {
        fn value(&self) -> Value {
            (**self).value()
        }
    }
    impl<T: JsonRef + ?Sized> JsonRef for &mut T {
        fn value(&self) -> Value {
            (**self).value()
        }
    }
    impl<T: JsonRef> JsonRef for Option<T> {
        fn value(&self) -> Value {
            self.as_ref().map_or(Value::Null, |v| v.value())
        }
    }

    fn member(j: &impl JsonRef, key: &str) -> Value {
        j.value().get(key).cloned().unwrap_or_else(|| panic!("NullPointerException: no JSON member {key:?}"))
    }

    pub fn has(j: &impl JsonRef, key: &str) -> bool {
        j.value().get(key).is_some()
    }
    pub fn get_as_json_object(j: &impl JsonRef) -> Map<String, Value> {
        match j.value() {
            Value::Object(m) => m,
            v => panic!("IllegalStateException: Not a JSON Object: {v}"),
        }
    }
    pub fn get_as_json_array(j: &impl JsonRef) -> Vec<Value> {
        match j.value() {
            Value::Array(a) => a,
            v => panic!("IllegalStateException: Not a JSON Array: {v}"),
        }
    }
    pub fn get_as_json_primitive(j: &impl JsonRef) -> Value {
        j.value()
    }
    pub fn get_as_string(j: &impl JsonRef) -> String {
        match j.value() {
            Value::String(s) => s,
            Value::Null => panic!("UnsupportedOperationException: JsonNull"),
            v => v.to_string(),
        }
    }
    pub fn get_as_double(j: &impl JsonRef) -> f64 {
        match j.value() {
            Value::Number(n) => n.as_f64().unwrap_or(0.0),
            Value::String(s) => s.trim().parse().expect("NumberFormatException"),
            Value::Bool(b) => panic!("UnsupportedOperationException: {b}"),
            v => panic!("UnsupportedOperationException: {v}"),
        }
    }
    pub fn get_as_number(j: &impl JsonRef) -> f64 {
        get_as_double(j)
    }
    pub fn get_as_float(j: &impl JsonRef) -> f32 {
        get_as_double(j) as f32
    }
    pub fn get_as_int(j: &impl JsonRef) -> i32 {
        match j.value() {
            Value::Number(n) => n.as_i64().map_or_else(|| n.as_f64().unwrap_or(0.0) as i32, |x| x as i32),
            Value::String(s) => s.trim().parse().expect("NumberFormatException"),
            v => panic!("UnsupportedOperationException: {v}"),
        }
    }
    pub fn get_as_long(j: &impl JsonRef) -> i64 {
        match j.value() {
            Value::Number(n) => n.as_i64().unwrap_or_else(|| n.as_f64().unwrap_or(0.0) as i64),
            Value::String(s) => s.trim().parse().expect("NumberFormatException"),
            v => panic!("UnsupportedOperationException: {v}"),
        }
    }
    pub fn get_as_boolean(j: &impl JsonRef) -> bool {
        match j.value() {
            Value::Bool(b) => b,
            Value::String(s) => s.eq_ignore_ascii_case("true"),
            v => panic!("UnsupportedOperationException: {v}"),
        }
    }
    pub fn get_as_json_object_of(j: &impl JsonRef, key: &str) -> Map<String, Value> {
        get_as_json_object(&member(j, key))
    }
    pub fn get_as_json_array_of(j: &impl JsonRef, key: &str) -> Vec<Value> {
        get_as_json_array(&member(j, key))
    }
    pub fn get_as_json_primitive_of(j: &impl JsonRef, key: &str) -> Value {
        member(j, key)
    }
    pub fn get_as_string_of(j: &impl JsonRef, key: &str) -> String {
        get_as_string(&member(j, key))
    }
    pub fn get_as_int_of(j: &impl JsonRef, key: &str) -> i32 {
        get_as_int(&member(j, key))
    }
    pub fn get_as_long_of(j: &impl JsonRef, key: &str) -> i64 {
        get_as_long(&member(j, key))
    }
    pub fn get_as_double_of(j: &impl JsonRef, key: &str) -> f64 {
        get_as_double(&member(j, key))
    }
    pub fn get_as_float_of(j: &impl JsonRef, key: &str) -> f32 {
        get_as_float(&member(j, key))
    }
    pub fn get_as_boolean_of(j: &impl JsonRef, key: &str) -> bool {
        get_as_boolean(&member(j, key))
    }
    pub fn is_object(j: &impl JsonRef) -> bool {
        j.value().is_object()
    }
    pub fn is_array(j: &impl JsonRef) -> bool {
        j.value().is_array()
    }
    pub fn is_primitive(j: &impl JsonRef) -> bool {
        matches!(j.value(), Value::String(_) | Value::Number(_) | Value::Bool(_))
    }
    pub fn is_null(j: &impl JsonRef) -> bool {
        j.value().is_null()
    }
    pub fn to_value(j: impl JsonRef) -> Value {
        j.value()
    }
    pub fn to_string(j: &impl JsonRef) -> String {
        j.value().to_string()
    }
}


/// A Gson tree of the mod (serde_json) for the port's `JsonOps`, whose tree is `lwjgl::json::Value`
/// (json_ops.rs header): the same JSON, re-read.
pub fn json_to_port(v: &serde_json::Value) -> mcrust::com::mojang::serialization::Obj {
    mcrust::com::mojang::serialization::obj(
        mcrust::lwjgl::json::parse(&v.to_string()).expect("a serde_json tree is valid JSON"),
    )
}

/// The other way: what `JsonOps` produced, as the mod's Gson tree.
pub fn json_from_port(o: &mcrust::com::mojang::serialization::Obj) -> serde_json::Value {
    match mcrust::com::mojang::serialization::cast::<mcrust::lwjgl::json::Value>(o) {
        Some(v) => serde_json::from_str(&mcrust::lwjgl::json::to_json(v)).unwrap_or(serde_json::Value::Null),
        None => serde_json::Value::Null,
    }
}



// ===========================================================================
// Gson's reflective serialisation, generated per class (fabric2seer round 24)
// ===========================================================================
//
// Java's `gson.toJson(obj, Config.class)` walks the object's fields by reflection. The converter knows every
// field of every class a `X.class` literal names (and of the classes those fields hold), so it generates each
// class's Gson form instead: `X::__gson_write` (Gson's field order - the class's own fields, then its
// superclass's; `serializeNulls`; transient and static fields skipped), `X::__gson_read` (Gson's
// `ReflectiveTypeAdapter`: a fresh object from the no-argument constructor, then each key present set; unknown keys
// ignored) and `X::__gson_patch` (Dandelion's ConfigPatcher: a dotted field path, a boolean or enum set). A
// `Class<X>` is a `ClassOf<X>` holding those functions.

/// A JSON value as Gson writes it: object members in order, numbers as Java prints them (`1.0` for a float).
#[derive(Clone, Debug, PartialEq)]
pub enum GsonValue {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<GsonValue>),
    Obj(Vec<(String, GsonValue)>),
}

impl GsonValue {
    pub fn int(v: i64) -> GsonValue {
        GsonValue::Num(v.to_string())
    }
    /// a `float` field: `Float.toString`
    pub fn float(v: f32) -> GsonValue {
        GsonValue::Num(mcrust::javacompat::java_float_to_string(v))
    }
    /// a `double` field: `Double.toString`
    pub fn double(v: f64) -> GsonValue {
        GsonValue::Num(mcrust::javacompat::java_double_to_string(v))
    }
    pub fn str(v: &str) -> GsonValue {
        GsonValue::Str(v.to_string())
    }
    /// a map's members: the port's maps keep no order, so the keys are written sorted (Java writes its map's own
    /// iteration order; a reader of the file does not depend on it)
    pub fn sorted_arr(mut members: Vec<GsonValue>) -> GsonValue {
        members.sort_by_cached_key(|m| gson_text(m, false));
        GsonValue::Arr(members)
    }
    pub fn sorted_obj(mut members: Vec<(String, GsonValue)>) -> GsonValue {
        members.sort_by(|a, b| a.0.cmp(&b.0));
        GsonValue::Obj(members)
    }
    /// a value of the port's JSON (`serde_json`), as Gson would print the same tree
    pub fn of_json(v: &serde_json::Value) -> GsonValue {
        match v {
            serde_json::Value::Null => GsonValue::Null,
            serde_json::Value::Bool(b) => GsonValue::Bool(*b),
            serde_json::Value::Number(n) => match (n.as_i64(), n.as_f64()) {
                (Some(i), _) => GsonValue::int(i),
                (None, Some(f)) => GsonValue::double(f),
                _ => GsonValue::Num(n.to_string()),
            },
            serde_json::Value::String(s) => GsonValue::Str(s.clone()),
            serde_json::Value::Array(a) => GsonValue::Arr(a.iter().map(GsonValue::of_json).collect()),
            serde_json::Value::Object(o) => GsonValue::Obj(o.iter().map(|(k, v)| (k.clone(), GsonValue::of_json(v))).collect()),
        }
    }
}

/// Gson's `JsonWriter.string` with `htmlSafe` (Gson's default): `"`, `\`, the control characters, U+2028/U+2029 and
/// `< > & = '` escaped.
fn gson_quote(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\u{c}' => out.push_str("\\f"),
            '<' | '>' | '&' | '=' | '\'' | '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}

fn gson_write_value(v: &GsonValue, pretty: bool, depth: usize, out: &mut String) {
    let nl = |out: &mut String, d: usize| {
        if pretty {
            out.push('\n');
            for _ in 0..d {
                out.push_str("  ");
            }
        }
    };
    match v {
        GsonValue::Null => out.push_str("null"),
        GsonValue::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        GsonValue::Num(n) => out.push_str(n),
        GsonValue::Str(s) => gson_quote(s, out),
        GsonValue::Arr(a) => {
            out.push('[');
            for (i, e) in a.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                nl(out, depth + 1);
                gson_write_value(e, pretty, depth + 1, out);
            }
            if !a.is_empty() {
                nl(out, depth);
            }
            out.push(']');
        }
        GsonValue::Obj(o) => {
            out.push('{');
            for (i, (k, e)) in o.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                nl(out, depth + 1);
                gson_quote(k, out);
                out.push(':');
                if pretty {
                    out.push(' ');
                }
                gson_write_value(e, pretty, depth + 1, out);
            }
            if !o.is_empty() {
                nl(out, depth);
            }
            out.push('}');
        }
    }
}

/// `gson.toJson(..)`: the text Gson writes (`setPrettyPrinting`: two-space indent, `"key": value`).
pub fn gson_text(v: &GsonValue, pretty: bool) -> String {
    let mut out = String::new();
    gson_write_value(v, pretty, 0, &mut out);
    out
}

/// Gson's lenient reads of one value (a number written as a string is still a number, and so on); `None` is a
/// value Gson would reject or a JSON `null`.
pub fn gson_bool(v: &serde_json::Value) -> Option<bool> {
    match v {
        serde_json::Value::Bool(b) => Some(*b),
        serde_json::Value::String(s) => Some(s.eq_ignore_ascii_case("true")),
        _ => None,
    }
}

pub fn gson_i64(v: &serde_json::Value) -> Option<i64> {
    match v {
        serde_json::Value::Number(n) => n.as_i64().or_else(|| n.as_f64().filter(|f| f.fract() == 0.0).map(|f| f as i64)),
        serde_json::Value::String(s) => s.trim().parse::<i64>().ok(),
        _ => None,
    }
}

pub fn gson_f64(v: &serde_json::Value) -> Option<f64> {
    match v {
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::String(s) => s.trim().parse::<f64>().ok(),
        _ => None,
    }
}

pub fn gson_string(v: &serde_json::Value) -> Option<String> {
    match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        serde_json::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// A value of the port that a `Codec` serialises (Gson's `CodecTypeAdapter`: `Component`, `Style`): its JSON.
pub fn gson_by_codec<T>(codec: &mcrust::com::mojang::serialization::codec::CodecRef<T>, v: &T) -> GsonValue {
    use mcrust::com::mojang::serialization::encoder::Encoder as _;
    match codec.encode_start(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), v).result() {
        Some(o) => GsonValue::of_json(&json_from_port(&o)),
        None => GsonValue::Null,
    }
}

pub fn gson_parse_codec<T>(codec: &mcrust::com::mojang::serialization::codec::CodecRef<T>, v: &serde_json::Value) -> Option<T> {
    use mcrust::com::mojang::serialization::decoder::Decoder as _;
    codec.parse(&mcrust::com::mojang::serialization::json_ops::JsonOps::instance(), &json_to_port(v)).result()
}

#[cfg(test)]
mod gson_format_tests {
    use super::*;

    /// what `new GsonBuilder().serializeNulls().setPrettyPrinting().create().toJson(obj)` writes for the same
    /// tree (Gson 2.11): two-space indent, `"key": value`, `[]` / `{}` for empty containers, HTML-safe escapes,
    /// numbers as Java prints them
    #[test]
    fn gson_pretty_text_is_javas() {
        let v = GsonValue::Obj(vec![
            ("version".into(), GsonValue::int(3)),
            ("scale".into(), GsonValue::float(1.5)),
            ("tenth".into(), GsonValue::float(0.1)),
            ("big".into(), GsonValue::double(1.0e10)),
            ("name".into(), GsonValue::str("x<y & \"z\"")),
            ("nothing".into(), GsonValue::Null),
            ("inner".into(), GsonValue::Obj(vec![("enabled".into(), GsonValue::Bool(true)), ("mode".into(), GsonValue::str("OFF"))])),
            ("tags".into(), GsonValue::Arr(vec![])),
            ("counts".into(), GsonValue::sorted_obj(vec![("b".into(), GsonValue::int(2)), ("a".into(), GsonValue::int(1))])),
            ("colour".into(), GsonValue::int(-65536)),
            ("list".into(), GsonValue::Arr(vec![GsonValue::str("a"), GsonValue::float(2.0)])),
            ("empty".into(), GsonValue::Obj(vec![])),
        ]);
        let expected = "{
  \"version\": 3,
  \"scale\": 1.5,
  \"tenth\": 0.1,
  \"big\": 1.0E10,
  \"name\": \"x\\u003cy \\u0026 \\\"z\\\"\",
  \"nothing\": null,
  \"inner\": {
    \"enabled\": true,
    \"mode\": \"OFF\"
  },
  \"tags\": [],
  \"counts\": {
    \"a\": 1,
    \"b\": 2
  },
  \"colour\": -65536,
  \"list\": [
    \"a\",
    2.0
  ],
  \"empty\": {}
}";
        assert_eq!(gson_text(&v, true), expected);
        assert_eq!(gson_text(&GsonValue::Arr(vec![GsonValue::int(1), GsonValue::Null]), false), "[1,null]");
    }

    #[test]
    fn gson_lenient_reads() {
        assert_eq!(gson_i64(&serde_json::json!(3.0)), Some(3));
        assert_eq!(gson_i64(&serde_json::json!("7")), Some(7));
        assert_eq!(gson_bool(&serde_json::json!("TRUE")), Some(true));
        assert_eq!(gson_string(&serde_json::json!(5)), Some("5".to_string()));
        assert_eq!(gson_i64(&serde_json::Value::Null), None);
    }
}

/// A Dandelion config patch's value (`ConfigPatch.OfBoolean` / `OfEnum`).
#[derive(Clone, Debug, PartialEq)]
pub enum PatchValue {
    Bool(bool),
    Enum(String),
}

/// Java's `Class<T>` for a class the converter generated the Gson form of: what reflection would answer.
pub struct ClassOf<T> {
    pub name: &'static str,
    /// `getDeclaredConstructor().newInstance()`
    pub new: fn() -> T,
    pub to_gson: fn(&T) -> GsonValue,
    /// `gson.fromJson(json, Class)`: `None` for JSON `null` or a value Gson would reject
    pub read_gson: fn(&serde_json::Value) -> Option<T>,
    /// a dotted field path set to a boolean / an enum constant (Dandelion's ConfigPatcher); false where the
    /// reflective walk would fail
    pub patch: fn(&mut T, &[&str], &PatchValue) -> bool,
    pub is_enum: bool,
    pub enum_constants: fn() -> Vec<String>,
}

impl<T> Clone for ClassOf<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ClassOf<T> {}

impl<T> std::fmt::Debug for ClassOf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {}", self.name)
    }
}

impl<T> std::fmt::Display for ClassOf<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "class {}", self.name)
    }
}

impl<T> PartialEq for ClassOf<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T> ClassOf<T> {
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
    pub fn get_simple_name(&self) -> String {
        self.name.rsplit(['.', '$']).next().unwrap_or(self.name).to_string()
    }
    pub fn is_enum(&self) -> bool {
        self.is_enum
    }
    pub fn new_instance(&self) -> T {
        (self.new)()
    }
}

/// `com.google.gson.FieldNamingPolicy` - the two policies YACL configures. The generated per-class forms write the
/// Java field names (`IDENTITY`); a form that honours the builder's policy names each field through
/// `translate_name` (GAPS_Y2.md G13: the converter must generate that, for nested objects too).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldNamingPolicy {
    /// `IDENTITY`
    Identity,
    /// `LOWER_CASE_WITH_UNDERSCORES` - YACL's default for `GsonConfigSerializer` and `GsonConfigInstance`
    LowerCaseWithUnderscores,
}

impl FieldNamingPolicy {
    /// `translateName(Field f)` over the field's Java name: `LOWER_CASE_WITH_UNDERSCORES` is
    /// `separateCamelCase(name, '_').toLowerCase(Locale.ENGLISH)` - a `_` before every upper-case letter that is not
    /// the first character.
    pub fn translate_name(self, name: &str) -> String {
        match self {
            FieldNamingPolicy::Identity => name.to_string(),
            FieldNamingPolicy::LowerCaseWithUnderscores => {
                let mut out = String::with_capacity(name.len() + 4);
                for c in name.chars() {
                    if c.is_uppercase() && !out.is_empty() {
                        out.push('_');
                    }
                    out.push(c);
                }
                out.to_lowercase()
            }
        }
    }
}

/// `new GsonBuilder()..create()`: the options the generated forms honour (`setPrettyPrinting`; `serializeNulls` and
/// `IDENTITY` field names are how they are written; the type adapters a config uses are built into each form).
#[derive(Clone, Debug, Default)]
pub struct GsonBuilder {
    pub pretty: bool,
    /// `setFieldNamingPolicy(policy)` - `None` is Gson's default, `IDENTITY`
    pub field_naming_policy: Option<FieldNamingPolicy>,
}

impl GsonBuilder {
    pub fn new() -> GsonBuilder {
        GsonBuilder::default()
    }
    pub fn set_pretty_printing(mut self) -> GsonBuilder {
        self.pretty = true;
        self
    }
    pub fn serialize_nulls(self) -> GsonBuilder {
        self
    }
    pub fn set_field_naming_policy(mut self, policy: FieldNamingPolicy) -> GsonBuilder {
        self.field_naming_policy = Some(policy);
        self
    }
    pub fn disable_html_escaping(self) -> GsonBuilder {
        self
    }
    /// `registerTypeHierarchyAdapter(Color.class, adapter)` & co: the generated forms carry Dandelion's five
    /// adapters (Color, Component, Style, Identifier, Item) for every field of those types
    pub fn register_type_hierarchy_adapter(self) -> GsonBuilder {
        self
    }
    pub fn create(self) -> Gson {
        Gson { pretty: self.pretty, field_naming_policy: self.field_naming_policy.unwrap_or(FieldNamingPolicy::Identity) }
    }
}

/// `com.google.gson.Gson` over the generated per-class forms.
#[derive(Clone, Debug)]
pub struct Gson {
    pub pretty: bool,
    /// `fieldNamingStrategy()` - what a form that honours it names fields with (`FieldNamingPolicy::translate_name`)
    pub field_naming_policy: FieldNamingPolicy,
}

impl Default for Gson {
    fn default() -> Gson {
        Gson { pretty: false, field_naming_policy: FieldNamingPolicy::Identity }
    }
}

impl Gson {
    pub fn new() -> Gson {
        Gson::default()
    }
    /// `toJson(src, Class)`
    pub fn to_json<T>(&self, src: &T, class: &ClassOf<T>) -> String {
        gson_text(&(class.to_gson)(src), self.pretty)
    }
    /// `fromJson(JsonElement, Class)`
    pub fn from_json_tree<T>(&self, json: &serde_json::Value, class: &ClassOf<T>) -> Option<T> {
        (class.read_gson)(json)
    }
    /// `fromJson(String, Class)`
    pub fn from_json<T>(&self, json: &str, class: &ClassOf<T>) -> Option<T> {
        let v: serde_json::Value = serde_json::from_str(json).expect("JsonSyntaxException");
        (class.read_gson)(&v)
    }
}


/// A Java REFERENCE to an object of the mod's own class hierarchy: shared by every holder (Java aliasing -
/// the same waypoint in a list and in a map, a widget returned after it was stored), mutable through any
/// of them. The lock is RE-ENTRANT, so a method that reaches its own object again on the same thread does
/// not hang; a conflicting borrow (a `&mut self` method re-entered) panics loudly instead.
pub type Shared<T: ?Sized> = std::sync::Arc<parking_lot::ReentrantMutex<std::cell::RefCell<T>>>;

/// `new X(..)` as a shared reference (cast to `Shared<dyn Trait>` where a trait object is held).
/// The object's own handle, kept by the object (Java's `this` handed to other objects).
pub type WeakShared<T: ?Sized> = std::sync::Weak<parking_lot::ReentrantMutex<std::cell::RefCell<T>>>;

/// Reads the object behind a shared reference: the borrow lasts exactly as long as `f`, never across a call
/// that may reach the same object again.
pub fn read<T: ?Sized, R>(s: &Shared<T>, f: impl FnOnce(&T) -> R) -> R {
    let guard = s.lock();
    let value = guard.borrow();
    f(&value)
}

/// Writes the object behind a shared reference, the borrow lasting exactly as long as `f`.
pub fn write<T: ?Sized, R>(s: &Shared<T>, f: impl FnOnce(&mut T) -> R) -> R {
    let guard = s.lock();
    let mut value = guard.borrow_mut();
    f(&mut value)
}

/// An object of the mod that keeps its own handle (`self_ref`): told it when it is put behind one.
pub trait HasSelfRef {
    fn set_self_ref(&mut self, handle: WeakShared<Self>);
}

/// `new X(..)` of a class the mod extends: the object behind its handle, knowing its handle.
pub fn shared_self<T: HasSelfRef>(value: T) -> Shared<T> {
    let handle = shared(value);
    let weak = std::sync::Arc::downgrade(&handle);
    write(&handle, |t| t.set_self_ref(weak));
    handle
}

/// Fetches what a call on a shared object needs (its trait method's body, a function pointer) under a
/// momentary borrow; the call itself then runs with nothing borrowed.
pub fn dispatch<T: ?Sized, R>(s: &Shared<T>, f: impl FnOnce(&T) -> R) -> R {
    read(s, f)
}

/// Every object of the mod's class hierarchy can be looked into as its concrete class (Java's checked cast);
/// a supertrait of each root trait of the mod, implemented for every type (no per-class code).
pub trait DynAny: std::any::Any {
    fn dyn_any(&self) -> &dyn std::any::Any;
    fn dyn_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: std::any::Any> DynAny for T {
    fn dyn_any(&self) -> &dyn std::any::Any {
        self
    }
    fn dyn_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Reads a field of the concrete class behind a trait-object handle (a per-statement downcast).
pub fn read_as<C: std::any::Any, D: ?Sized + DynAny, R>(s: &Shared<D>, f: impl FnOnce(&C) -> R) -> R {
    read(s, |t| f(t.dyn_any().downcast_ref::<C>().expect("compat::read_as: not an object of this class")))
}

/// Writes a field of the concrete class behind a trait-object handle (a per-statement downcast).
pub fn write_as<C: std::any::Any, D: ?Sized + DynAny, R>(s: &Shared<D>, f: impl FnOnce(&mut C) -> R) -> R {
    write(s, |t| f(t.dyn_any_mut().downcast_mut::<C>().expect("compat::write_as: not an object of this class")))
}

/// The same handle seen as another type (a trait object of a supertrait, the concrete class), borrowed from
/// the original: no allocation and no reference count taken. It lives no longer than the handle it views.
pub struct View<'a, T: ?Sized>(std::mem::ManuallyDrop<Shared<T>>, std::marker::PhantomData<&'a ()>);

impl<T: ?Sized> std::ops::Deref for View<'_, T> {
    type Target = Shared<T>;
    fn deref(&self) -> &Shared<T> {
        &self.0
    }
}

/// `h` as a handle of a supertrait (`|a| a as Shared<dyn Super>`, an unsizing coercion: the same allocation).
pub fn view<'a, A: ?Sized, B: ?Sized>(h: &'a Shared<A>, up: fn(Shared<A>) -> Shared<B>) -> View<'a, B> {
    // SAFETY: the Arc is rebuilt from the handle's own pointer without taking a count and never dropped
    // (ManuallyDrop); `up` is an unsizing coercion, so it is the same allocation, borrowed for 'a.
    let same = unsafe { std::sync::Arc::from_raw(std::sync::Arc::as_ptr(h)) };
    View(std::mem::ManuallyDrop::new(up(same)), std::marker::PhantomData)
}

/// `h` (a trait object) as a handle of its concrete class `C` (checked, as Java's cast), then `up` to the
/// handle type wanted (`|a| a` for `C` itself, `|a| a as Shared<dyn T>` for another trait `C` implements).
pub fn view_as<'a, C: std::any::Any, D: ?Sized + DynAny, B: ?Sized>(h: &'a Shared<D>, up: fn(Shared<C>) -> Shared<B>)
        -> View<'a, B> {
    assert!(read(h, |t| t.dyn_any().is::<C>()), "compat::view_as: not an object of this class");
    let p = std::sync::Arc::as_ptr(h) as *const parking_lot::ReentrantMutex<std::cell::RefCell<C>>;
    // SAFETY: the object behind the handle IS a `C` (checked above), so this is the allocation's own type;
    // no count is taken and the Arc is never dropped (ManuallyDrop); borrowed for 'a.
    let same = unsafe { std::sync::Arc::from_raw(p) };
    View(std::mem::ManuallyDrop::new(up(same)), std::marker::PhantomData)
}

/// `h` (a trait object) as its concrete class `C` when it is one (Java's `instanceof` + cast), borrowed from
/// the handle.
pub fn try_view<'a, C: std::any::Any, D: ?Sized + DynAny>(h: &'a Shared<D>) -> Option<View<'a, C>> {
    if !read(h, |t| t.dyn_any().is::<C>()) {
        return None;
    }
    Some(view_as::<C, D, C>(h, |a| a))
}

/// `h` (a trait object) as an owned handle of its concrete class `C` (Java's `this` handed on from a body
/// that runs on a trait object): the same object, its reference count taken.
pub fn handle_as<C: std::any::Any, D: ?Sized + DynAny>(h: &Shared<D>) -> Shared<C> {
    std::sync::Arc::clone(&view_as::<C, D, C>(h, |a| a))
}

/// A value a closure carries that the port types `Send + Sync` (a button's action, a scheduled task): the mod's
/// objects are not `Send`, and Java only touches them on the render thread - which this cell checks on every use.
pub struct RenderThread<T> {
    value: T,
    owner: std::thread::ThreadId,
}

// SAFETY: the value is reached only through `get`, which panics off the thread that made the cell.
unsafe impl<T> Send for RenderThread<T> {}
unsafe impl<T> Sync for RenderThread<T> {}

impl<T> RenderThread<T> {
    pub fn new(value: T) -> RenderThread<T> {
        RenderThread { value, owner: std::thread::current().id() }
    }

    pub fn get(&self) -> &T {
        assert!(std::thread::current().id() == self.owner, "a mod object was reached off the render thread");
        &self.value
    }
}

impl<T: Clone> Clone for RenderThread<T> {
    fn clone(&self) -> RenderThread<T> {
        RenderThread { value: self.get().clone(), owner: self.owner }
    }
}

/// A mod class that extends the game's `AbstractWidget` (or `AbstractButton`, ...): its widget half (`base`). The
/// game keeps the live widget - x, y, size, message, `active`, `visible` - in the screen's list and LENDS it to
/// the object for each of its methods (`WidgetAdapter::lend`), so `this.getX()` / `this.active = false` reach it.
pub trait WidgetHalf {
    fn widget_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
}

/// The fields of a widget the mod's code sets from outside the widget's own methods.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct WidgetSnap {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    message: String,
    active: bool,
    visible: bool,
    alpha: f32,
}

impl WidgetSnap {
    pub fn of(w: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> WidgetSnap {
        WidgetSnap {
            x: w.get_x(),
            y: w.get_y(),
            width: w.width,
            height: w.height,
            message: w.message.clone(),
            active: w.active,
            visible: w.visible,
            alpha: w.alpha,
        }
    }

    /// What `from` changed since this snapshot, written into `to`.
    fn apply(&self, from: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
             to: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) {
        let now = WidgetSnap::of(from);
        if now.x != self.x {
            to.set_x(now.x);
        }
        if now.y != self.y {
            to.set_y(now.y);
        }
        if now.width != self.width {
            to.width = now.width;
        }
        if now.height != self.height {
            to.height = now.height;
        }
        if now.message != self.message {
            to.message = from.message.clone();
            to.message_text = from.message_text.clone();
        }
        if now.active != self.active {
            to.active = now.active;
        }
        if now.visible != self.visible {
            to.visible = now.visible;
        }
        if now.alpha != self.alpha {
            to.alpha = now.alpha;
        }
    }
}

/// A mod's widget object as the game's `ModWidget`: its handle (the same object the mod's code holds). Checked to
/// the render thread on every call (the mod's objects are not `Send`).
pub struct WidgetAdapter<C: 'static> {
    object: Shared<C>,
    owner: std::thread::ThreadId,
    /// the object's widget half as of the end of the last call: what the mod's code changes between calls is
    /// told apart from what the game changes (a layout moving the widget) and brought in at the next call
    snap: std::cell::RefCell<WidgetSnap>,
}

// SAFETY: every use goes through `check`, which pins the adapter to the thread that made it.
unsafe impl<C: 'static> Send for WidgetAdapter<C> {}

impl<C: WidgetHalf + 'static> WidgetAdapter<C> {
    pub fn new(object: Shared<C>) -> WidgetAdapter<C> {
        let snap = write(&object, |t| WidgetSnap::of(t.widget_half()));
        WidgetAdapter { object, owner: std::thread::current().id(), snap: std::cell::RefCell::new(snap) }
    }

    /// `new MyWidget(..)` handed to the game (`addRenderableWidget`, a layout): the game's widget, built from
    /// the object's own fields, with the object attached.
    pub fn widget(object: Shared<C>) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget
    where
        WidgetAdapter<C>: mcrust::mcrust::modding::widgets::ModWidget,
    {
        let mut w = write(&object, |t| t.widget_half().clone());
        w.kind = mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::Mod(
            mcrust::mcrust::modding::widgets::ModWidgetSlot::new(Box::new(WidgetAdapter::new(object))),
        );
        w
    }

    /// The mod object.
    pub fn object(&self) -> &Shared<C> {
        self.check();
        &self.object
    }

    fn check(&self) {
        assert!(std::thread::current().id() == self.owner, "a mod widget was reached off the render thread");
    }

    /// One of the object's methods with the game's widget lent to it (swapped into its `base`, back after - also
    /// on unwind); what the mod's code changed since the last call reaches the game's widget first.
    pub fn lend<R>(
        &self,
        widget: &mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        struct Back<'a, C: WidgetHalf>(
            &'a Shared<C>,
            &'a mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
            &'a std::cell::RefCell<WidgetSnap>,
        );
        impl<C: WidgetHalf> Drop for Back<'_, C> {
            fn drop(&mut self) {
                let lent = &mut *self.1;
                let snap = self.2;
                write(self.0, |t| {
                    std::mem::swap(t.widget_half(), lent);
                    // the object keeps a copy of the game's widget, but not the game's link to the object itself
                    let half = t.widget_half();
                    let kind = std::mem::replace(&mut half.kind, mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainText);
                    *half = lent.clone();
                    half.kind = kind;
                    *snap.borrow_mut() = WidgetSnap::of(half);
                });
            }
        }
        write(&self.object, |t| {
            self.snap.borrow().apply(t.widget_half(), widget);
            std::mem::swap(t.widget_half(), widget);
        });
        let _back = Back(&self.object, widget, &self.snap);
        f(&self.object)
    }

    /// A method that only READS the game's widget (`extractWidgetRenderState`, narration): the object's half is
    /// brought up to date first, keeping what the mod's code changed and the game has not seen yet.
    pub fn lend_ref<R>(
        &self,
        widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        write(&self.object, |t| {
            let mut fresh = widget.clone();
            self.snap.borrow().apply(t.widget_half(), &mut fresh);
            let half = t.widget_half();
            fresh.kind = std::mem::replace(&mut half.kind, mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::PlainText);
            *half = fresh;
        });
        f(&self.object)
    }

    /// A method that does not touch the widget half (`isValidClickButton`).
    pub fn call<R>(&self, f: impl FnOnce(&Shared<C>) -> R) -> R {
        self.check();
        f(&self.object)
    }
}

/// A mod's `ObjectSelectionList` / `ContainerObjectSelectionList` subclass: the port's `ModSelectionList` it is
/// built on (`base`).
pub trait ListHalf {
    fn list_half(&mut self) -> &mut mcrust::mcrust::modding::widgets::ModSelectionList;
}

/// A mod's list subclass as the port's list CLASS (`widgets::ModList`): its handle, asked by the port's list
/// bodies at every statement Java calls an overridable method from (`getRowWidth()`, `scrollBarX()`, ...). The
/// list itself - the rows, the scroll, the box - is the game's widget (`attach_list`); the object keeps the
/// mod's own fields.
pub struct ListClass<L: 'static> {
    object: RenderThread<Shared<L>>,
}

thread_local! {
    static SUPER_ANSWER: std::cell::RefCell<Vec<Box<dyn std::any::Any>>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// `super.getRowWidth()` (& co) inside a list class's override: the answer of the class it extends, which the
/// port hands the override (`ListClass::ask`).
pub fn super_answer<T: Clone + 'static>() -> T {
    SUPER_ANSWER.with(|s| {
        s.borrow()
            .last()
            .and_then(|v| v.downcast_ref::<T>().cloned())
            .expect("super.x() of a list override outside the call that hands it its super answer")
    })
}

impl<L: ListHalf + 'static> ListClass<L> {
    pub fn new(object: Shared<L>) -> ListClass<L> {
        ListClass { object: RenderThread::new(object) }
    }

    /// The mod object (`instanceof MyList`: `list.class::<ListClass<MyList>>()`).
    pub fn object(&self) -> &Shared<L> {
        self.object.get()
    }

    /// One override, asked with the list's current state (the object's own view of its box - `this.getWidth()`
    /// inside the override - brought up to date first) and the base class's answer for `super.x()`.
    pub fn ask<T: Clone + 'static, R>(
        &self,
        list: &mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList,
        super_answer: T,
        f: impl FnOnce(&Shared<L>) -> R,
    ) -> R {
        let object = self.object.get();
        write(object, |t| {
            let mut view = list.clone();
            view.set_class(None); // the object's copy holds no link back to the object
            t.list_half().list = view;
        });
        SUPER_ANSWER.with(|s| s.borrow_mut().push(Box::new(super_answer)));
        struct Pop;
        impl Drop for Pop {
            fn drop(&mut self) {
                SUPER_ANSWER.with(|s| {
                    s.borrow_mut().pop();
                });
            }
        }
        let _pop = Pop;
        f(object)
    }
}

/// `addRenderableWidget(list)` (or a field the screen holds it in): the game's widget, the rows the object built
/// moved into it, the object attached as the list's class. From here on the LIVE list is the game's; the
/// object's code reaches it through its widget reference (`lend_list`).
pub fn attach_list<L: ListHalf + 'static>(object: Shared<L>) -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget
where
    ListClass<L>: mcrust::mcrust::modding::widgets::ModList,
{
    let mut list = write(&object, |t| {
        let kind = t.list_half().kind;
        std::mem::replace(t.list_half(), mcrust::mcrust::modding::widgets::ModSelectionList::new(kind, 0, 0, 0, 0))
    });
    let (x, y, w, h) = (list.list.get_x(), list.list.get_y(), list.list.get_width(), list.list.get_height());
    list.list.set_class(Some(mcrust::mcrust::modding::widgets::ModListClass(std::sync::Arc::new(ListClass::new(object)))));
    mcrust::mcrust::modding::widgets::ModWidgetSlot::widget(x, y, w, h, String::new(), Box::new(list))
}

/// One of the list object's own methods with the live list (the game's) lent to it for the call.
pub fn lend_list<L: ListHalf, R>(
    object: &Shared<L>,
    list: &mut mcrust::mcrust::modding::widgets::ModSelectionList,
    f: impl FnOnce() -> R,
) -> R {
    struct Back<'a, L: ListHalf>(&'a Shared<L>, &'a mut mcrust::mcrust::modding::widgets::ModSelectionList);
    impl<L: ListHalf> Drop for Back<'_, L> {
        fn drop(&mut self) {
            let lent = &mut *self.1;
            write(self.0, |t| std::mem::swap(t.list_half(), lent));
        }
    }
    write(object, |t| std::mem::swap(t.list_half(), list));
    let _back = Back(object, list);
    f()
}

/// The mod list object attached to a game widget (`list` the screen holds by reference).
pub fn list_object<L: ListHalf + 'static>(
    widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
) -> Option<Shared<L>>
where
    ListClass<L>: mcrust::mcrust::modding::widgets::ModList,
{
    widget.mod_widget()?.with::<mcrust::mcrust::modding::widgets::ModSelectionList, _>(|l| {
        l.class::<ListClass<L>>().map(|c| c.object().clone())
    }).flatten()
}

/// The mod entry object at a row of the game's list (a `ModList` override's `list`).
pub fn row_object<E: ?Sized + 'static>(
    list: &mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList,
    index: usize,
) -> Option<Shared<E>>
where
    EntryAdapter<E>: mcrust::mcrust::modding::widgets::ModListEntry,
{
    list.entry_object(index)?.with::<EntryAdapter<E>, _>(|a| a.object.clone())
}

/// The row of a mod entry object in the game's list (a `ModList` override's answer).
pub fn row_index<E: ?Sized + 'static>(
    list: &mcrust::net::minecraft::client::gui::components::abstract_selection_list::AbstractSelectionList,
    entry: &Shared<E>,
) -> Option<usize>
where
    EntryAdapter<E>: mcrust::mcrust::modding::widgets::ModListEntry,
{
    (0..list.entry_count()).position(|i| row_object::<E>(list, i).is_some_and(|o| std::sync::Arc::ptr_eq(&o, entry)))
}

/// The rows of a mod list as the mod's entry objects (`children()`).
pub fn list_entries<E: ?Sized + 'static>(list: &mcrust::mcrust::modding::widgets::ModSelectionList) -> Vec<Shared<E>>
where
    EntryAdapter<E>: mcrust::mcrust::modding::widgets::ModListEntry,
{
    (0..list.entry_count()).filter_map(|i| entry_at::<E>(list, i)).collect()
}

/// The row index of a mod entry object in its list (`removeEntry(e)`, `setSelected(e)`).
pub fn entry_index<E: ?Sized + 'static>(list: &mcrust::mcrust::modding::widgets::ModSelectionList, entry: &Shared<E>) -> Option<usize>
where
    EntryAdapter<E>: mcrust::mcrust::modding::widgets::ModListEntry,
{
    (0..list.entry_count()).position(|i| entry_at::<E>(list, i).is_some_and(|o| std::sync::Arc::ptr_eq(&o, entry)))
}

/// The mod entry object at a row (`getSelected()`).
pub fn entry_at<E: ?Sized + 'static>(list: &mcrust::mcrust::modding::widgets::ModSelectionList, index: usize) -> Option<Shared<E>>
where
    EntryAdapter<E>: mcrust::mcrust::modding::widgets::ModListEntry,
{
    list.entry_object(index)?.with::<EntryAdapter<E>, _>(|a| a.object.clone())
}

/// A mod's list row class (`ContainerObjectSelectionList.Entry` subclass): its row box (`base`) and the widgets
/// it holds (`__row`, a `Screen` as the port's container rows keep theirs, and their references' cache).
pub trait RowHalf {
    fn entry_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry;
    fn row_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, &mut WidgetCache);
}

/// A mod's row object as the game's `ModListEntry`: its handle, and the row's widgets, which the game reaches
/// through `children()` - lent to the object for each of its methods.
pub struct EntryAdapter<E: ?Sized + 'static> {
    object: Shared<E>,
    pub row: mcrust::net::minecraft::client::gui::screens::screen::Screen,
    owner: std::thread::ThreadId,
}

// SAFETY: every use goes through `lend` / `call`, which check the thread.
unsafe impl<E: ?Sized + 'static> Send for EntryAdapter<E> {}

impl<E: RowHalf + ?Sized + 'static> EntryAdapter<E> {
    /// `addEntry(e)`: the row's widgets move to the adapter (the game draws and clicks them there).
    pub fn new(object: Shared<E>) -> EntryAdapter<E> {
        let row = write(&object, |t| {
            std::mem::replace(t.row_parts().0, mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new()))
        });
        EntryAdapter { object, row, owner: std::thread::current().id() }
    }

    pub fn object(&self) -> &Shared<E> {
        &self.object
    }

    /// One of the row object's methods with its widgets (and, when the game hands it, its row box) lent to it.
    pub fn lend<R>(
        &mut self,
        entry: Option<&mcrust::net::minecraft::client::gui::components::abstract_selection_list::Entry>,
        f: impl FnOnce(&Shared<E>) -> R,
    ) -> R {
        assert!(std::thread::current().id() == self.owner, "a mod list row was reached off the render thread");
        if let Some(entry) = entry {
            write(&self.object, |t| *t.entry_half() = entry.clone());
        }
        lend_row(&self.object, &mut self.row, || f(&self.object))
    }
}

/// The row's widgets lent to the row object for `f` (swapped into its `__row`, back after - also on unwind).
pub fn lend_row<E: RowHalf + ?Sized, R>(
    object: &Shared<E>,
    row: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
    f: impl FnOnce() -> R,
) -> R {
    struct Back<'a, E: RowHalf + ?Sized>(&'a Shared<E>, &'a mut mcrust::net::minecraft::client::gui::screens::screen::Screen);
    impl<E: RowHalf + ?Sized> Drop for Back<'_, E> {
        fn drop(&mut self) {
            let lent = &mut *self.1;
            write(self.0, |t| {
                let (screen, cache) = t.row_parts();
                cache.end(screen);
                std::mem::swap(screen, lent);
            });
        }
    }
    write(object, |t| {
        let (screen, cache) = t.row_parts();
        std::mem::swap(screen, row);
        cache.begin(screen);
    });
    let _back = Back(object, row);
    f()
}

/// A mod class that extends the game's `Slot`: its slot half (`base`).
pub trait SlotHalf {
    fn slot_half(&mut self) -> &mut mcrust::net::minecraft::world::inventory::slot::Slot;
}

/// A mod's `Slot` subclass as the port's slot class (`container_menus::ModSlot`): its handle, asked with the
/// slot's current state (copied into the object's `base`, so `this.y` reads it) and the base class's answer.
pub struct SlotClass<C: 'static> {
    object: RenderThread<Shared<C>>,
}

impl<C: SlotHalf + 'static> SlotClass<C> {
    pub fn new(object: Shared<C>) -> SlotClass<C> {
        SlotClass { object: RenderThread::new(object) }
    }

    pub fn object(&self) -> &Shared<C> {
        self.object.get()
    }

    pub fn ask<T: Clone + 'static, R>(
        &self,
        slot: &mcrust::net::minecraft::world::inventory::slot::Slot,
        super_answer: T,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        let object = self.object.get();
        write(object, |t| *t.slot_half() = slot.clone());
        SUPER_ANSWER.with(|s| s.borrow_mut().push(Box::new(super_answer)));
        struct Pop;
        impl Drop for Pop {
            fn drop(&mut self) {
                SUPER_ANSWER.with(|s| {
                    s.borrow_mut().pop();
                });
            }
        }
        let _pop = Pop;
        f(object)
    }
}

/// `new MySlot(container, index, x, y)` handed to the game (`slots.set(i, slot)`): the game's slot, built from
/// the object's own fields, with the object attached as its class.
pub fn slot_with_class<C: SlotHalf + 'static>(object: Shared<C>) -> mcrust::net::minecraft::world::inventory::slot::Slot
where
    SlotClass<C>: mcrust::mcrust::modding::container_menus::ModSlot,
{
    let slot = write(&object, |t| t.slot_half().clone());
    slot.with_mod_slot(std::sync::Arc::new(SlotClass::new(object)))
}

/// An anonymous `new Slot(..) { boolean isActive() {..} }`: its overrides as closures over the slot (Java's
/// `this`) and the base class's answer.
#[derive(Default)]
pub struct SlotFns {
    pub is_active: Option<Box<dyn Fn(&mcrust::net::minecraft::world::inventory::slot::Slot, bool) -> bool + Send + Sync>>,
    pub is_highlightable: Option<Box<dyn Fn(&mcrust::net::minecraft::world::inventory::slot::Slot, bool) -> bool + Send + Sync>>,
    pub may_pickup: Option<Box<dyn Fn(&mcrust::net::minecraft::world::inventory::slot::Slot, bool) -> bool + Send + Sync>>,
    pub get_max_stack_size: Option<Box<dyn Fn(&mcrust::net::minecraft::world::inventory::slot::Slot, i32) -> i32 + Send + Sync>>,
}

impl mcrust::mcrust::modding::container_menus::ModSlot for SlotFns {
    fn is_active(&self, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, super_answer: bool) -> bool {
        self.is_active.as_ref().map_or(super_answer, |f| f(slot, super_answer))
    }
    fn is_highlightable(&self, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, super_answer: bool) -> bool {
        self.is_highlightable.as_ref().map_or(super_answer, |f| f(slot, super_answer))
    }
    fn may_pickup(&self, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, super_answer: bool) -> bool {
        self.may_pickup.as_ref().map_or(super_answer, |f| f(slot, super_answer))
    }
    fn get_max_stack_size(&self, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, super_answer: i32) -> i32 {
        self.get_max_stack_size.as_ref().map_or(super_answer, |f| f(slot, super_answer))
    }
}

// ---------------------------------------------------------------------------------------------------------------
// Mod menus (`class MyHandler extends ChestMenu`) and mod container screens (`extends AbstractContainerScreen`)
// ---------------------------------------------------------------------------------------------------------------

/// Java's `player.getInventory()` handed to a mod menu's or mod container screen's constructor that only hands it
/// on to the game's base (`super(type, id, playerInventory, ..)`): the port builds that base over the menu and
/// screen the game opened, which hold the local player's inventory already - so the argument names it, it is not a
/// copy of it. (`game::with_player` reaches the inventory itself.)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlayerInventory;

/// A layout the mod holds, lent where the port's function takes `&mut LayoutElement` (`centerInRectangle`): the
/// element is the layout, written back into the mod's value when the borrow ends (Java moves the object itself).
pub struct LayoutBack<'a> {
    element: mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement,
    grid: Option<&'a mut mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout>,
    frame: Option<&'a mut mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout>,
}

impl<'a> LayoutBack<'a> {
    pub fn grid(layout: &'a mut mcrust::net::minecraft::client::gui::layouts::grid_layout::GridLayout) -> LayoutBack<'a> {
        let element = mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement::Grid(layout.clone());
        LayoutBack { element, grid: Some(layout), frame: None }
    }

    pub fn frame(layout: &'a mut mcrust::net::minecraft::client::gui::layouts::frame_layout::FrameLayout) -> LayoutBack<'a> {
        let element = mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement::Frame(layout.clone());
        LayoutBack { element, grid: None, frame: Some(layout) }
    }
}

impl std::ops::Deref for LayoutBack<'_> {
    type Target = mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl std::ops::DerefMut for LayoutBack<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl Drop for LayoutBack<'_> {
    fn drop(&mut self) {
        use mcrust::net::minecraft::client::gui::layouts::layout_element::LayoutElement;
        match &self.element {
            LayoutElement::Grid(g) => {
                if let Some(t) = self.grid.as_deref_mut() {
                    *t = g.clone();
                }
            }
            LayoutElement::Frame(f) => {
                if let Some(t) = self.frame.as_deref_mut() {
                    *t = f.clone();
                }
            }
            _ => {}
        }
    }
}

/// A `ChestMenu` a mod keeps (`this.handler = handler`): Java holds the menu object; the port's open menu is the
/// player's, reached each time through `game::with_chest_menu` while it is still the one whose container id was
/// recorded (`None` once the server closed it or opened another - Java would read the closed object).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LiveChestMenu {
    pub container_id: i32,
}

impl LiveChestMenu {
    pub fn of(menu: &mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu) -> LiveChestMenu {
        LiveChestMenu { container_id: menu.menu.container_id }
    }

    pub fn with<R>(&self, f: impl FnOnce(&mut mcrust::net::minecraft::world::inventory::chest_menu::ChestMenu) -> R) -> Option<R> {
        let id = self.container_id;
        mcrust::mcrust::modding::game::with_chest_menu(|chest| (chest.menu.container_id == id).then(|| f(chest))).flatten()
    }
}

/// A mod class that extends the game's container menu: its menu half (`base`), the LIVE menu while the game lends
/// it (the constructor's body, `ModMenu::init`).
pub trait MenuHalf {
    fn menu_half(&mut self) -> &mut mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu;
}

thread_local! {
    /// the constructor bodies of mod menu objects built but not yet attached, keyed by the object
    static PENDING_MENUS: std::cell::RefCell<Vec<(usize, Box<dyn FnOnce()>)>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// `new MyHandler(..)`: Java runs the constructor's body on the menu `super(..)` built; the port's menu is the one
/// the server opened, lent to the object when it is attached (`ModMenu::init`), so the body is kept until then.
/// Bodies of a `this(..)` chain run in Java's order (the delegate's first).
pub fn defer_menu_init<M: 'static>(object: &Shared<M>, body: impl FnOnce(&Shared<M>) + 'static) {
    let key = std::sync::Arc::as_ptr(object) as *const () as usize;
    let handle = object.clone();
    PENDING_MENUS.with(|p| p.borrow_mut().push((key, Box::new(move || body(&handle)))));
}

fn take_menu_inits<M: 'static>(object: &Shared<M>) -> Vec<Box<dyn FnOnce()>> {
    let key = std::sync::Arc::as_ptr(object) as *const () as usize;
    PENDING_MENUS.with(|p| {
        let mut p = p.borrow_mut();
        let mut out = Vec::new();
        let mut i = 0;
        while i < p.len() {
            if p[i].0 == key {
                out.push(p.remove(i).1);
            } else {
                i += 1;
            }
        }
        out
    })
}

/// A mod menu class's overrides of the menu's own answers (`canDragTo`, `canTakeItemForPickAll`), on its handle;
/// `super.x(..)` inside them is the answer the port hands (`super_answer`). The defaults are the base's.
pub trait MenuOverrides: Sized + 'static {
    fn can_drag_to(this: &Shared<Self>, slot: &mcrust::net::minecraft::world::inventory::slot::Slot) -> Option<bool> {
        let _ = (this, slot);
        None
    }

    fn can_take_item_for_pick_all(
        this: &Shared<Self>,
        carried: &mcrust::net::minecraft::world::item::item_stack::ItemStack,
        target: &mcrust::net::minecraft::world::inventory::slot::Slot,
    ) -> Option<bool> {
        let _ = (this, carried, target);
        None
    }
}

/// An override that reads `super.x()`: the base's answer, for `compat::super_answer` while `f` runs.
pub fn with_super_answer<T: Clone + 'static, R>(answer: T, f: impl FnOnce() -> R) -> R {
    SUPER_ANSWER.with(|s| s.borrow_mut().push(Box::new(answer)));
    struct Pop;
    impl Drop for Pop {
        fn drop(&mut self) {
            SUPER_ANSWER.with(|s| {
                s.borrow_mut().pop();
            });
        }
    }
    let _pop = Pop;
    f()
}

/// A mod menu object as the game's `ModMenu`: its handle; `init` lends it the menu and runs its constructor's
/// body.
pub struct MenuAdapter<M: 'static> {
    object: Shared<M>,
    inits: Vec<Box<dyn FnOnce()>>,
    owner: std::thread::ThreadId,
}

// SAFETY: every use is checked to the thread that made it (the render thread, where the game opens menus).
unsafe impl<M: 'static> Send for MenuAdapter<M> {}

/// `player.containerMenu = handler` (the factory's answer): the object, its constructor kept for `init`.
pub fn mod_menu<M: MenuHalf + MenuOverrides + 'static>(object: Shared<M>) -> Box<dyn mcrust::mcrust::modding::container_menus::ModMenu> {
    let inits = take_menu_inits(&object);
    Box::new(MenuAdapter { object, inits, owner: std::thread::current().id() })
}

impl<M: MenuHalf + 'static> MenuAdapter<M> {
    pub fn object(&self) -> &Shared<M> {
        &self.object
    }
}

/// The object's code with the live menu lent to it (swapped into `base`, back after - also on unwind).
pub fn lend_menu<M: MenuHalf, R>(
    object: &Shared<M>,
    menu: &mut mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu,
    f: impl FnOnce() -> R,
) -> R {
    struct Back<'a, M: MenuHalf>(&'a Shared<M>, &'a mut mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu);
    impl<M: MenuHalf> Drop for Back<'_, M> {
        fn drop(&mut self) {
            let lent = &mut *self.1;
            write(self.0, |t| std::mem::swap(t.menu_half(), lent));
        }
    }
    write(object, |t| std::mem::swap(t.menu_half(), menu));
    let _back = Back(object, menu);
    f()
}

impl<M: MenuHalf + MenuOverrides + 'static> mcrust::mcrust::modding::container_menus::ModMenu for MenuAdapter<M> {
    fn init(&mut self, menu: &mut mcrust::net::minecraft::world::inventory::abstract_container_menu::AbstractContainerMenu) {
        assert!(std::thread::current().id() == self.owner, "a mod menu was reached off the render thread");
        let inits = std::mem::take(&mut self.inits);
        lend_menu(&self.object, menu, || {
            for init in inits {
                init();
            }
        });
    }

    fn can_drag_to(&mut self, slot: &mcrust::net::minecraft::world::inventory::slot::Slot, super_answer: bool) -> bool {
        assert!(std::thread::current().id() == self.owner, "a mod menu was reached off the render thread");
        with_super_answer(super_answer, || M::can_drag_to(&self.object, slot)).unwrap_or(super_answer)
    }

    fn can_take_item_for_pick_all(
        &mut self,
        carried: &mcrust::net::minecraft::world::item::item_stack::ItemStack,
        target: &mcrust::net::minecraft::world::inventory::slot::Slot,
        super_answer: bool,
    ) -> bool {
        assert!(std::thread::current().id() == self.owner, "a mod menu was reached off the render thread");
        with_super_answer(super_answer, || M::can_take_item_for_pick_all(&self.object, carried, target))
            .unwrap_or(super_answer)
    }
}

/// A mod class that extends the game's `AbstractContainerScreen`: its screen half (`base`).
pub trait ContainerHalf {
    fn container_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen;
}

/// A mod container screen object as the game's `ModContainerScreen`: its handle; each of its methods runs with
/// the game's screen lent to it.
pub struct ContainerScreenAdapter<C: 'static> {
    object: Shared<C>,
    owner: std::thread::ThreadId,
}

// SAFETY: every use is checked to the thread that made it.
unsafe impl<C: 'static> Send for ContainerScreenAdapter<C> {}

impl<C: ContainerHalf + 'static> ContainerScreenAdapter<C> {
    pub fn new(object: Shared<C>) -> ContainerScreenAdapter<C> {
        ContainerScreenAdapter { object, owner: std::thread::current().id() }
    }

    pub fn object(&self) -> &Shared<C> {
        &self.object
    }

    fn check(&self) {
        assert!(std::thread::current().id() == self.owner, "a mod container screen was reached off the render thread");
    }

    /// One of the object's methods with the game's screen lent to it (swapped into `base`, back after).
    pub fn lend<R>(
        &self,
        screen: &mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        struct Back<'a, C: ContainerHalf>(&'a Shared<C>, &'a mut mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen);
        impl<C: ContainerHalf> Drop for Back<'_, C> {
            fn drop(&mut self) {
                let lent = &mut *self.1;
                write(self.0, |t| std::mem::swap(t.container_half(), lent));
            }
        }
        write(&self.object, |t| std::mem::swap(t.container_half(), screen));
        let _back = Back(&self.object, screen);
        f(&self.object)
    }

    /// A method that only READS the game's screen (the extract passes): its geometry brought into the object.
    pub fn lend_ref<R>(
        &self,
        screen: &mcrust::net::minecraft::client::gui::screens::inventory::abstract_container_screen::AbstractContainerScreen,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        write(&self.object, |t| {
            let half = t.container_half();
            half.left_pos = screen.left_pos;
            half.top_pos = screen.top_pos;
            half.image_width = screen.image_width;
            half.image_height = screen.image_height;
            half.title_label_x = screen.title_label_x;
            half.title_label_y = screen.title_label_y;
            half.inventory_label_x = screen.inventory_label_x;
            half.inventory_label_y = screen.inventory_label_y;
            half.hovered_slot = screen.hovered_slot;
            half.screen.width = screen.screen.width;
            half.screen.height = screen.screen.height;
            half.screen.title = screen.screen.title.clone();
        });
        f(&self.object)
    }
}

thread_local! {
    static SUPER_CALLS: std::cell::RefCell<Vec<*mut (dyn FnMut() + 'static)>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// The base class's body of an override the port hands as a closure (`super_extract_background`), for
/// `super.extractBackground(..)` inside the override (`call_super`).
pub fn with_super<R>(sup: &mut dyn FnMut(), f: impl FnOnce() -> R) -> R {
    let p: *mut (dyn FnMut() + '_) = sup;
    // SAFETY: the pointer is popped before `with_super` returns (also on unwind), so it never outlives `sup`.
    let p: *mut (dyn FnMut() + 'static) = unsafe { std::mem::transmute(p) };
    SUPER_CALLS.with(|s| s.borrow_mut().push(p));
    struct Pop;
    impl Drop for Pop {
        fn drop(&mut self) {
            SUPER_CALLS.with(|s| {
                s.borrow_mut().pop();
            });
        }
    }
    let _pop = Pop;
    f()
}

/// `super.extractBackground(..)` (& co) inside an override the port hands its base body.
pub fn call_super() {
    let p = SUPER_CALLS.with(|s| s.borrow().last().copied());
    if let Some(p) = p {
        // SAFETY: `with_super` keeps the closure alive while the pointer is on the stack.
        unsafe { (*p)() }
    }
}

/// What a `MenuScreens.fromPacket` interception decided (`container_screens::register`'s factory): the screen
/// to open and the menu object over the server's menu.
#[derive(Default)]
pub struct ContainerResult {
    screen: Option<Box<dyn mcrust::mcrust::modding::container_screens::ModContainerScreen>>,
    plain: Option<Box<dyn mcrust::mcrust::modding::screens::ModScreen>>,
    menu: Option<Box<dyn mcrust::mcrust::modding::container_menus::ModMenu>>,
}

impl ContainerResult {
    pub fn container_screen(&mut self, screen: Box<dyn mcrust::mcrust::modding::container_screens::ModContainerScreen>) {
        self.screen = Some(screen);
    }

    pub fn mod_screen(&mut self, screen: Box<dyn mcrust::mcrust::modding::screens::ModScreen>) {
        self.plain = Some(screen);
    }

    pub fn menu(&mut self, menu: Box<dyn mcrust::mcrust::modding::container_menus::ModMenu>) {
        self.menu = Some(menu);
    }

    /// `ci.cancel()`: the answer.
    pub fn finish(&mut self) -> Option<mcrust::mcrust::modding::container_screens::ModContainer> {
        use mcrust::mcrust::modding::container_screens::ModContainer;
        if let Some(plain) = self.plain.take() {
            return Some(match self.menu.take() {
                Some(m) => ModContainer::mod_screen_and_menu(plain, m),
                None => ModContainer::mod_screen(plain),
            });
        }
        match (self.screen.take(), self.menu.take()) {
            (Some(s), Some(m)) => Some(ModContainer::screen_and_menu(s, m)),
            (Some(s), None) => Some(ModContainer::screen(s)),
            (None, Some(m)) => Some(ModContainer::menu(m)),
            (None, None) => None,
        }
    }
}

/// A mod class that extends the game's `Screen`: its `Screen` half (`base`, or the base of its base). The game
/// keeps the live half - the widget tree, `width`, `height`, `title` - and LENDS it to the object for each of
/// its methods (`ScreenAdapter::lend`), so the object's code reaches it through `base` exactly as Java's
/// `this.width` / `this.addRenderableWidget(..)` reach the superclass's fields.
pub trait ScreenHalf {
    fn screen_half(&mut self) -> &mut mcrust::net::minecraft::client::gui::screens::screen::Screen;

    /// The `Screen` half and the object's widget references' cache (`WidgetCache`), together.
    fn screen_parts(&mut self) -> (&mut mcrust::net::minecraft::client::gui::screens::screen::Screen, Option<&mut WidgetCache>) {
        (self.screen_half(), None)
    }
}

type ScreenWidget = mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget;
type ScreenTree = mcrust::net::minecraft::client::gui::screens::screen::Screen;

/// Java's reference to a widget of a mod screen. Before the widget is added to the screen it is a value the
/// object holds; once added (`addRenderableWidget`, a layout's `addChild`) the SCREEN holds it, by index, and
/// every access through this reference reaches the screen's widget - the one the player sees and clicks.
#[derive(Clone, Debug)]
pub enum WidgetRef {
    Value(Box<ScreenWidget>),
    Screen(usize),
}

impl WidgetRef {
    pub fn value(widget: ScreenWidget) -> WidgetRef {
        WidgetRef::Value(Box::new(widget))
    }
}

/// The screen's widgets as the object last saw them, for code that runs while the game has NOT lent the object
/// its `Screen` half (a button's action runs inside the game's own widget pass): it reads that copy - brought up
/// to date at the end of every call the game makes on the object, including the click that runs the action -
/// and what it changes is written into the screen's widget at the start of the next call (every tick).
#[derive(Default, Debug)]
pub struct WidgetCache {
    lent: bool,
    shadow: std::collections::HashMap<usize, ScreenWidget>,
    dirty: std::collections::HashSet<usize>,
}

impl WidgetCache {
    /// The game lent the object its `Screen` half: the changes made since go into the screen's widgets.
    pub fn begin(&mut self, screen: &mut ScreenTree) {
        for i in std::mem::take(&mut self.dirty) {
            if let Some(w) = self.shadow.get(&i) {
                if i < screen.children().len() {
                    *screen.widget_mut(i) = w.clone();
                }
            }
        }
        self.lent = true;
    }

    /// The call is over: the copies are brought up to date.
    pub fn end(&mut self, screen: &ScreenTree) {
        self.lent = false;
        for (i, w) in self.shadow.iter_mut() {
            if *i < screen.children().len() {
                *w = screen.widget(*i).clone();
            }
        }
    }

    /// One access through a widget reference (`button.active = false`, `field.getValue()`).
    pub fn with<R>(&mut self, screen: &mut ScreenTree, r: &mut WidgetRef, f: impl FnOnce(&mut ScreenWidget) -> R) -> R {
        match r {
            WidgetRef::Value(w) => f(w),
            WidgetRef::Screen(i) => {
                let i = *i;
                if self.lent && i < screen.children().len() {
                    let out = f(screen.widget_mut(i));
                    self.shadow.insert(i, screen.widget(i).clone());
                    out
                } else {
                    let w = self.shadow.get_mut(&i).expect("a mod screen's widget reached before the screen was shown");
                    self.dirty.insert(i);
                    f(w)
                }
            }
        }
    }

    /// `addRenderableWidget(w)` / `addWidget(w)`: the screen takes the widget; the reference now reaches it there.
    pub fn add(&mut self, screen: &mut ScreenTree, r: &mut WidgetRef, renderable: bool) -> usize {
        match r {
            WidgetRef::Screen(i) => {
                if renderable && self.lent {
                    screen.mark_renderable(*i);
                }
                *i
            }
            WidgetRef::Value(w) => {
                let w = (**w).clone();
                let i = if renderable { screen.add_renderable_widget(w) } else { screen.add_widget(w) };
                self.shadow.insert(i, screen.widget(i).clone());
                *r = WidgetRef::Screen(i);
                i
            }
        }
    }

    /// `layout.addChild(w)`: the widget joins the screen's list (a layout holds it by index), and its size.
    pub fn add_to_layout(&mut self, screen: &mut ScreenTree, r: &mut WidgetRef) -> (usize, i32, i32) {
        let i = self.add(screen, r, false);
        let w = screen.widget(i);
        (i, w.get_width(), w.get_height())
    }
}

/// A mod's screen object as the game's `ModScreen`: the object's handle (the same object the mod's other
/// references hold). The game calls it on the render thread only - checked on every call - which is why it
/// may be `Send` although the mod's objects are not.
/// The game's `Screen` half lent to a mod screen object for `f` - one of its methods the game calls, or a button
/// action, which the game runs with the whole screen in hand: swapped into the object's `base` for the call and
/// back after (also on unwind), the object's widget references reaching the live widgets meanwhile; the object
/// keeps the last size and title for code that runs between calls.
pub fn lend_screen<C: ScreenHalf, R>(
    object: &Shared<C>,
    screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
    f: impl FnOnce() -> R,
) -> R {
    struct Back<'a, C: ScreenHalf>(&'a Shared<C>, &'a mut mcrust::net::minecraft::client::gui::screens::screen::Screen);
    impl<C: ScreenHalf> Drop for Back<'_, C> {
        fn drop(&mut self) {
            let lent = &mut *self.1;
            write(self.0, |t| {
                if let (screen, Some(cache)) = t.screen_parts() {
                    cache.end(screen);
                }
                std::mem::swap(t.screen_half(), lent);
                let half = t.screen_half();
                half.width = lent.width;
                half.height = lent.height;
                half.title = lent.title.clone();
            });
        }
    }
    write(object, |t| {
        std::mem::swap(t.screen_half(), screen);
        if let (screen, Some(cache)) = t.screen_parts() {
            cache.begin(screen);
        }
    });
    let _back = Back(object, screen);
    f()
}

pub struct ScreenAdapter<C: 'static> {
    object: Shared<C>,
    owner: std::thread::ThreadId,
}

// SAFETY: every use goes through `check`, which pins the adapter to the thread that made it (the render
// thread, the only one the game calls a screen on).
unsafe impl<C: 'static> Send for ScreenAdapter<C> {}

impl<C: ScreenHalf + 'static> ScreenAdapter<C> {
    pub fn new(object: Shared<C>) -> ScreenAdapter<C> {
        ScreenAdapter { object, owner: std::thread::current().id() }
    }

    /// The mod object (`minecraft.screen instanceof C c` binds this).
    pub fn object(&self) -> &Shared<C> {
        self.check();
        &self.object
    }

    fn check(&self) {
        assert!(std::thread::current().id() == self.owner, "a mod screen was reached off the render thread");
    }

    /// One of the object's methods with the game's `Screen` half lent to it (swapped into its `base` for the
    /// call, swapped back after - also on unwind); the object keeps the last size and title for code that runs
    /// between calls (a button's action).
    pub fn lend<R>(
        &self,
        screen: &mut mcrust::net::minecraft::client::gui::screens::screen::Screen,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        lend_screen(&self.object, screen, || f(&self.object))
    }

    /// A method that only READS the game's half (`extractRenderState`): the size is brought up to date first.
    pub fn lend_ref<R>(
        &self,
        screen: &mcrust::net::minecraft::client::gui::screens::screen::Screen,
        f: impl FnOnce(&Shared<C>) -> R,
    ) -> R {
        self.check();
        write(&self.object, |t| {
            let half = t.screen_half();
            half.width = screen.width;
            half.height = screen.height;
            if let (_, Some(cache)) = t.screen_parts() {
                cache.end(screen);
            }
        });
        f(&self.object)
    }

    /// A method that does not touch the `Screen` half (`removed`, `onClose`, `shouldCloseOnEsc`).
    pub fn call<R>(&self, f: impl FnOnce(&Shared<C>) -> R) -> R {
        self.check();
        f(&self.object)
    }

    /// `getTitle()` - the title the constructor gave `super(title)`.
    pub fn title(&self) -> String {
        self.check();
        write(&self.object, |t| t.screen_half().title.clone())
    }
}

pub fn shared<T>(value: T) -> Shared<T> {
    std::sync::Arc::new(parking_lot::ReentrantMutex::new(std::cell::RefCell::new(value)))
}


/// A key category by id: vanilla's, or the mod's own (registered on first use,
/// `KeyMapping.Category.register`).
pub fn category(id: &str) -> mcrust::net::minecraft::client::key_mapping::Category {
    use mcrust::net::minecraft::client::key_mapping::Category;
    let Some(ident) = mcrust::net::minecraft::resources::identifier::Identifier::try_parse(id) else {
        return Category::Misc;
    };
    Category::by_id(&ident).unwrap_or_else(|| {
        crate::prelude::key_binding_helper::register_category(id).unwrap_or(Category::Misc)
    })
}

// =====================================================================================================================
// YACL (batch Y2): the pieces of the game's GUI the YACL screen draws and dispatches ITSELF.
//
// YACL's screen keeps every widget it shows as its own object (`Arc<..>`, state in cells) and runs Java's
// `ContainerEventHandler` / `AbstractSelectionList` bodies over them from its `ModScreen` methods, which the game lends
// the `Screen` and the frame's draw context. Nothing YACL builds is registered in the game's widget list (except the
// cursor sink below), so a listener that reaches a widget while another one is mid-event never meets a borrow the game
// holds (PATTERNS_Y2.md P1-P3).
// =====================================================================================================================

/// `minecraft.execute(task)` for a task that holds the mod's objects (not `Send`): Java's `BlockableEventLoop.execute`
/// - inline on the render thread outside a running task, else queued for it (`game::execute`).
pub fn execute_local(task: impl FnOnce() + 'static) {
    let cell: RenderThread<std::cell::RefCell<Option<Box<dyn FnOnce()>>>> =
        RenderThread::new(std::cell::RefCell::new(Some(Box::new(task))));
    mcrust::mcrust::modding::game::execute(move || {
        let task = cell.get().borrow_mut().take();
        if let Some(task) = task {
            task();
        }
    });
}

/// A `CompletableFuture<T>` over the mod's own objects, which are not `Send` (the port's `CompletableFuture` needs
/// `Send` values). It is completed and continued on the render thread.
pub struct LocalFuture<T: 'static> {
    state: std::rc::Rc<std::cell::RefCell<LocalFutureState<T>>>,
}

struct LocalFutureState<T> {
    result: Option<Result<T, String>>,
    callbacks: Vec<Box<dyn FnOnce(&Result<T, String>)>>,
}

impl<T: 'static> Clone for LocalFuture<T> {
    fn clone(&self) -> Self {
        LocalFuture { state: self.state.clone() }
    }
}

impl<T: 'static> PartialEq for LocalFuture<T> {
    /// `CompletableFuture.equals` is identity.
    fn eq(&self, other: &Self) -> bool {
        std::rc::Rc::ptr_eq(&self.state, &other.state)
    }
}

impl<T: Clone + 'static> LocalFuture<T> {
    /// `new CompletableFuture<>()`
    pub fn new() -> LocalFuture<T> {
        LocalFuture { state: std::rc::Rc::new(std::cell::RefCell::new(LocalFutureState { result: None, callbacks: Vec::new() })) }
    }

    /// `CompletableFuture.completedFuture(value)`
    pub fn completed_future(value: T) -> LocalFuture<T> {
        let future = LocalFuture::new();
        future.complete(value);
        future
    }

    fn finish(&self, result: Result<T, String>) -> bool {
        let callbacks = {
            let mut state = self.state.borrow_mut();
            if state.result.is_some() {
                return false;
            }
            state.result = Some(result);
            std::mem::take(&mut state.callbacks)
        };
        let result = self.state.borrow().result.clone().expect("completed above");
        for callback in callbacks {
            callback(&result);
        }
        true
    }

    /// `complete(value)`
    pub fn complete(&self, value: T) -> bool {
        self.finish(Ok(value))
    }

    /// `completeExceptionally(ex)`
    pub fn complete_exceptionally(&self, error: String) -> bool {
        self.finish(Err(error))
    }

    /// `isDone()`
    pub fn is_done(&self) -> bool {
        self.state.borrow().result.is_some()
    }

    /// `getNow(valueIfAbsent)` (an exceptional completion is the `CompletionException` panic)
    pub fn get_now(&self, value_if_absent: T) -> T {
        match self.state.borrow().result.clone() {
            None => value_if_absent,
            Some(Ok(v)) => v,
            Some(Err(e)) => panic!("CompletionException: {e}"),
        }
    }

    /// `join()` of a completed future (the render thread cannot wait for itself: an incomplete one panics)
    pub fn join(&self) -> T {
        match self.state.borrow().result.clone() {
            None => panic!("LocalFuture.join() on the render thread before the future completed"),
            Some(Ok(v)) => v,
            Some(Err(e)) => panic!("CompletionException: {e}"),
        }
    }

    fn on_complete(&self, callback: impl FnOnce(&Result<T, String>) + 'static) {
        let done = self.state.borrow().result.clone();
        match done {
            Some(result) => callback(&result),
            None => self.state.borrow_mut().callbacks.push(Box::new(callback)),
        }
    }

    /// `thenApply(fn)`: on the thread that completes this one (the render thread).
    pub fn then_apply<U: Clone + 'static>(&self, f: impl FnOnce(T) -> U + 'static) -> LocalFuture<U> {
        let next = LocalFuture::<U>::new();
        let target = next.clone();
        self.on_complete(move |result| match result.clone() {
            Ok(v) => {
                target.complete(f(v));
            }
            Err(e) => {
                target.complete_exceptionally(e);
            }
        });
        next
    }

    /// `thenAccept(action)`
    pub fn then_accept(&self, f: impl FnOnce(T) + 'static) {
        self.on_complete(move |result| {
            if let Ok(v) = result.clone() {
                f(v);
            }
        });
    }
}

/// Java's `Screen` reference handed to `Minecraft.setScreen(..)` by a mod (`GuiUtils.setScreen`): the port keeps the
/// screen stack in the client loop, so the reference names WHICH screen - a new mod screen (with or without the screen
/// that is up as its parent), the parent the open screen holds, or `null`.
pub enum ScreenRef {
    /// `null`
    Null,
    /// the screen the open one holds as its parent (`setScreen(this.parent)`)
    Parent,
    /// `Minecraft.getInstance().screen` handed to a new screen's constructor as its parent
    Current,
    /// `new MyScreen(..)`; `keep_parent` when its constructor keeps the screen that is up
    Mod { screen: Box<dyn mcrust::mcrust::modding::screens::ModScreen>, keep_parent: bool },
}

impl ScreenRef {
    /// `new MyScreen(..)` with no parent.
    pub fn new(screen: impl mcrust::mcrust::modding::screens::ModScreen + 'static) -> ScreenRef {
        ScreenRef::Mod { screen: Box::new(screen), keep_parent: false }
    }

    /// `new MyScreen(minecraft.screen)`.
    pub fn new_keeping_parent(screen: impl mcrust::mcrust::modding::screens::ModScreen + 'static) -> ScreenRef {
        ScreenRef::Mod { screen: Box::new(screen), keep_parent: true }
    }

    /// A screen built as a `Box` already.
    pub fn of_box(screen: Box<dyn mcrust::mcrust::modding::screens::ModScreen>, keep_parent: bool) -> ScreenRef {
        ScreenRef::Mod { screen, keep_parent }
    }

    /// `minecraft.setScreen(this)`.
    pub fn set(self) {
        use mcrust::mcrust::modding::game;
        match self {
            ScreenRef::Null => game::set_screen_null(),
            // `setScreen(parent)` where the parent is the screen that is up: nothing changes
            ScreenRef::Current => {}
            ScreenRef::Parent => game::set_screen_to_parent(),
            ScreenRef::Mod { screen, keep_parent: true } => game::set_screen_keeping_parent(screen),
            ScreenRef::Mod { screen, keep_parent: false } => game::set_screen(screen),
        }
    }
}

/// `java.lang.Enum` for a mod enum handed to YACL as a value type (`EnumController`, `CyclingListController` over
/// `getEnumConstants()`): its constants in declaration order, `ordinal()`, `name()`, and `instanceof NameableEnum`.
pub trait JavaEnum: Clone + PartialEq + 'static {
    /// `enumClass.getEnumConstants()`
    fn enum_constants() -> Vec<Self>;
    /// `ordinal()`
    fn ordinal(&self) -> i32;
    /// `name()`
    fn name(&self) -> String;
    /// `this instanceof NameableEnum n ? n.getDisplayName() : null`
    fn nameable_display_name(&self) -> Option<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent> {
        None
    }
}

/// `Minecraft.getInstance().getSoundManager().play(SimpleSoundInstance.forUI(event, 1.0F))` from mod code.
/// rust: port gap (GAPS_Y2.md G3) - the sound engine is a local of the frame loop; a mod has no way to play a UI sound
/// outside the game's own click sweep. The request is dropped (logged once).
pub fn play_ui_sound(event: &mcrust::net::minecraft::sounds::sound_event::SoundEvent) {
    let _ = event;
    mcrust::mcrust::modding::log::warn_once(
        "yacl.play_ui_sound",
        "YACL: a UI sound was requested from mod code; the port has no mod-side sound player yet (GAPS_Y2 G3)",
    );
}

/// `AbstractWidget.playDownSound` - `UI_BUTTON_CLICK`.
pub fn play_down_sound() {
    play_ui_sound(mcrust::net::minecraft::sounds::sound_events::ui_button_click());
}

/// `Minecraft.getInstance().keyboardHandler.getClipboard()`.
pub fn get_clipboard() -> String {
    use mcrust::net::minecraft::client::gui::components::edit_box::ClipboardAccess;
    match mcrust::net::minecraft::client::minecraft::SdlClipboard::published() {
        Some(mut clipboard) => clipboard.get_clipboard(),
        None => String::new(),
    }
}

/// `Minecraft.getInstance().keyboardHandler.setClipboard(value)`.
pub fn set_clipboard(value: &str) {
    use mcrust::net::minecraft::client::gui::components::edit_box::ClipboardAccess;
    if let Some(mut clipboard) = mcrust::net::minecraft::client::minecraft::SdlClipboard::published() {
        clipboard.set_clipboard(value);
    }
}

/// `com.mojang.blaze3d.platform.cursor.CursorTypes` as a mod names one (`graphics.requestCursor(CursorTypes.IBEAM)`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CursorKind {
    Arrow,
    IBeam,
    Crosshair,
    PointingHand,
    ResizeNs,
    ResizeEw,
    ResizeAll,
    NotAllowed,
}

thread_local! {
    /// The cursor the mod screen's last extract asked for (`graphics.requestCursor(..)`), answered to the window by the
    /// cursor sink widget after the frame.
    static REQUESTED_CURSOR: std::cell::Cell<Option<CursorKind>> = const { std::cell::Cell::new(None) };
}

/// The game asks every renderable widget of the open screen for its cursor after the frame (window.rs); a mod screen
/// that draws its own widgets requests its cursor through this one, a zero-size widget that answers what the screen's
/// extract asked for (`GuiGraphicsExtractor.requestCursor`). Add it in `init` (`cursor_sink_widget()`).
pub struct CursorSink;

impl mcrust::mcrust::modding::widgets::ModWidget for CursorSink {
    fn extract_widget_render_state(
        &mut self,
        _widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        _context: &mut mcrust::mcrust::modding::widgets::WidgetRenderContext<'_>,
        _mouse_x: i32,
        _mouse_y: i32,
        _a: f32,
    ) {
    }

    fn update_widget_narration(
        &mut self,
        _widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        _output: &mut dyn mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput,
    ) {
    }

    fn is_mouse_over(
        &self,
        _widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        _mouse_x: f64,
        _mouse_y: f64,
    ) -> bool {
        false
    }

    fn handle_cursor(
        &self,
        _widget: &mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget,
        cursors: &mcrust::com::mojang::blaze3d::platform::cursor::cursor_types::CursorTypes,
    ) -> Option<mcrust::com::mojang::blaze3d::platform::cursor::cursor_type::CursorType> {
        let kind = REQUESTED_CURSOR.with(|c| c.get())?;
        Some(match kind {
            CursorKind::Arrow => cursors.arrow,
            CursorKind::IBeam => cursors.ibeam,
            CursorKind::Crosshair => cursors.crosshair,
            CursorKind::PointingHand => cursors.pointing_hand,
            CursorKind::ResizeNs => cursors.resize_ns,
            CursorKind::ResizeEw => cursors.resize_ew,
            CursorKind::ResizeAll => cursors.resize_all,
            CursorKind::NotAllowed => cursors.not_allowed,
        })
    }
}

/// The cursor sink as the game's widget (add it with `screen.add_renderable_widget(..)`).
pub fn cursor_sink_widget() -> mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget {
    let mut w = mcrust::mcrust::modding::widgets::ModWidgetSlot::widget(-1, -1, 0, 0, String::new(), Box::new(CursorSink));
    w.active = false;
    w
}

/// Java's `GuiGraphicsExtractor graphics` handed to the YACL screen's widgets: the frame's extractor with the GL handle
/// and the `Minecraft` this port's draw functions take, and Java's method names.
pub struct Graphics<'a> {
    pub gl: &'a mcrust::lwjgl::opengl::Gl,
    pub mc: &'a mcrust::net::minecraft::client::minecraft::Minecraft,
    pub extractor: mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor,
    poses: std::cell::RefCell<Vec<mcrust::org::joml::matrix3x2f::Matrix3x2f>>,
    /// `GuiGraphicsExtractor.deferredTooltip` - the first tooltip a widget parked this frame (`setTooltipForNextFrame`),
    /// drawn over everything by `extract_deferred_tooltip` (`extractDeferredElements`).
    deferred_tooltip: std::cell::RefCell<Option<DeferredTooltip>>,
}

/// A tooltip parked for the end of the frame: its lines, where it was asked for, and its positioner.
pub struct DeferredTooltip {
    lines: Vec<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>,
    x: i32,
    y: i32,
    positioner: Box<dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_positioner::ClientTooltipPositioner>,
}

impl<'a> Graphics<'a> {
    pub fn new(gl: &'a mcrust::lwjgl::opengl::Gl, mc: &'a mcrust::net::minecraft::client::minecraft::Minecraft) -> Graphics<'a> {
        REQUESTED_CURSOR.with(|c| c.set(None));
        Graphics {
            gl,
            mc,
            extractor: mcrust::net::minecraft::client::gui::gui_graphics_extractor::GuiGraphicsExtractor::new(),
            poses: std::cell::RefCell::new(Vec::new()),
            deferred_tooltip: std::cell::RefCell::new(None),
        }
    }

    /// `setTooltipForNextFrame(font, lines, positioner, x, y, false)` - the first tooltip of the frame wins
    /// (`setTooltipForNextFrameInternal:1202-1206`).
    pub fn set_tooltip_for_next_frame(
        &self,
        lines: Vec<mcrust::net::minecraft::network::chat::formatted_text::FormattedText>,
        positioner: Box<dyn mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_positioner::ClientTooltipPositioner>,
        x: i32,
        y: i32,
    ) {
        let mut slot = self.deferred_tooltip.borrow_mut();
        if !lines.is_empty() && slot.is_none() {
            *slot = Some(DeferredTooltip { lines, x, y, positioner });
        }
    }

    /// `extractDeferredElements` - the parked tooltip, on the next stratum over everything drawn so far
    /// (`GuiGraphicsExtractor.tooltip:1211-1254`).
    pub fn extract_deferred_tooltip(&self) {
        use mcrust::net::minecraft::client::gui::gui_graphics_extractor;
        use mcrust::net::minecraft::client::gui::screens::inventory::tooltip::client_tooltip_component::TooltipGraphics;
        let Some(tooltip) = self.deferred_tooltip.borrow_mut().take() else {
            return;
        };
        let components = gui_graphics_extractor::text_components(tooltip.lines);
        let mut gui_items = Vec::new();
        let mut tooltip_graphics = TooltipGraphics {
            gl: self.gl,
            gfx: self.mc,
            gui_items: &mut gui_items,
            resolver: None,
            level: None,
            owner: None,
            cooldowns: None,
            cooldown_partial_tick: 0.0,
        };
        gui_graphics_extractor::next_stratum();
        gui_graphics_extractor::tooltip(
            &mut tooltip_graphics,
            &*self.mc.font,
            &components,
            tooltip.x,
            tooltip.y,
            &*tooltip.positioner,
            None,
        );
    }

    /// `graphics.guiWidth()`
    pub fn gui_width(&self) -> i32 {
        self.mc.window.get_gui_scaled_width()
    }

    /// `graphics.guiHeight()`
    pub fn gui_height(&self) -> i32 {
        self.mc.window.get_gui_scaled_height()
    }

    /// The same frame as a `WidgetRenderContext`, for drawing the game's own widgets (`ButtonLook`).
    pub fn widget_context(&self) -> mcrust::mcrust::modding::widgets::WidgetRenderContext<'a> {
        mcrust::mcrust::modding::widgets::WidgetRenderContext::new(self.gl, self.mc)
    }

    /// `graphics.fill(x0, y0, x1, y1, color)`
    pub fn fill(&self, x0: i32, y0: i32, x1: i32, y1: i32, color: i32) {
        self.extractor.fill(self.gl, x0, y0, x1, y1, color);
    }

    /// `graphics.fillGradient(x0, y0, x1, y1, col1, col2)`
    pub fn fill_gradient(&self, x0: i32, y0: i32, x1: i32, y1: i32, col1: i32, col2: i32) {
        self.extractor.fill_gradient(self.gl, x0, y0, x1, y1, col1, col2);
    }

    /// `graphics.outline(x, y, width, height, color)`
    pub fn outline(&self, x: i32, y: i32, width: i32, height: i32, color: i32) {
        self.fill(x, y, x + width, y + 1, color);
        self.fill(x, y + height - 1, x + width, y + height, color);
        self.fill(x, y + 1, x + 1, y + height - 1, color);
        self.fill(x + width - 1, y + 1, x + width, y + height - 1, color);
    }

    /// `graphics.text(font, text, x, y, color, dropShadow)`
    pub fn text(&self, text: &mcrust::net::minecraft::network::chat::formatted_text::FormattedText, x: i32, y: i32, color: i32, drop_shadow: bool) {
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::draw_formatted_text_with_drop_shadow(self.gl, self.mc, text, x, y, color, drop_shadow);
    }

    /// `graphics.text(font, component, x, y, color, dropShadow)`
    pub fn text_component(&self, text: &mcrust::net::minecraft::network::chat::mutable_component::MutableComponent, x: i32, y: i32, color: i32, drop_shadow: bool) {
        let text = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(text.clone());
        self.text(&text, x, y, color, drop_shadow);
    }

    /// `graphics.text(font, string, x, y, color, dropShadow)`
    pub fn text_str(&self, text: &str, x: i32, y: i32, color: i32, drop_shadow: bool) {
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::text_string_with_drop_shadow(self.gl, self.mc, Some(text), x, y, color, drop_shadow);
    }

    /// `graphics.centeredText(font, text, x, y, color)`
    pub fn centered_text(&self, text: &mcrust::net::minecraft::network::chat::formatted_text::FormattedText, x: i32, y: i32, color: i32) {
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::centered_text(self.gl, self.mc, text, x, y, color);
    }

    /// `graphics.blitSprite(RenderPipelines.GUI_TEXTURED, sprite, x, y, width, height)`
    pub fn blit_sprite(&self, sprite: &mcrust::net::minecraft::resources::identifier::Identifier, x: i32, y: i32, width: i32, height: i32) {
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::blit_sprite_identifier(
            self.gl,
            self.mc,
            mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(),
            sprite,
            x,
            y,
            width,
            height,
        );
    }

    /// `graphics.blitSprite(RenderPipelines.GUI_TEXTURED, sprite, x, y, width, height, color)`
    pub fn blit_sprite_with_color(&self, sprite: &mcrust::net::minecraft::resources::identifier::Identifier, x: i32, y: i32, width: i32, height: i32, color: i32) {
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::blit_sprite_identifier_with_color(
            self.gl,
            self.mc,
            mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(),
            sprite,
            x,
            y,
            width,
            height,
            color,
        );
    }

    /// `graphics.blit(RenderPipelines.GUI_TEXTURED, texture, x, y, u, v, width, height, textureWidth, textureHeight)`
    #[allow(clippy::too_many_arguments)]
    pub fn blit(
        &self,
        texture: &mcrust::net::minecraft::resources::identifier::Identifier,
        x: i32,
        y: i32,
        u: f32,
        v: f32,
        width: i32,
        height: i32,
        texture_width: i32,
        texture_height: i32,
    ) {
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::blit_identifier(
            self.gl,
            self.mc,
            mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(),
            texture,
            x,
            y,
            u,
            v,
            width,
            height,
            texture_width,
            texture_height,
        );
    }

    /// `graphics.blit(RenderPipelines.GUI_TEXTURED, texture, x, y, u, v, width, height, textureWidth, textureHeight, color)`
    #[allow(clippy::too_many_arguments)]
    pub fn blit_with_color(
        &self,
        texture: &mcrust::net::minecraft::resources::identifier::Identifier,
        x: i32,
        y: i32,
        u: f32,
        v: f32,
        width: i32,
        height: i32,
        texture_width: i32,
        texture_height: i32,
        color: i32,
    ) {
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::blit_identifier_with_color(
            self.gl,
            self.mc,
            mcrust::net::minecraft::client::renderer::render_pipelines::gui_textured(),
            texture,
            x,
            y,
            u,
            v,
            width,
            height,
            texture_width,
            texture_height,
            color,
        );
    }

    /// `graphics.enableScissor(x0, y0, x1, y1)`
    pub fn enable_scissor(&self, x0: i32, y0: i32, x1: i32, y1: i32) {
        self.extractor.enable_scissor(x0, y0, x1, y1);
    }

    /// `graphics.disableScissor()`
    pub fn disable_scissor(&self) {
        self.extractor.disable_scissor();
    }

    /// `graphics.pose().pushMatrix()`
    pub fn push_pose(&self) {
        let saved = mcrust::net::minecraft::client::gui::gui_graphics_extractor::push_pose();
        self.poses.borrow_mut().push(saved);
    }

    /// `graphics.pose().popMatrix()`
    pub fn pop_pose(&self) {
        let saved = self.poses.borrow_mut().pop().expect("graphics.pose().popMatrix() with no matching pushMatrix()");
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::pop_pose(saved);
    }

    /// `graphics.pose().translate(x, y)`
    pub fn translate(&self, x: f32, y: f32) {
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::pose_translate(x, y);
    }

    /// `graphics.pose().scale(x, y)`
    pub fn scale(&self, x: f32, y: f32) {
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::pose_scale(x, y);
    }

    /// `graphics.pose().rotate(angle)` (radians)
    pub fn rotate(&self, angle: f32) {
        mcrust::net::minecraft::client::gui::gui_graphics_extractor::pose_rotate(angle);
    }

    /// `graphics.item(stack, x, y)`
    pub fn item(&self, stack: &mcrust::net::minecraft::world::item::item_stack::ItemStack, x: i32, y: i32) {
        self.extractor.item(stack, x, y);
    }

    /// `graphics.requestCursor(cursor)`
    pub fn request_cursor(&self, cursor: CursorKind) {
        REQUESTED_CURSOR.with(|c| c.set(Some(cursor)));
    }
}

impl Drop for Graphics<'_> {
    fn drop(&mut self) {
        // a pose a widget pushed and forgot (an unwind) is popped for it
        for saved in self.poses.borrow_mut().drain(..).rev() {
            mcrust::net::minecraft::client::gui::gui_graphics_extractor::pop_pose(saved);
        }
    }
}

/// Java's `GuiEventListener` + `Renderable` for an object the mod dispatches itself (the YACL screen's buttons, edit box,
/// list and YACL's own widgets). Every default body is `GuiEventListener.java`'s. Methods take `&self`: the object keeps
/// its state in cells, so a listener it fires can reach it again. `DynAny`: `child instanceof SearchFieldWidget`.
pub trait GuiEventListener: DynAny {
    /// `Renderable.extractRenderState(graphics, mouseX, mouseY, a)`
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32);

    /// `mouseClicked(event, doubleClick)` - `false`.
    fn mouse_clicked(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent, double_click: bool) -> bool {
        let _ = (event, double_click);
        false
    }

    /// `mouseReleased(event)` - `false`.
    fn mouse_released(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent) -> bool {
        let _ = event;
        false
    }

    /// `mouseDragged(event, dx, dy)` - `false`.
    fn mouse_dragged(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent, dx: f64, dy: f64) -> bool {
        let _ = (event, dx, dy);
        false
    }

    /// `mouseScrolled(x, y, scrollX, scrollY)` - `false`.
    fn mouse_scrolled(&self, x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        let _ = (x, y, scroll_x, scroll_y);
        false
    }

    /// `mouseMoved(x, y)` - empty.
    fn mouse_moved(&self, x: f64, y: f64) {
        let _ = (x, y);
    }

    /// `keyPressed(event)` - `false`.
    fn key_pressed(&self, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        let _ = event;
        false
    }

    /// `charTyped(event)` - `false`.
    fn char_typed(&self, event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent) -> bool {
        let _ = event;
        false
    }

    /// `isMouseOver(x, y)` - `false`.
    fn is_mouse_over(&self, x: f64, y: f64) -> bool {
        let _ = (x, y);
        false
    }

    /// `setFocused(boolean)`
    fn set_focused(&self, focused: bool);

    /// `isFocused()`
    fn is_focused(&self) -> bool;

    /// `shouldTakeFocusAfterInteraction()` - `true`.
    fn should_take_focus_after_interaction(&self) -> bool {
        true
    }
}

/// `AbstractContainerEventHandler`'s two fields (`focused`, `isDragging`) and `ContainerEventHandler`'s default bodies,
/// over the children the owner hands in (its `children()`). Identity is the `Arc`'s.
#[derive(Default)]
pub struct ContainerEventHandler {
    focused: std::cell::RefCell<Option<std::sync::Arc<dyn GuiEventListener>>>,
    dragging: std::cell::Cell<bool>,
    /// The owner's `setFocused(GuiEventListener)` is a plain field write (a class that implements
    /// `ContainerEventHandler` itself, like YACL's `ListEntryWidget`) rather than `AbstractContainerEventHandler`'s.
    plain: bool,
}

impl ContainerEventHandler {
    pub fn new() -> ContainerEventHandler {
        ContainerEventHandler::default()
    }

    /// For an owner whose `setFocused(GuiEventListener)` is `this.focused = focused` (no `setFocused(boolean)` calls on
    /// the old and new child).
    pub fn new_plain() -> ContainerEventHandler {
        ContainerEventHandler { plain: true, ..ContainerEventHandler::default() }
    }

    /// `this.focused = focused`
    pub fn set_focused_field(&self, focused: Option<std::sync::Arc<dyn GuiEventListener>>) {
        *self.focused.borrow_mut() = focused;
    }

    /// `getFocused()`
    pub fn get_focused(&self) -> Option<std::sync::Arc<dyn GuiEventListener>> {
        self.focused.borrow().clone()
    }

    /// `AbstractContainerEventHandler.setFocused(GuiEventListener)` (or the plain field write, `new_plain`)
    pub fn set_focused(&self, focused: Option<std::sync::Arc<dyn GuiEventListener>>) {
        if self.plain {
            self.set_focused_field(focused);
            return;
        }
        let current = self.focused.borrow().clone();
        let same = match (&current, &focused) {
            (Some(a), Some(b)) => std::sync::Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };
        if !same {
            if let Some(current) = current {
                current.set_focused(false);
            }
            if let Some(focused) = &focused {
                focused.set_focused(true);
            }
            *self.focused.borrow_mut() = focused;
        }
    }

    /// `isDragging()`
    pub fn is_dragging(&self) -> bool {
        self.dragging.get()
    }

    /// `setDragging(boolean)`
    pub fn set_dragging(&self, dragging: bool) {
        self.dragging.set(dragging);
    }

    /// `getChildAt(x, y)`
    pub fn get_child_at(children: &[std::sync::Arc<dyn GuiEventListener>], x: f64, y: f64) -> Option<std::sync::Arc<dyn GuiEventListener>> {
        children.iter().find(|child| child.is_mouse_over(x, y)).cloned()
    }

    /// `ContainerEventHandler.mouseClicked:39-55`
    pub fn mouse_clicked(
        &self,
        children: &[std::sync::Arc<dyn GuiEventListener>],
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
        double_click: bool,
    ) -> bool {
        let Some(widget) = ContainerEventHandler::get_child_at(children, event.x(), event.y()) else {
            return false;
        };
        if widget.mouse_clicked(event, double_click) && widget.should_take_focus_after_interaction() {
            self.set_focused(Some(widget));
            if event.button() == 1 {
                self.set_dragging(true);
            }
        }
        true
    }

    /// `ContainerEventHandler.mouseClicked:39-55` for an owner that overrides `setFocused(GuiEventListener)`: the
    /// owner's `set_focused` is called where the default body calls `this.setFocused(widget)`.
    pub fn mouse_clicked_with(
        &self,
        children: &[std::sync::Arc<dyn GuiEventListener>],
        event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent,
        double_click: bool,
        set_focused: &dyn Fn(Option<std::sync::Arc<dyn GuiEventListener>>),
    ) -> bool {
        let Some(widget) = ContainerEventHandler::get_child_at(children, event.x(), event.y()) else {
            return false;
        };
        if widget.mouse_clicked(event, double_click) && widget.should_take_focus_after_interaction() {
            set_focused(Some(widget));
            if event.button() == 1 {
                self.set_dragging(true);
            }
        }
        true
    }

    /// `ContainerEventHandler.mouseReleased:57-66`
    pub fn mouse_released(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent) -> bool {
        if event.button() == 1 && self.is_dragging() {
            self.set_dragging(false);
            if let Some(focused) = self.get_focused() {
                return focused.mouse_released(event);
            }
        }
        false
    }

    /// `ContainerEventHandler.mouseDragged:68-71`
    pub fn mouse_dragged(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent, dx: f64, dy: f64) -> bool {
        match self.get_focused() {
            Some(focused) if self.is_dragging() && event.button() == 1 => focused.mouse_dragged(event, dx, dy),
            _ => false,
        }
    }

    /// `ContainerEventHandler.mouseScrolled:74-78`
    pub fn mouse_scrolled(children: &[std::sync::Arc<dyn GuiEventListener>], x: f64, y: f64, scroll_x: f64, scroll_y: f64) -> bool {
        ContainerEventHandler::get_child_at(children, x, y).is_some_and(|child| child.mouse_scrolled(x, y, scroll_x, scroll_y))
    }

    /// `ContainerEventHandler.keyPressed:82-85`
    pub fn key_pressed(&self, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        self.get_focused().is_some_and(|focused| focused.key_pressed(event))
    }

    /// `ContainerEventHandler.charTyped:92-95`
    pub fn char_typed(&self, event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent) -> bool {
        self.get_focused().is_some_and(|focused| focused.char_typed(event))
    }

    /// `ContainerEventHandler.setFocused(boolean)` - `if (!focused) setFocused(null)`.
    pub fn set_focused_boolean(&self, focused: bool) {
        if !focused {
            self.set_focused(None);
        }
    }

    /// `ContainerEventHandler.isFocused()` - `getFocused() != null`.
    pub fn is_focused(&self) -> bool {
        self.focused.borrow().is_some()
    }
}

/// `net.minecraft.client.gui.components.AbstractWidget`'s fields, for a game widget class the mod draws itself
/// (`Button`, the YACL description pane, the tab bar): Java's `x`, `y`, `width`, `height`, `message`, `active`, `visible`,
/// `alpha`, `isHovered`, `focused` and the tooltip, in cells.
pub struct WidgetFields {
    pub x: std::cell::Cell<i32>,
    pub y: std::cell::Cell<i32>,
    pub width: std::cell::Cell<i32>,
    pub height: std::cell::Cell<i32>,
    pub message: std::cell::RefCell<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent>,
    pub active: std::cell::Cell<bool>,
    pub visible: std::cell::Cell<bool>,
    pub alpha: std::cell::Cell<f32>,
    pub is_hovered: std::cell::Cell<bool>,
    pub focused: std::cell::Cell<bool>,
    /// `setTooltip(Tooltip)` - the tooltip's text.
    pub tooltip: std::cell::RefCell<Option<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent>>,
    /// `AbstractWidget.tooltip` - the holder's delay and display clock.
    pub tooltip_holder: mcrust::net::minecraft::client::gui::components::widget_tooltip_holder::WidgetTooltipHolder,
}

impl WidgetFields {
    /// `AbstractWidget(x, y, width, height, message)`
    pub fn new(x: i32, y: i32, width: i32, height: i32, message: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent) -> WidgetFields {
        WidgetFields {
            x: std::cell::Cell::new(x),
            y: std::cell::Cell::new(y),
            width: std::cell::Cell::new(width),
            height: std::cell::Cell::new(height),
            message: std::cell::RefCell::new(message),
            active: std::cell::Cell::new(true),
            visible: std::cell::Cell::new(true),
            alpha: std::cell::Cell::new(1.0),
            is_hovered: std::cell::Cell::new(false),
            focused: std::cell::Cell::new(false),
            tooltip: std::cell::RefCell::new(None),
            tooltip_holder: mcrust::net::minecraft::client::gui::components::widget_tooltip_holder::WidgetTooltipHolder::new(),
        }
    }

    /// `AbstractWidget.extractRenderState:60`'s `this.tooltip.refreshTooltipForNextRenderPass(graphics, mouseX, mouseY,
    /// isHovered, isFocused, getRectangle())` - `Tooltip.toCharSequence` (the message split at 170) parked with
    /// `MenuTooltipPositioner` over the widget (the mouse never reaches a keyboard-only positioner here: the YACL
    /// screen has no keyboard focus navigation, GAPS_Y2.md G7).
    pub fn refresh_tooltip_for_next_render_pass(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32) {
        use mcrust::net::minecraft::client::gui::screens::inventory::tooltip::menu_tooltip_positioner::MenuTooltipPositioner;
        let tooltip = self.tooltip.borrow().clone();
        let shown = self.tooltip_holder.refresh_with(
            mcrust::net::minecraft::util::util::get_millis(),
            tooltip.is_some(),
            self.is_hovered(),
            self.focused.get(),
            false,
        );
        if let (true, Some(tooltip)) = (shown, tooltip) {
            let lines = font().split(
                &mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(tooltip),
                mcrust::net::minecraft::client::gui::components::tooltip::MAX_WIDTH,
            );
            let rectangle = mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle::new_at(
                self.get_x(),
                self.get_y(),
                self.get_width(),
                self.get_height(),
            );
            graphics.set_tooltip_for_next_frame(lines, Box::new(MenuTooltipPositioner::new(rectangle)), mouse_x, mouse_y);
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x.get()
    }
    pub fn get_y(&self) -> i32 {
        self.y.get()
    }
    pub fn set_x(&self, x: i32) {
        self.x.set(x);
    }
    pub fn set_y(&self, y: i32) {
        self.y.set(y);
    }
    pub fn get_width(&self) -> i32 {
        self.width.get()
    }
    pub fn get_height(&self) -> i32 {
        self.height.get()
    }
    pub fn set_width(&self, width: i32) {
        self.width.set(width);
    }
    pub fn set_height(&self, height: i32) {
        self.height.set(height);
    }
    /// `getRight()`
    pub fn get_right(&self) -> i32 {
        self.get_x() + self.get_width()
    }
    /// `getBottom()`
    pub fn get_bottom(&self) -> i32 {
        self.get_y() + self.get_height()
    }
    /// `setMessage(Component)`
    pub fn set_message(&self, message: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent) {
        *self.message.borrow_mut() = message;
    }
    /// `getMessage()`
    pub fn get_message(&self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        self.message.borrow().clone()
    }
    /// `setTooltip(Tooltip.create(text))` / `setTooltip(null)`
    pub fn set_tooltip(&self, tooltip: Option<mcrust::net::minecraft::network::chat::mutable_component::MutableComponent>) {
        *self.tooltip.borrow_mut() = tooltip;
    }
    /// `isActive()` - `visible && active`.
    pub fn is_active(&self) -> bool {
        self.visible.get() && self.active.get()
    }
    /// `isHovered()`
    pub fn is_hovered(&self) -> bool {
        self.is_hovered.get()
    }
    /// `isHoveredOrFocused()`
    pub fn is_hovered_or_focused(&self) -> bool {
        self.is_hovered() || self.focused.get()
    }
    /// `areCoordinatesInRectangle(x, y)`
    pub fn are_coordinates_in_rectangle(&self, x: f64, y: f64) -> bool {
        x >= self.get_x() as f64 && y >= self.get_y() as f64 && x < self.get_right() as f64 && y < self.get_bottom() as f64
    }
    /// `AbstractWidget.isMouseOver:161-164`
    pub fn is_mouse_over(&self, x: f64, y: f64) -> bool {
        self.is_active() && self.are_coordinates_in_rectangle(x, y)
    }
    /// `AbstractWidget.extractRenderState:56-62`'s hover test (the pass's scissor is the list's own, applied by the
    /// caller that clips).
    pub fn update_hover(&self, mouse_x: i32, mouse_y: i32) {
        self.is_hovered.set(self.are_coordinates_in_rectangle(mouse_x as f64, mouse_y as f64));
    }
}

/// `AbstractButton.extractDefaultSprite(graphics)` + `extractDefaultLabel(..)` - what a plain `Button` draws - for a
/// button the mod draws itself: a one-widget `Screen` holding a `WidgetKind::Button`, brought up to the mod button's box
/// and state before each draw, drawn through the game's own button pass. Kept in the object (no allocation per frame
/// unless the message changed).
pub struct ButtonLook {
    screen: std::cell::RefCell<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
}

impl Default for ButtonLook {
    fn default() -> ButtonLook {
        ButtonLook::new()
    }
}

impl ButtonLook {
    pub fn new() -> ButtonLook {
        use mcrust::net::minecraft::client::gui::components::abstract_widget::{AbstractWidget, OnPress, WidgetKind};
        let mut screen = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        screen.add_renderable_widget(AbstractWidget::new(0, 0, 0, 0, String::new(), WidgetKind::Button, OnPress::Inert));
        ButtonLook { screen: std::cell::RefCell::new(screen) }
    }

    /// The button drawn over `fields`' box, message (`None`: no label), `active`, `visible`, alpha and focus.
    pub fn draw(
        &self,
        fields: &WidgetFields,
        message: Option<&mcrust::net::minecraft::network::chat::formatted_text::FormattedText>,
        graphics: &Graphics<'_>,
        mouse_x: i32,
        mouse_y: i32,
    ) {
        let mut screen = self.screen.borrow_mut();
        let b = screen.widget_mut(0);
        b.set_position(fields.get_x(), fields.get_y());
        b.width = fields.get_width();
        b.height = fields.get_height();
        match message {
            Some(message) => {
                if b.message_text.as_ref() != Some(message) {
                    b.message = message.get_string();
                    b.message_text = Some(message.clone());
                }
            }
            None => {
                if !b.message.is_empty() || b.message_text.is_some() {
                    b.message = String::new();
                    b.message_text = None;
                }
            }
        }
        b.active = fields.active.get();
        b.visible = fields.visible.get();
        b.alpha = fields.alpha.get();
        if b.is_focused() != fields.focused.get() {
            b.set_focused(fields.focused.get(), mcrust::net::minecraft::client::input_type::InputType::None);
        }
        graphics.widget_context().extract_widgets(&mut screen, mouse_x, mouse_y);
    }
}

/// `net.minecraft.client.gui.components.Button` as a mod screen that draws its own widgets holds one: the fields, the
/// `OnPress` lambda (handed the button, Java's `b`), and the game's look.
pub struct Button {
    pub fields: WidgetFields,
    on_press: std::cell::RefCell<Option<std::rc::Rc<dyn Fn(&Button)>>>,
    look: ButtonLook,
}

impl Button {
    /// `Button.builder(message, onPress).pos(x, y).size(width, height).build()`
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        message: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent,
        on_press: impl Fn(&Button) + 'static,
    ) -> Button {
        Button {
            fields: WidgetFields::new(x, y, width, height, message),
            on_press: std::cell::RefCell::new(Some(std::rc::Rc::new(on_press))),
            look: ButtonLook::new(),
        }
    }

    /// `onPress.onPress(this)` - a lambda installed after construction (Java's builder takes it first).
    pub fn set_on_press(&self, on_press: impl Fn(&Button) + 'static) {
        *self.on_press.borrow_mut() = Some(std::rc::Rc::new(on_press));
    }

    /// `Button.onPress(input)` - `this.onPress.onPress(this)`.
    pub fn on_press(&self) {
        let on_press = self.on_press.borrow().clone();
        if let Some(on_press) = on_press {
            on_press(self);
        }
    }

    /// `AbstractButton.extractWidgetRenderState` - the default sprite and label, with an explicit message (a subclass
    /// that draws its own label passes `None`).
    pub fn extract_default(&self, graphics: &Graphics<'_>, message: Option<&mcrust::net::minecraft::network::chat::formatted_text::FormattedText>, mouse_x: i32, mouse_y: i32) {
        self.look.draw(&self.fields, message, graphics, mouse_x, mouse_y);
    }

    /// `AbstractButton.extractDefaultSprite(graphics)` - `SPRITES.get(active, isHoveredOrFocused())` over the box, in
    /// `ARGB.white(alpha)`. `SPRITES` is `new WidgetSprites(BUTTON, BUTTON_DISABLED, BUTTON_HIGHLIGHTED)`.
    pub fn extract_default_sprite(&self, graphics: &Graphics<'_>) {
        use mcrust::net::minecraft::resources::identifier::Identifier;
        let f = &self.fields;
        let sprite = match (f.active.get(), f.is_hovered_or_focused()) {
            (true, true) => "widget/button_highlighted",
            (true, false) => "widget/button",
            (false, _) => "widget/button_disabled",
        };
        graphics.blit_sprite_with_color(
            &Identifier::with_default_namespace(sprite),
            f.get_x(),
            f.get_y(),
            f.get_width(),
            f.get_height(),
            mcrust::net::minecraft::util::argb::white_f32(f.alpha.get()),
        );
    }

    /// `AbstractButton.extractDefaultLabel(graphics.textRendererForWidget(this, NONE))` -
    /// `extractScrollingStringOverContents(output, getMessage(), 2)`: centred, or scrolling inside the box when it is
    /// too wide, in `ARGB.white(alpha)` with a drop shadow.
    pub fn extract_default_label(&self, graphics: &Graphics<'_>) {
        use mcrust::net::minecraft::client::gui::active_text_collector;
        let f = &self.fields;
        let message = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(f.get_message());
        let line_width = font().width_of(&message);
        let placement = active_text_collector::scrolling_string_over_contents(
            f.get_x(),
            f.get_y(),
            f.get_width(),
            f.get_height(),
            2,
            line_width,
            mcrust::net::minecraft::util::util::get_millis(),
        );
        if let Some((left, right, top, bottom)) = placement.scissor {
            graphics.enable_scissor(left, top, right, bottom);
        }
        graphics.text(&message, placement.x, placement.y, mcrust::net::minecraft::util::argb::white_f32(f.alpha.get()), true);
        if placement.scissor.is_some() {
            graphics.disable_scissor();
        }
    }

    /// `AbstractButton.handleCursor`: a pointing hand (not allowed when inactive) while hovered.
    pub fn handle_cursor(&self, graphics: &Graphics<'_>) {
        if self.fields.is_hovered() {
            graphics.request_cursor(if self.fields.is_active() { CursorKind::PointingHand } else { CursorKind::NotAllowed });
        }
    }
}

impl GuiEventListener for Button {
    /// `AbstractWidget.extractRenderState:56-62` -> `AbstractButton.extractWidgetRenderState`.
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        if !self.fields.visible.get() {
            return;
        }
        self.fields.update_hover(mouse_x, mouse_y);
        let message = mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(self.fields.get_message());
        self.extract_default(graphics, Some(&message), mouse_x, mouse_y);
        self.handle_cursor(graphics);
        self.fields.refresh_tooltip_for_next_render_pass(graphics, mouse_x, mouse_y);
    }

    /// `AbstractWidget.mouseClicked:109-125` -> `AbstractButton.onClick` -> `onPress`.
    fn mouse_clicked(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent, _double_click: bool) -> bool {
        if !self.fields.is_active() {
            return false;
        }
        if event.button() == 1 && self.is_mouse_over(event.x(), event.y()) {
            play_down_sound();
            self.on_press();
            return true;
        }
        false
    }

    /// `AbstractWidget.mouseReleased:127-135`
    fn mouse_released(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent) -> bool {
        event.button() == 1
    }

    /// `AbstractWidget.mouseDragged:141-149`
    fn mouse_dragged(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent, _dx: f64, _dy: f64) -> bool {
        event.button() == 1
    }

    /// `AbstractButton.keyPressed:61-72` - a selection key presses an active button.
    fn key_pressed(&self, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if !self.fields.is_active() {
            return false;
        }
        if event.is_selection() {
            play_down_sound();
            self.on_press();
            return true;
        }
        false
    }

    fn is_mouse_over(&self, x: f64, y: f64) -> bool {
        self.fields.is_mouse_over(x, y)
    }

    fn set_focused(&self, focused: bool) {
        self.fields.focused.set(focused);
    }

    fn is_focused(&self) -> bool {
        self.fields.focused.get()
    }
}

/// `net.minecraft.client.gui.components.EditBox` as a mod screen that draws its own widgets holds one (a subclass:
/// `SearchFieldWidget`): the game's `EditBox` state inside a one-widget `Screen` (the game's own draw), the widget's
/// fields, and the responder - run after an edit has finished (the box's `onValueChange` is the last statement of every
/// edit), so it may reach this box again.
pub struct EditBoxWidget {
    screen: std::cell::RefCell<mcrust::net::minecraft::client::gui::screens::screen::Screen>,
    responder: std::cell::RefCell<Option<std::rc::Rc<dyn Fn(&str)>>>,
    focused: std::cell::Cell<bool>,
}

impl EditBoxWidget {
    /// `new EditBox(font, x, y, width, height, message)`
    pub fn new(x: i32, y: i32, width: i32, height: i32, message: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent) -> EditBoxWidget {
        use mcrust::net::minecraft::client::gui::components::abstract_widget::{AbstractWidget, OnPress, WidgetKind};
        use mcrust::net::minecraft::network::chat::component::Component as _;
        let font = font();
        let edit = mcrust::net::minecraft::client::gui::components::edit_box::EditBox::new(&font.splitter(), x, y, width, height);
        let mut widget = AbstractWidget::new(x, y, width, height, message.get_string(), WidgetKind::EditBox(edit), OnPress::Inert);
        widget.message_text = Some(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(message));
        let mut screen = mcrust::net::minecraft::client::gui::screens::screen::Screen::new(String::new());
        screen.add_renderable_widget(widget);
        EditBoxWidget { screen: std::cell::RefCell::new(screen), responder: std::cell::RefCell::new(None), focused: std::cell::Cell::new(false) }
    }

    /// The game's widget (its box, `active`, `visible`, message).
    pub fn with_widget<R>(&self, f: impl FnOnce(&mut mcrust::net::minecraft::client::gui::components::abstract_widget::AbstractWidget) -> R) -> R {
        let mut screen = self.screen.borrow_mut();
        f(screen.widget_mut(0))
    }

    /// The game's `EditBox` state.
    pub fn with_edit_box<R>(&self, f: impl FnOnce(&mut mcrust::net::minecraft::client::gui::components::edit_box::EditBox) -> R) -> R {
        self.with_widget(|w| match &mut w.kind {
            mcrust::net::minecraft::client::gui::components::abstract_widget::WidgetKind::EditBox(e) => f(e),
            _ => unreachable!("EditBoxWidget holds an EditBox"),
        })
    }

    /// `setResponder(responder)`
    pub fn set_responder(&self, responder: impl Fn(&str) + 'static) {
        *self.responder.borrow_mut() = Some(std::rc::Rc::new(responder));
    }

    /// The responder, run for an edit that changed the value (`onValueChange`).
    fn after_edit(&self, before: &str) {
        let value = self.get_value();
        if value != before {
            let responder = self.responder.borrow().clone();
            if let Some(responder) = responder {
                responder(&value);
            }
        }
    }

    /// `getValue()`
    pub fn get_value(&self) -> String {
        self.with_edit_box(|e| e.get_value())
    }

    /// `setValue(value)` - the responder runs (`onValueChange`).
    pub fn set_value(&self, value: &str) {
        let before = self.get_value();
        let font = font();
        self.with_edit_box(|e| e.set_value(&font.splitter(), value));
        self.after_edit(&before);
    }

    /// `setValue(value)` without the responder (`SearchFieldWidget.setValueDoNotUpdate`).
    pub fn set_value_silently(&self, value: &str) {
        let font = font();
        self.with_edit_box(|e| e.set_value(&font.splitter(), value));
    }

    /// `getCursorPosition()`
    pub fn get_cursor_position(&self) -> i32 {
        self.with_edit_box(|e| e.get_cursor_position())
    }

    /// `setCursorPosition(pos)`
    pub fn set_cursor_position(&self, pos: i32) {
        let font = font();
        self.with_edit_box(|e| e.set_cursor_position(&font.splitter(), pos));
    }

    /// `setHint(component)`
    pub fn set_hint(&self, hint: mcrust::net::minecraft::network::chat::mutable_component::MutableComponent) {
        self.with_edit_box(|e| e.hint = Some(mcrust::net::minecraft::network::chat::formatted_text::FormattedText::from(hint)));
    }

    pub fn get_x(&self) -> i32 {
        self.with_widget(|w| w.get_x())
    }
    pub fn get_y(&self) -> i32 {
        self.with_widget(|w| w.get_y())
    }
    pub fn get_width(&self) -> i32 {
        self.with_widget(|w| w.get_width())
    }
    pub fn get_height(&self) -> i32 {
        self.with_widget(|w| w.get_height())
    }
    pub fn is_visible(&self) -> bool {
        self.with_widget(|w| w.visible)
    }
    pub fn is_active(&self) -> bool {
        self.with_widget(|w| w.is_active())
    }
}

impl GuiEventListener for EditBoxWidget {
    /// `EditBox.extractWidgetRenderState` (the game's), with `handleCursor`'s I-beam.
    fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
        let mut screen = self.screen.borrow_mut();
        let focused = self.focused.get();
        let w = screen.widget_mut(0);
        if w.is_focused() != focused {
            w.set_focused(focused, mcrust::net::minecraft::client::input_type::InputType::None);
        }
        let visible = w.visible;
        let hovered = visible && w.is_mouse_over(mouse_x as f64, mouse_y as f64);
        graphics.widget_context().extract_widgets(&mut screen, mouse_x, mouse_y);
        if hovered {
            graphics.request_cursor(CursorKind::IBeam);
        }
    }

    /// `AbstractWidget.mouseClicked` -> `EditBox.onClick:388-394` (the cursor moves to the press).
    fn mouse_clicked(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent, double_click: bool) -> bool {
        if !self.is_active() || event.button() != 1 || !self.is_mouse_over(event.x(), event.y()) {
            return false;
        }
        let font = font();
        let shift = {
            use mcrust::net::minecraft::client::input::input_with_modifiers::InputWithModifiers as _;
            event.button_info().has_shift_down()
        };
        self.with_edit_box(|e| e.on_click(&font.splitter(), event.x(), shift, double_click));
        true
    }

    /// `EditBox.onDrag:396-399`
    fn mouse_dragged(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent, _dx: f64, _dy: f64) -> bool {
        if event.button() != 1 {
            return false;
        }
        let font = font();
        self.with_edit_box(|e| {
            let pos = e.find_clicked_position_in_text(&font.splitter(), event.x());
            e.move_cursor_to(&font.splitter(), pos, true);
        });
        true
    }

    fn mouse_released(&self, event: &mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent) -> bool {
        event.button() == 1
    }

    /// `EditBox.keyPressed:280-345` with the window's clipboard.
    fn key_pressed(&self, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        let before = self.get_value();
        let font = font();
        let focused = self.focused.get();
        let active = self.is_active();
        let handled = self.with_edit_box(|e| {
            let mut memory = mcrust::net::minecraft::client::gui::components::edit_box::MemoryClipboard::default();
            match mcrust::net::minecraft::client::minecraft::SdlClipboard::published() {
                Some(mut clipboard) => e.key_pressed(&font.splitter(), *event, focused, active, &mut clipboard),
                None => e.key_pressed(&font.splitter(), *event, focused, active, &mut memory),
            }
        });
        self.after_edit(&before);
        handled
    }

    /// `EditBox.charTyped:352-360`
    fn char_typed(&self, event: &mcrust::net::minecraft::client::input::character_event::CharacterEvent) -> bool {
        let Some(c) = char::from_u32(event.codepoint() as u32) else {
            return false;
        };
        let before = self.get_value();
        let font = font();
        let focused = self.focused.get();
        let active = self.is_active();
        let handled = self.with_edit_box(|e| e.char_typed(&font.splitter(), c, focused, active));
        self.after_edit(&before);
        handled
    }

    fn is_mouse_over(&self, x: f64, y: f64) -> bool {
        self.with_widget(|w| w.is_mouse_over(x, y))
    }

    /// `EditBox.setFocused:529-541` - `canLoseFocus || focused`.
    fn set_focused(&self, focused: bool) {
        let can_lose_focus = self.with_edit_box(|e| e.can_lose_focus);
        if can_lose_focus || focused {
            self.focused.set(focused);
            self.with_widget(|w| w.set_focused(focused, mcrust::net::minecraft::client::input_type::InputType::None));
        }
    }

    fn is_focused(&self) -> bool {
        self.focused.get()
    }
}

/// A Java object reference held as a VALUE (an `Item` option's value): equal only to the same object (`==`, which is
/// what `Object.equals` answers for a class that does not override it).
///
/// Why not plain `Arc<T>`: `Arc<T>: PartialEq` compares the POINTEES and needs `T: PartialEq`, which `Item` does not
/// (and must not) implement; Java's `Option<Item>.pendingValue().equals(binding.getValue())` - the "changed" test -
/// is reference identity, so the wrapper's `PartialEq` is `Arc::ptr_eq`. The registry holds ONE `Arc<Item>` per item
/// (`ErasedItem`), so two lookups of the same id compare equal exactly as two Java reads of `Items.STONE` do. It is
/// `Send + Sync` whenever `T` is (an `Item` is: it lives in the static registry), so config statics can hold it.
pub struct JavaRef<T: ?Sized>(pub std::sync::Arc<T>);

impl<T: ?Sized> JavaRef<T> {
    pub fn new(value: std::sync::Arc<T>) -> JavaRef<T> {
        JavaRef(value)
    }
}

impl<T: ?Sized> Clone for JavaRef<T> {
    fn clone(&self) -> JavaRef<T> {
        JavaRef(self.0.clone())
    }
}

impl<T: ?Sized> PartialEq for JavaRef<T> {
    fn eq(&self, other: &JavaRef<T>) -> bool {
        std::sync::Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T: ?Sized> Eq for JavaRef<T> {}

impl<T: ?Sized> std::ops::Deref for JavaRef<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: ?Sized> std::fmt::Debug for JavaRef<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JavaRef@{:p}", std::sync::Arc::as_ptr(&self.0) as *const ())
    }
}

/// `net.minecraft.client.gui.components.tabs.Tab` for a screen that draws its own widgets (the YACL screen): the tab's
/// widgets are the mod's own (`GuiEventListener`), visited by the screen's `TabManager`.
pub trait Tab: DynAny {
    /// `Component getTabTitle()`
    fn get_tab_title(&self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

    /// `Component getTabExtraNarration()` - `CommonComponents.EMPTY` by default
    fn get_tab_extra_narration(&self) -> mcrust::net::minecraft::network::chat::mutable_component::MutableComponent {
        mcrust::net::minecraft::network::chat::component::empty()
    }

    /// `void visitChildren(Consumer<AbstractWidget> consumer)`
    fn visit_children(&self, consumer: &mut dyn FnMut(std::sync::Arc<dyn GuiEventListener>));

    /// `void doLayout(ScreenRectangle screenRectangle)`
    fn do_layout(&self, screen_rectangle: mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle);

    /// `tab instanceof TabExt tabExt ? tabExt : null`
    fn as_tab_ext(&self) -> Option<&dyn crate::dev::isxander::yacl3::gui::tab::tab_ext::TabExt> {
        None
    }
}

/// `NumberFormat.getInstance()` - the default locale's NUMBER pattern `#,##0.###` (grouping by three, at most three
/// fraction digits, HALF_EVEN) over the port's `DecimalFormat`. The port carries the US / ROOT symbols only, which
/// are what an English JVM uses (GAPS_Y2.md G9: a JVM in another default locale formats with its own).
pub fn number_format_instance() -> mcrust::java::text::decimal_format::DecimalFormat {
    use mcrust::java::text::decimal_format_symbols::{self, Locale};
    mcrust::java::text::decimal_format::DecimalFormat::new("#,##0.###", decimal_format_symbols::get_instance(Locale::Us))
}

/// `NumberFormat.getInstance().parse(text, parsePosition)` (`DecimalFormat.parse` -> `subparse` with the default
/// pattern's prefixes `""` / `"-"`, grouping on, not integer-only, not strict): parses from char index `start` and
/// answers the number's `doubleValue()` and the index after it, or `None` (the `ParsePosition` index unchanged) when
/// no number starts there. `NaN`, the infinity sign, the grouping separator inside the integer part and an `E`
/// exponent are recognised as `DecimalFormat` does.
pub fn number_format_parse(text: &str, start: usize) -> (Option<f64>, usize) {
    let chars: Vec<char> = text.chars().collect();
    let symbols = mcrust::java::text::decimal_format_symbols::get_instance(mcrust::java::text::decimal_format_symbols::Locale::Us);
    let region = |at: usize, s: &str| -> bool {
        let pat: Vec<char> = s.chars().collect();
        at + pat.len() <= chars.len() && chars[at..at + pat.len()] == pat[..]
    };
    // `DecimalFormat.parse:2088-2092` - NaN before any prefix
    if region(start, symbols.nan) {
        return (Some(f64::NAN), start + symbols.nan.chars().count());
    }
    // `subparse` with (positivePrefix "", negativePrefix "-"): the longer matching prefix wins
    let mut position = start;
    let negative = position < chars.len() && chars[position] == symbols.minus_sign;
    if negative {
        position += 1;
    }
    // `subparseNumber`: infinity
    if region(position, symbols.infinity) {
        let end = position + symbols.infinity.chars().count();
        return (Some(if negative { f64::NEG_INFINITY } else { f64::INFINITY }), end);
    }
    // the digits, the decimal separator, the grouping separators and the exponent
    let mut digits = String::new();
    let mut saw_decimal = false;
    let mut saw_digit = false;
    let mut backup: Option<usize> = None;
    let mut exponent: i64 = 0;
    while position < chars.len() {
        let ch = chars[position];
        if let Some(digit) = ch.to_digit(10) {
            saw_digit = true;
            backup = None;
            if saw_decimal && !digits.contains('.') {
                if digits.is_empty() {
                    digits.push('0');
                }
                digits.push('.');
            }
            digits.push(char::from_digit(digit, 10).unwrap_or('0'));
            position += 1;
        } else if ch == symbols.decimal_separator && !saw_decimal {
            saw_decimal = true;
            backup = None;
            position += 1;
        } else if ch == symbols.grouping_separator && !saw_decimal {
            // a grouping separator is consumed only when a digit follows (`backup`)
            if backup.is_none() {
                backup = Some(position);
            }
            position += 1;
        } else if region(position, "E") {
            // `subparseNumber` - the exponent, parsed recursively with prefixes ("", "-"); a failure leaves it unread
            let mut at = position + 1;
            let exponent_negative = at < chars.len() && chars[at] == symbols.minus_sign;
            if exponent_negative {
                at += 1;
            }
            let digits_start = at;
            let mut value: i64 = 0;
            while at < chars.len() {
                match chars[at].to_digit(10) {
                    Some(d) => {
                        value = value.saturating_mul(10).saturating_add(d as i64);
                        at += 1;
                    }
                    None => break,
                }
            }
            if at > digits_start {
                position = at;
                exponent = if exponent_negative { -value } else { value };
            }
            break;
        } else {
            break;
        }
    }
    if let Some(backup) = backup {
        position = backup;
    }
    if !saw_digit {
        return (None, start);
    }
    let mut literal = if digits.is_empty() { String::from("0") } else { digits };
    if exponent != 0 {
        literal.push_str(&format!("e{exponent}"));
    }
    let magnitude: f64 = literal.parse().unwrap_or(0.0);
    (Some(if negative { -magnitude } else { magnitude }), position)
}

/// `Integer.toString(int i, int radix)` - lowercase digits, a leading `-` for a negative value; a radix outside
/// 2..=36 is 10.
pub fn integer_to_string_radix(value: i32, radix: u32) -> String {
    let radix = if (2..=36).contains(&radix) { radix } else { 10 };
    let negative = value < 0;
    let mut magnitude = (value as i64).unsigned_abs();
    let mut digits = Vec::new();
    loop {
        let digit = (magnitude % radix as u64) as u32;
        digits.push(char::from_digit(digit, radix).unwrap_or('0'));
        magnitude /= radix as u64;
        if magnitude == 0 {
            break;
        }
    }
    if negative {
        digits.push('-');
    }
    digits.iter().rev().collect()
}

/// `Character.toLowerCase(char)` - the simple (one-to-one) lowercase mapping; a character whose full mapping is more
/// than one character keeps itself, as `UnicodeData`'s simple mapping does for those.
pub fn char_to_lower_case(ch: char) -> char {
    let mut lower = ch.to_lowercase();
    match (lower.next(), lower.next()) {
        (Some(single), None) => single,
        _ => ch,
    }
}

pub use tabs::{MenuTabButton, TabManager, TabNavigationBar};

/// `net.minecraft.client.gui.components.tabs` (`TabManager`, `TabNavigationBar` / `MenuTabBar`, `TabButton` /
/// `MenuTabBar.MenuTabButton`) for a screen that draws and dispatches its own widgets (the YACL screen). The game's
/// versions live in the screen's widget tree with tabs as indices (`tab_manager.rs`, `menu_tab_bar.rs`); these hold the
/// mod's `Tab` objects and reuse the game's sprites and geometry (`menu_tab_bar`).
pub mod tabs {
    use super::{ContainerEventHandler, CursorKind, Graphics, GuiEventListener, Tab, WidgetFields};
    use mcrust::net::minecraft::client::gui::active_text_collector;
    use mcrust::net::minecraft::client::gui::components::tabs::menu_tab_bar;
    use mcrust::net::minecraft::client::gui::navigation::screen_rectangle::ScreenRectangle;
    use mcrust::net::minecraft::client::input::key_event::KeyEvent;
    use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
    use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
    use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
    use mcrust::net::minecraft::resources::identifier::Identifier;
    use mcrust::net::minecraft::util::mth;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::Arc;

    /// `Objects.equals(a, b)` for two tabs: `Tab` does not override `equals`, so identity.
    pub fn same_tab(a: &Arc<dyn Tab>, b: &Arc<dyn Tab>) -> bool {
        std::ptr::eq(Arc::as_ptr(a) as *const (), Arc::as_ptr(b) as *const ())
    }

    /// `public class TabManager`
    pub struct TabManager {
        /// `private final Consumer<AbstractWidget> addWidget;`
        add_widget: Box<dyn Fn(Arc<dyn GuiEventListener>)>,
        /// `private final Consumer<AbstractWidget> removeWidget;`
        remove_widget: Box<dyn Fn(Arc<dyn GuiEventListener>)>,
        /// `@Nullable private Tab currentTab;`
        current_tab: RefCell<Option<Arc<dyn Tab>>>,
        /// `@Nullable private ScreenRectangle tabArea;`
        tab_area: Cell<Option<ScreenRectangle>>,
    }

    impl TabManager {
        /// `public TabManager(Consumer<AbstractWidget> addWidget, Consumer<AbstractWidget> removeWidget)` - the
        /// `onSelected` / `onDeselected` consumers are `t -> {}`.
        pub fn new(
            add_widget: impl Fn(Arc<dyn GuiEventListener>) + 'static,
            remove_widget: impl Fn(Arc<dyn GuiEventListener>) + 'static,
        ) -> TabManager {
            TabManager {
                add_widget: Box::new(add_widget),
                remove_widget: Box::new(remove_widget),
                current_tab: RefCell::new(None),
                tab_area: Cell::new(None),
            }
        }

        /// `public void setTabArea(ScreenRectangle tabArea)`
        pub fn set_tab_area(&self, tab_area: ScreenRectangle) {
            self.tab_area.set(Some(tab_area));
            if let Some(tab) = self.get_current_tab() {
                tab.do_layout(tab_area);
            }
        }

        /// `public void setCurrentTab(Tab tab, boolean playSound)`
        pub fn set_current_tab(&self, tab: Arc<dyn Tab>, play_sound: bool) {
            self.set_current_tab_3(tab, play_sound, true);
        }

        /// `public void setCurrentTab(Tab tab, boolean playSound, boolean addWidget)`
        pub fn set_current_tab_3(&self, tab: Arc<dyn Tab>, play_sound: bool, add_widget: bool) {
            let current = self.get_current_tab();
            if current.as_ref().is_some_and(|current| same_tab(current, &tab)) {
                return;
            }
            if let Some(current) = current {
                current.visit_children(&mut |widget| (self.remove_widget)(widget));
            }
            *self.current_tab.borrow_mut() = Some(tab.clone());
            if add_widget {
                tab.visit_children(&mut |widget| (self.add_widget)(widget));
            }
            if let Some(tab_area) = self.tab_area.get() {
                tab.do_layout(tab_area);
            }
            if play_sound {
                // Minecraft.getInstance().getSoundManager().play(SimpleSoundInstance.forUI(SoundEvents.UI_BUTTON_CLICK, 1.0F));
                super::play_down_sound();
            }
            // this.onDeselected.accept(oldTab); this.onSelected.accept(this.currentTab); - both `t -> {}`
        }

        /// `@Nullable public Tab getCurrentTab()`
        pub fn get_current_tab(&self) -> Option<Arc<dyn Tab>> {
            self.current_tab.borrow().clone()
        }
    }

    /// `MenuTabBar.MenuTabButton extends TabButton extends AbstractWidget.WithInactiveMessage`
    pub struct MenuTabButton {
        /// the `AbstractWidget` fields
        pub fields: WidgetFields,
        /// `private final TabManager tabManager;`
        tab_manager: Rc<TabManager>,
        /// `private final Tab tab;`
        tab: Arc<dyn Tab>,
    }

    impl MenuTabButton {
        /// `public MenuTabButton(TabManager tabManager, Tab tab, int width, int height)` -
        /// `super(0, 0, width, height, tab.getTabTitle())`
        pub fn new(tab_manager: Rc<TabManager>, tab: Arc<dyn Tab>, width: i32, height: i32) -> MenuTabButton {
            let title = tab.get_tab_title();
            MenuTabButton { fields: WidgetFields::new(0, 0, width, height, title), tab_manager, tab }
        }

        /// `public Tab tab()`
        pub fn tab(&self) -> Arc<dyn Tab> {
            self.tab.clone()
        }

        /// `public boolean isSelected()` - `this.tabManager.getCurrentTab() == this.tab`
        pub fn is_selected(&self) -> bool {
            self.tab_manager.get_current_tab().is_some_and(|current| same_tab(&current, &self.tab))
        }

        /// `protected void renderMenuBackground(..)` -
        /// `Screen.extractMenuBackgroundTexture(graphics, MENU_BACKGROUND, x0, y0, 0, 0, x1 - x0, y1 - y0)`
        fn render_menu_background(&self, graphics: &Graphics<'_>, x0: i32, y0: i32, x1: i32, y1: i32) {
            graphics.blit(
                &Identifier::with_default_namespace("textures/gui/menu_background.png"),
                x0,
                y0,
                0.0,
                0.0,
                x1 - x0,
                y1 - y0,
                32,
                32,
            );
        }

        /// `private void renderLabel(ActiveTextCollector output)`
        fn render_label(&self, graphics: &Graphics<'_>) {
            let f = &self.fields;
            let left = f.get_x() + 1;
            let top = f.get_y() + if self.is_selected() { 0 } else { menu_tab_bar::SELECTED_OFFSET };
            let right = f.get_x() + f.get_width() - 1;
            let bottom = f.get_y() + f.get_height();
            let message = FormattedText::from(f.get_message());
            let placement = active_text_collector::accept_scrolling_with_default_center(
                left,
                right,
                top,
                bottom,
                super::font().width_of(&message),
                mcrust::net::minecraft::util::util::get_millis(),
            );
            if let Some((l, r, t, b)) = placement.scissor {
                graphics.enable_scissor(l, t, r, b);
            }
            graphics.text(&message, placement.x, placement.y, mcrust::net::minecraft::util::argb::white_f32(f.alpha.get()), true);
            if placement.scissor.is_some() {
                graphics.disable_scissor();
            }
        }

        /// `private void renderFocusUnderline(GuiGraphicsExtractor graphics, Font font, int color)`
        fn render_focus_underline(&self, graphics: &Graphics<'_>, color: i32) {
            let f = &self.fields;
            let message_width = super::font().width_of(&FormattedText::from(f.get_message()));
            let (left, top, right, bottom) =
                menu_tab_bar::underline_bounds(f.get_x(), f.get_y(), f.get_width(), f.get_height(), message_width);
            graphics.fill(left, top, right, bottom, color);
        }
    }

    impl GuiEventListener for MenuTabButton {
        /// `AbstractWidget.extractRenderState` -> `MenuTabButton.extractWidgetRenderState`
        fn extract_render_state(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, _a: f32) {
            let f = &self.fields;
            if !f.visible.get() {
                return;
            }
            f.update_hover(mouse_x, mouse_y);

            graphics.blit_sprite(
                &menu_tab_bar::sprite(self.is_selected(), f.is_hovered_or_focused()),
                f.get_x(),
                f.get_y(),
                f.get_width(),
                f.get_height(),
            );
            let underline_color = menu_tab_bar::underline_color(f.active.get());
            if self.is_selected() {
                self.render_menu_background(graphics, f.get_x() + 2, f.get_y() + 2, f.get_right() - 2, f.get_bottom());
                self.render_focus_underline(graphics, underline_color);
            }
            self.render_label(graphics);
            // this.handleCursor(graphics);
            if f.is_hovered() {
                graphics.request_cursor(if f.is_active() { CursorKind::PointingHand } else { CursorKind::NotAllowed });
            }
            f.refresh_tooltip_for_next_render_pass(graphics, mouse_x, mouse_y);
        }

        /// `AbstractWidget.mouseClicked` - `playDownSound` is empty for a `TabButton` and `onClick` does nothing; the
        /// bar switches the tab when it takes the focus.
        fn mouse_clicked(&self, event: &MouseButtonEvent, _double_click: bool) -> bool {
            self.fields.is_active() && event.button() == 1 && self.is_mouse_over(event.x(), event.y())
        }

        fn mouse_released(&self, event: &MouseButtonEvent) -> bool {
            event.button() == 1
        }

        fn mouse_dragged(&self, event: &MouseButtonEvent, _dx: f64, _dy: f64) -> bool {
            event.button() == 1
        }

        fn is_mouse_over(&self, x: f64, y: f64) -> bool {
            self.fields.is_mouse_over(x, y)
        }

        fn set_focused(&self, focused: bool) {
            self.fields.focused.set(focused);
        }

        fn is_focused(&self) -> bool {
            self.fields.focused.get()
        }
    }

    /// `public class TabNavigationBar extends AbstractContainerWidget` and `MenuTabBar extends TabNavigationBar` - the
    /// fields and the bodies a subclass (`ScrollableNavigationBar`) builds on. Its `layout` (a `FrameLayout` holding
    /// a horizontal `LinearLayout` of the buttons) is the position the buttons are laid out from.
    pub struct TabNavigationBar {
        /// the `AbstractWidget` fields
        pub fields: WidgetFields,
        /// `protected final FrameLayout layout;` - its position
        layout_x: Cell<i32>,
        layout_y: Cell<i32>,
        /// `private final TabManager tabManager;`
        tab_manager: Rc<TabManager>,
        /// `protected final ImmutableList<Tab> tabs;`
        pub tabs: Vec<Arc<dyn Tab>>,
        /// `protected final ImmutableList<TabButton> tabButtons;`
        pub tab_buttons: Vec<Arc<MenuTabButton>>,
        /// `AbstractContainerWidget`'s `focused` / `isDragging`
        pub container: ContainerEventHandler,
    }

    impl TabNavigationBar {
        /// `protected TabNavigationBar(int x, int y, int width, int height, TabManager tabManager,
        /// ImmutableList<TabButton> tabButtons, ImmutableList<Tab> tabs)`
        pub fn new(
            x: i32,
            y: i32,
            width: i32,
            height: i32,
            tab_manager: Rc<TabManager>,
            tab_buttons: Vec<Arc<MenuTabButton>>,
            tabs: Vec<Arc<dyn Tab>>,
        ) -> TabNavigationBar {
            let bar = TabNavigationBar {
                fields: WidgetFields::new(x, y, width, height, mcrust::net::minecraft::network::chat::component::empty()),
                layout_x: Cell::new(x),
                layout_y: Cell::new(y),
                tab_manager,
                tabs,
                tab_buttons,
                container: ContainerEventHandler::new(),
            };
            bar.layout_arrange_elements();
            bar
        }

        /// `layout.arrangeElements()` - the buttons side by side from the layout's position
        pub fn layout_arrange_elements(&self) {
            let mut x = self.layout_x.get();
            for button in &self.tab_buttons {
                button.fields.set_x(x);
                button.fields.set_y(self.layout_y.get());
                x += button.fields.get_width();
            }
        }

        /// `layout.getX()`
        pub fn layout_x(&self) -> i32 {
            self.layout_x.get()
        }

        /// `layout.getY()`
        pub fn layout_y(&self) -> i32 {
            self.layout_y.get()
        }

        /// `layout.setX(x)` - moves the laid-out buttons with it
        pub fn layout_set_x(&self, x: i32) {
            let dx = x - self.layout_x.get();
            self.layout_x.set(x);
            for button in &self.tab_buttons {
                button.fields.set_x(button.fields.get_x() + dx);
            }
        }

        /// `layout.setY(y)`
        pub fn layout_set_y(&self, y: i32) {
            let dy = y - self.layout_y.get();
            self.layout_y.set(y);
            for button in &self.tab_buttons {
                button.fields.set_y(button.fields.get_y() + dy);
            }
        }

        /// `layout.getHeight()` - the tallest button
        pub fn layout_height(&self) -> i32 {
            self.tab_buttons.iter().map(|b| b.fields.get_height()).max().unwrap_or(0)
        }

        /// `public List<Tab> getTabs()`
        pub fn get_tabs(&self) -> &[Arc<dyn Tab>] {
            &self.tabs
        }

        /// the `tabManager` field
        pub fn tab_manager(&self) -> Rc<TabManager> {
            self.tab_manager.clone()
        }

        /// `public List<? extends GuiEventListener> children()` - the tab buttons
        pub fn children(&self) -> Vec<Arc<dyn GuiEventListener>> {
            self.tab_buttons.iter().map(|b| b.clone() as Arc<dyn GuiEventListener>).collect()
        }

        /// `public void setFocused(@Nullable GuiEventListener focused)` - `super.setFocused(focused)`, then the tab of
        /// an active `TabButton` becomes current
        pub fn set_focused_child(&self, focused: Option<Arc<dyn GuiEventListener>>) {
            self.container.set_focused(focused.clone());
            if let Some(focused) = focused
                && let Some(button) = self.button_of(&focused)
                && button.fields.is_active()
            {
                self.tab_manager.set_current_tab(button.tab(), true);
            }
        }

        /// `focused instanceof TabButton button` - the button `focused` is
        pub fn button_of(&self, focused: &Arc<dyn GuiEventListener>) -> Option<Arc<MenuTabButton>> {
            self.tab_buttons
                .iter()
                .find(|b| std::ptr::eq(Arc::as_ptr(b) as *const (), Arc::as_ptr(focused) as *const ()))
                .cloned()
        }

        /// `public void selectTab(int index, boolean playSound)` - `set_focus` is the owner's
        /// `setFocused(GuiEventListener)`
        pub fn select_tab(&self, index: usize, play_sound: bool, set_focus: &dyn Fn(Option<Arc<dyn GuiEventListener>>)) {
            if self.container.is_focused() {
                set_focus(Some(self.tab_buttons[index].clone()));
            } else if self.tab_buttons[index].fields.is_active() {
                self.tab_manager.set_current_tab(self.tabs[index].clone(), play_sound);
            }
        }

        /// `public void setTabActiveState(int index, boolean active)`
        pub fn set_tab_active_state(&self, index: i32, active: bool) {
            if index >= 0 && (index as usize) < self.tab_buttons.len() {
                self.tab_buttons[index as usize].fields.active.set(active);
            }
        }

        /// `public void setTabTooltip(int index, @Nullable Tooltip hint)`
        pub fn set_tab_tooltip(&self, index: i32, hint: Option<MutableComponent>) {
            if index >= 0 && (index as usize) < self.tab_buttons.len() {
                self.tab_buttons[index as usize].fields.set_tooltip(hint);
            }
        }

        /// `public boolean keyPressed(KeyEvent event)` - Ctrl+digit / Ctrl+Tab switch tabs
        pub fn key_pressed(&self, event: &KeyEvent, set_focus: &dyn Fn(Option<Arc<dyn GuiEventListener>>)) -> bool {
            if event.has_control_down_with_quirk() {
                let tab_index = self.get_next_tab_index(self.current_tab_index(), event);
                if tab_index != -1 {
                    self.select_tab(mth::clamp_i32(tab_index, 0, self.tabs.len() as i32 - 1) as usize, true, set_focus);
                    return true;
                }
            }
            false
        }

        /// `private int getNextTabIndex(int currentTab, KeyEvent event)`
        fn get_next_tab_index(&self, current_tab: i32, event: &KeyEvent) -> i32 {
            let digit = mcrust::com::mojang::blaze3d::platform::input_constants::Type::Keyboard.get_or_create(event.key).get_numeric_key_value().unwrap_or(-1);
            if digit != -1 {
                (digit - 1).rem_euclid(10)
            } else if event.is_cycle_focus() && current_tab != -1 {
                let next_tab_index = if event.has_shift_down() { current_tab - 1 } else { current_tab + 1 };
                let index = next_tab_index.rem_euclid(self.tabs.len() as i32);
                if self.tab_buttons[index as usize].fields.active.get() { index } else { self.get_next_tab_index(index, event) }
            } else {
                -1
            }
        }

        /// `private int currentTabIndex()`
        pub fn current_tab_index(&self) -> i32 {
            let Some(current_tab) = self.tab_manager.get_current_tab() else {
                return -1;
            };
            self.tabs.iter().position(|tab| same_tab(tab, &current_tab)).map_or(-1, |index| index as i32)
        }

        /// `@Nullable private TabButton currentTabButton()`
        pub fn current_tab_button(&self) -> Option<Arc<MenuTabButton>> {
            let index = self.current_tab_index();
            if index != -1 { Some(self.tab_buttons[index as usize].clone()) } else { None }
        }

        /// `MenuTabBar.extractWidgetRenderState` - the two header separators, then `TabNavigationBar`'s (every button)
        pub fn extract_menu_tab_bar(&self, graphics: &Graphics<'_>, mouse_x: i32, mouse_y: i32, a: f32) {
            let separator = Identifier::with_default_namespace("textures/gui/header_separator.png");
            let y = self.layout_y() + self.layout_height() - 2;
            if let Some(first) = self.tab_buttons.first() {
                graphics.blit(&separator, 0, y, 0.0, 0.0, first.fields.get_x(), 2, 32, 2);
            }
            if let Some(last) = self.tab_buttons.last() {
                graphics.blit(&separator, last.fields.get_right(), y, 0.0, 0.0, self.fields.get_width(), 2, 32, 2);
            }
            for button in &self.tab_buttons {
                button.extract_render_state(graphics, mouse_x, mouse_y, a);
            }
        }

        /// `TabNavigationBar.isMouseOver` - over one of the laid-out buttons
        pub fn is_mouse_over_layout(&self, mouse_x: f64, mouse_y: f64) -> bool {
            self.tab_buttons.iter().any(|b| {
                let f = &b.fields;
                ScreenRectangle::new_at(f.get_x(), f.get_y(), f.get_width(), f.get_height()).contains_point(mouse_x as i32, mouse_y as i32)
            })
        }
    }
}
