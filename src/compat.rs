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
    value: std::cell::UnsafeCell<Option<T>>,
    owner: std::sync::OnceLock<std::thread::ThreadId>,
}

// SAFETY: every access goes through `check`, which pins the cell to one thread.
unsafe impl<T> Sync for MainThread<T> {}
unsafe impl<T> Send for MainThread<T> {}

impl<T> MainThread<T> {
    pub const fn new(init: fn() -> T) -> MainThread<T> {
        MainThread { init, value: std::cell::UnsafeCell::new(None), owner: std::sync::OnceLock::new() }
    }

    fn check(&self) {
        let me = std::thread::current().id();
        let owner = *self.owner.get_or_init(|| me);
        if owner != me {
            panic!(
                "fabric2seer MainThread<{}>: a static first used on {:?} was reached from {:?} - Java only touches it on the render thread",
                std::any::type_name::<T>(),
                owner,
                me
            );
        }
    }

    /// The value (created on first use).
    pub fn get(&self) -> &T {
        self.get_mut()
    }

    /// The value, mutably (a Java field write / mutating call).
    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        self.check();
        // SAFETY: one thread (checked above); no reference outlives the statement that asked for it in the
        // generated code.
        let slot = unsafe { &mut *self.value.get() };
        if slot.is_none() {
            *slot = Some((self.init)());
        }
        slot.as_mut().unwrap()
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
    s.chars().all(|c| c.is_whitespace())
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
    mcrust::net::minecraft::client::gui::font::Font { set: &minecraft().font }
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

/// `com.google.gson.FieldNamingPolicy` (only `IDENTITY` is what the generated forms write).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldNamingPolicy {
    Identity,
}

/// `new GsonBuilder()..create()`: the options the generated forms honour (`setPrettyPrinting`; `serializeNulls` and
/// `IDENTITY` field names are how they are written; the type adapters a config uses are built into each form).
#[derive(Clone, Debug, Default)]
pub struct GsonBuilder {
    pub pretty: bool,
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
    pub fn set_field_naming_policy(self, _policy: FieldNamingPolicy) -> GsonBuilder {
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
        Gson { pretty: self.pretty }
    }
}

/// `com.google.gson.Gson` over the generated per-class forms.
#[derive(Clone, Debug, Default)]
pub struct Gson {
    pub pretty: bool,
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
) -> Option<Shared<L>> {
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
