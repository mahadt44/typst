use super::TextNode;
use crate::prelude::*;

/// # Space
/// A text space.
///
/// ## Category
/// text
#[func]
#[capable(Unlabellable, Behave)]
#[derive(Debug, Hash)]
pub struct SpaceNode;

#[node]
impl SpaceNode {
    fn construct(_: &Vm, _: &mut Args) -> SourceResult<Content> {
        Ok(Self.pack())
    }
}

impl Unlabellable for SpaceNode {}

impl Behave for SpaceNode {
    fn behaviour(&self) -> Behaviour {
        Behaviour::Weak(2)
    }
}

/// # Line Break
/// Inserts a line break.
///
/// Advances the paragraph to the next line. A single trailing linebreak at the
/// end of a paragraph is ignored, but more than one creates additional empty
/// lines.
///
/// ## Example
/// ```
/// *Date:* 26.12.2022 \
/// *Topic:* Infrastructure Test \
/// *Severity:* High \
/// ```
///
/// ## Syntax
/// This function also has dedicated syntax: To insert a linebreak, simply write
/// a backslash followed by whitespace. This always creates an unjustified
/// break.
///
/// ## Parameters
/// - justify: bool (named)
///   Whether to justify the line before the break.
///
///   This is useful if you found a better line break opportunity in your
///   justified text than Typst did.
///
///   ### Example
///   ```
///   #set par(justify: true)
///   #let jb = linebreak(justify: true)
///
///   I have manually tuned the #jb
///   linebreaks in this paragraph #jb
///   for an _interesting_ result. #jb
///   ```
///
/// ## Category
/// text
#[func]
#[capable(Behave)]
#[derive(Debug, Hash)]
pub struct LinebreakNode {
    pub justify: bool,
}

#[node]
impl LinebreakNode {
    fn construct(_: &Vm, args: &mut Args) -> SourceResult<Content> {
        let justify = args.named("justify")?.unwrap_or(false);
        Ok(Self { justify }.pack())
    }

    fn field(&self, name: &str) -> Option<Value> {
        match name {
            "justify" => Some(Value::Bool(self.justify)),
            _ => None,
        }
    }
}

impl Behave for LinebreakNode {
    fn behaviour(&self) -> Behaviour {
        Behaviour::Destructive
    }
}

/// # Strong Emphasis
/// Strongly emphasizes content by increasing the font weight.
///
/// Increases the current font weight by a given `delta`.
///
/// ## Example
/// ```
/// This is *strong.* \
/// This is #strong[too.] \
///
/// #show strong: set text(red)
/// And this is *evermore.*
/// ```
///
/// ## Syntax
/// This function also has dedicated syntax: To strongly emphasize content,
/// simply enclose it in stars/asterisks (`*`). Note that this only works at
/// word boundaries. To strongly emphasize part of a word, you have to use the
/// function.
///
/// ## Parameters
/// - body: Content (positional, required)
///   The content to strongly emphasize.
///
/// ## Category
/// text
#[func]
#[capable(Show)]
#[derive(Debug, Hash)]
pub struct StrongNode(pub Content);

#[node]
impl StrongNode {
    /// The delta to apply on the font weight.
    ///
    /// # Example
    /// ```
    /// #set strong(delta: 0)
    /// No *effect!*
    /// ```
    pub const DELTA: i64 = 300;

    fn construct(_: &Vm, args: &mut Args) -> SourceResult<Content> {
        Ok(Self(args.expect("body")?).pack())
    }

    fn field(&self, name: &str) -> Option<Value> {
        match name {
            "body" => Some(Value::Content(self.0.clone())),
            _ => None,
        }
    }
}

impl Show for StrongNode {
    fn show(&self, _: &mut Vt, _: &Content, styles: StyleChain) -> SourceResult<Content> {
        Ok(self.0.clone().styled(TextNode::DELTA, Delta(styles.get(Self::DELTA))))
    }
}

/// A delta that is summed up when folded.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Delta(pub i64);

castable! {
    Delta,
    v: i64 => Self(v),
}

impl Fold for Delta {
    type Output = i64;

    fn fold(self, outer: Self::Output) -> Self::Output {
        outer + self.0
    }
}

/// # Emphasis
/// Emphasizes content by setting it in italics.
///
/// - If the current [text](@text) style is `{"normal"}`,
///   this turns it into `{"italic"}`.
/// - If it is already `{"italic"}` or `{"oblique"}`,
///   it turns it back to `{"normal"}`.
///
/// ## Example
/// ```
/// This is _emphasized._ \
/// This is #emph[too.]
///
/// #show emph: it => {
///   text(blue, it.body)
/// }
///
/// This is _emphasized_ differently.
/// ```
///
/// ## Syntax
/// This function also has dedicated syntax: To emphasize content, simply
/// enclose it in underscores (`_`). Note that this only works at word
/// boundaries. To emphasize part of a word, you have to use the function.
///
/// ## Parameters
/// - body: Content (positional, required)
///   The content to emphasize.
///
/// ## Category
/// text
#[func]
#[capable(Show)]
#[derive(Debug, Hash)]
pub struct EmphNode(pub Content);

#[node]
impl EmphNode {
    fn construct(_: &Vm, args: &mut Args) -> SourceResult<Content> {
        Ok(Self(args.expect("body")?).pack())
    }

    fn field(&self, name: &str) -> Option<Value> {
        match name {
            "body" => Some(Value::Content(self.0.clone())),
            _ => None,
        }
    }
}

impl Show for EmphNode {
    fn show(&self, _: &mut Vt, _: &Content, _: StyleChain) -> SourceResult<Content> {
        Ok(self.0.clone().styled(TextNode::EMPH, Toggle))
    }
}

/// A toggle that turns on and off alternatingly if folded.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Toggle;

impl Fold for Toggle {
    type Output = bool;

    fn fold(self, outer: Self::Output) -> Self::Output {
        !outer
    }
}

/// # Lowercase
/// Convert text or content to lowercase.
///
/// ## Example
/// ```
/// #lower("ABC") \
/// #lower[*My Text*] \
/// #lower[already low]
/// ```
///
/// ## Parameters
/// - text: ToCase (positional, required)
///   The text to convert to lowercase.
///
/// ## Category
/// text
#[func]
pub fn lower(args: &mut Args) -> SourceResult<Value> {
    case(Case::Lower, args)
}

/// # Uppercase
/// Convert text or content to uppercase.
///
/// ## Example
/// ```
/// #upper("abc") \
/// #upper[*my text*] \
/// #upper[ALREADY HIGH]
/// ```
///
/// ## Parameters
/// - text: ToCase (positional, required)
///   The text to convert to uppercase.
///
/// ## Category
/// text
#[func]
pub fn upper(args: &mut Args) -> SourceResult<Value> {
    case(Case::Upper, args)
}

/// Change the case of text.
fn case(case: Case, args: &mut Args) -> SourceResult<Value> {
    let Spanned { v, span } = args.expect("string or content")?;
    Ok(match v {
        Value::Str(v) => Value::Str(case.apply(&v).into()),
        Value::Content(v) => Value::Content(v.styled(TextNode::CASE, Some(case))),
        v => bail!(span, "expected string or content, found {}", v.type_name()),
    })
}

/// A value whose case can be changed.
struct ToCase;

castable! {
    ToCase,
    _: Str => Self,
    _: Content => Self,
}

/// A case transformation on text.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Case {
    /// Everything is lowercased.
    Lower,
    /// Everything is uppercased.
    Upper,
}

impl Case {
    /// Apply the case to a string.
    pub fn apply(self, text: &str) -> String {
        match self {
            Self::Lower => text.to_lowercase(),
            Self::Upper => text.to_uppercase(),
        }
    }
}

/// # Small Capitals
/// Display text in small capitals.
///
/// _Note:_ This enables the OpenType `smcp` feature for the font. Not all fonts
/// support this feature (including Typst's current default font,
/// unfortunately). Sometimes smallcaps are part of a dedicated font and
/// sometimes they are not available at all. In the future, this function will
/// support selecting a dedicated smallcaps font as well as synthesizing
/// smallcaps from normal letters, but this is not yet implemented.
///
/// ## Example
/// ```
/// #set par(justify: true)
/// #set text(family: "Noto Serif")
/// #set heading(numbering: "I.")
///
/// #show heading: it => {
///   set block(below: 10pt)
///   set text(weight: "regular")
///   align(center, smallcaps(it))
/// }
///
/// = Introduction
/// #lorem(40)
/// ```
///
/// ## Parameters
/// - text: Content (positional, required)
///   The text to display to small capitals.
///
/// ## Category
/// text
#[func]
pub fn smallcaps(args: &mut Args) -> SourceResult<Value> {
    let body: Content = args.expect("content")?;
    Ok(Value::Content(body.styled(TextNode::SMALLCAPS, true)))
}