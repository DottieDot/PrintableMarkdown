
#[derive(Debug)]
pub enum InlineMarkdown {
  Bold(Box<InlineMarkdown>),
  Italic(Box<InlineMarkdown>),
  Underline(Box<InlineMarkdown>),
  Strikethrough(Box<InlineMarkdown>),
  Span(Vec<InlineMarkdown>),
  Raw(String)
}

#[derive(Debug)]
pub enum Markdown {
  Heading {
    content: InlineMarkdown,
    level: i32
  },
  Paragraph {
    content: Vec<InlineMarkdown>
  },
  HorizontalDivider(),
  CodeBlock {
    content: String,
    language: Option<String>
  }
}
