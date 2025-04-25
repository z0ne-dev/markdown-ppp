//! Fully‑typed Abstract Syntax Tree (AST) for CommonMark + GitHub Flavored Markdown (GFM)
//! ------------------------------------------------------------------------------------
//! This module models every construct described in the **CommonMark 1.0 specification**
//! together with the widely‑used **GFM extensions**: tables, strikethrough, autolinks,
//! task‑list items and footnotes.
//!
//! The design separates **block‑level** and **inline‑level** nodes because parsers and
//! renderers typically operate on these tiers independently.
//!
//! ```text
//! Document ─┐
//!           └─ Block ─┐
//!                     ├─ Inline
//!                     └─ ...
//! ```

// ——————————————————————————————————————————————————————————————————————————
// Document root
// ——————————————————————————————————————————————————————————————————————————

/// Root of a Markdown document
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// Top‑level block sequence **in document order**.
    pub blocks: Vec<Block>,
}

// ——————————————————————————————————————————————————————————————————————————
// Block‑level nodes
// ——————————————————————————————————————————————————————————————————————————

/// Block‑level constructs in the order they appear in the CommonMark spec.
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    /// Ordinary paragraph
    Paragraph(Vec<Inline>),

    /// ATX (`# Heading`) or Setext (`===`) heading
    Heading(Heading),

    /// Thematic break (horizontal rule)
    ThematicBreak,

    /// Block quote
    BlockQuote(Vec<Block>),

    /// List (bullet or ordered)
    List(List),

    /// Fenced or indented code block
    CodeBlock(CodeBlock),

    /// Raw HTML block
    HtmlBlock(String),

    /// Link reference definition.  Preserved for round‑tripping.
    Definition(LinkDefinition),

    /// Tables
    Table(Table),

    /// Footnote definition
    FootnoteDefinition(FootnoteDefinition),

    /// Empty block. This is used to represent skipped blocks in the AST.
    Empty,
}

/// Heading with level 1–6 and inline content.
#[derive(Debug, Clone, PartialEq)]
pub struct Heading {
    /// Heading level (1 ⇒ `<h1>`, …, 6 ⇒ `<h6>`).
    pub level: u8,

    /// Inlines that form the heading text (before trimming).
    pub content: Vec<Inline>,
}

// ——————————————————————————————————————————————————————————————————————————
// Lists
// ——————————————————————————————————————————————————————————————————————————

/// A list container — bullet or ordered.
#[derive(Debug, Clone, PartialEq)]
pub struct List {
    /// Kind of list together with additional semantic data (start index or
    /// bullet marker).
    pub kind: ListKind,

    /// List items in source order.
    pub items: Vec<ListItem>,
}

/// Specifies *what kind* of list we have.
#[derive(Debug, Clone, PartialEq)]
pub enum ListKind {
    /// Ordered list (`1.`, `42.` …) with an *optional* explicit start number.
    Ordered(ListOrderedKindOptions),

    /// Bullet list (`-`, `*`, or `+`) together with the concrete marker.
    Bullet(ListBulletKind),
}

/// Specifies *what kind* of list we have.
#[derive(Debug, Clone, PartialEq)]
pub struct ListOrderedKindOptions {
    /// Start index (1, 2, …) for ordered lists.
    pub start: u64,
}

/// Concrete bullet character used for a bullet list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListBulletKind {
    /// `-` U+002D
    Dash,

    /// `*` U+002A
    Star,

    /// `+` U+002B
    Plus,
}

/// Item within a list.
#[derive(Debug, Clone, PartialEq)]
pub struct ListItem {
    /// Task‑list checkbox state (GFM task‑lists). `None` ⇒ not a task list.
    pub task: Option<TaskState>,

    /// Nested blocks inside the list item.
    pub blocks: Vec<Block>,
}

/// State of a task‑list checkbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    /// Unchecked (GFM task‑list item)
    Incomplete,

    /// Checked (GFM task‑list item)
    Complete,
}

// ——————————————————————————————————————————————————————————————————————————
// Code blocks
// ——————————————————————————————————————————————————————————————————————————

/// Fenced or indented code block.
#[derive(Debug, Clone, PartialEq)]
pub struct CodeBlock {
    /// Distinguishes indented vs fenced code and stores the *info string*.
    pub kind: CodeBlockKind,

    /// Literal text inside the code block **without** final newline trimming.
    pub literal: String,
}

/// The concrete kind of a code block.
#[derive(Debug, Clone, PartialEq)]
pub enum CodeBlockKind {
    /// Indented block (≥ 4 spaces or 1 tab per line).
    Indented,

    /// Fenced block with *optional* info string (language, etc.).
    Fenced { info: Option<String> },
}

// ——————————————————————————————————————————————————————————————————————————
// Link reference definitions
// ——————————————————————————————————————————————————————————————————————————

/// Link reference definition (GFM) with a label, destination and optional title.
#[derive(Debug, Clone, PartialEq)]
pub struct LinkDefinition {
    /// Normalized label (acts as the *identifier*).
    pub label: String,

    /// Link URL (absolute or relative) or email address.
    pub destination: String,

    /// Optional title (for links and images).
    pub title: Option<String>,
}

// ——————————————————————————————————————————————————————————————————————————
// Tables
// ——————————————————————————————————————————————————————————————————————————

/// A table is a collection of rows and columns with optional alignment.
/// The first row is the header row.
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    /// Each row is a vector of *cells*; header row is **row 0**.
    pub rows: Vec<TableRow>,

    /// Column alignment; `alignments.len() == column_count`.
    pub alignments: Vec<Alignment>,
}

/// A table row is a vector of cells (columns).
pub type TableRow = Vec<TableCell>;

/// A table cell is a vector of inlines (text, links, etc.).
pub type TableCell = Vec<Inline>;

/// Specifies the alignment of a table cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    /// No alignment specified
    None,

    /// Left aligned
    Left,

    /// Right aligned
    Center,

    /// Right aligned
    Right,
}

// ——————————————————————————————————————————————————————————————————————————
// Footnotes
// ——————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, PartialEq)]
pub struct FootnoteDefinition {
    /// Normalized label (without leading `^`).
    pub label: String,

    /// Footnote content (blocks).
    pub blocks: Vec<Block>,
}

// ——————————————————————————————————————————————————————————————————————————
// Inline‑level nodes
// ——————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, PartialEq)]
pub enum Inline {
    /// Plain text (decoded entity references, preserved backslash escapes).
    Text(String),

    /// Hard line break
    LineBreak,

    /// Inline code span
    Code(String),

    /// Raw HTML fragment
    Html(String),

    /// Link to a destination with optional title.
    Link(Link),

    /// Reference link
    LinkReference(LinkReference),

    /// Image with optional title.
    Image(Link),

    /// Emphasis (`*` / `_`)
    Emphasis(Vec<Inline>),
    /// Strong emphasis (`**` / `__`)
    Strong(Vec<Inline>),
    /// Strikethrough (`~~`)
    Strikethrough(Vec<Inline>),

    /// Autolink (`<https://>` or `<mailto:…>`)
    Autolink(String),

    /// Footnote reference (`[^label]`)
    FootnoteReference(String),

    /// Empty element. This is used to represent skipped elements in the AST.
    Empty,
}

/// Re‑usable structure for links and images (destination + children).
#[derive(Debug, Clone, PartialEq)]
pub struct Link {
    /// Destination URL (absolute or relative) or email address.
    pub destination: String,

    /// Optional title (for links and images).
    pub title: Option<String>,

    /// Inline content (text, code, etc.) inside the link or image.
    pub children: Vec<Inline>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LinkReference {
    /// Normalized label (acts as the *identifier*).
    pub label: String,

    /// Link text
    pub text: String,
}
