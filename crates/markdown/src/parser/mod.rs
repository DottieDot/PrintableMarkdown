mod markdown;

use peg::parser;

pub use markdown::*;

parser! {
  pub grammar markdown_parser() for str {
    pub rule markdown_document() -> Vec<Markdown>
      = (eol()*) elements:(markdown_element() ** (eol()+)) (eol()*) eof() {
        elements
      }

    rule markdown_element() -> Markdown
      = horizontal_divider()
      / header()
      / code_block()
      / paragraph()

    rule header() -> Markdown
      = n:$(['#']*<1,6>) " " text:span(<>) {
        Markdown::Heading {
          content: text,
          level: n.len() as i32
        }
      }

    rule paragraph() -> Markdown
      = lines:(span(<>) ++ eol()) {
        Markdown::Paragraph {
          content: lines
        }
      }

    rule horizontal_divider() -> Markdown
      = "---" {
        Markdown::HorizontalDivider()
      }

    rule code_block() -> Markdown
      = "```" lang:$(['a'..='z' | 'A'..='Z']+(&eol()))? eol()? code:$([_] ** (!(eol()* "```"))) eol()* "```" {
        Markdown::CodeBlock {
          content: code.to_owned(),
          language: lang.map(|s| s.to_owned())
        }
      }

    rule bold() -> InlineMarkdown
      = "**" s:span(<!"**">) "**" {
        InlineMarkdown::Bold(Box::new(s))
      }

    rule italic() -> InlineMarkdown
      = "*" s:span(<(!("*"))>) "*" {
        InlineMarkdown::Italic(Box::new(s))
      }

    rule underline() -> InlineMarkdown
      = "__" s:span(<!"__">) "__" {
        InlineMarkdown::Underline(Box::new(s))
      }
    
    rule strikethrough() -> InlineMarkdown
      = "~~" s:span(<!"~~">) "~~" {
        InlineMarkdown::Strikethrough(Box::new(s))
      }

    rule raw<T>(exclude: rule<T>) -> InlineMarkdown
      = exclude() s:$([^'\n'] ++ (exclude() !span_text())) {
        InlineMarkdown::Raw(s.to_owned())
      }

    rule span<T>(exclude: rule<T>) -> InlineMarkdown
      = contents:((span_text() / raw(&exclude))+) {
        InlineMarkdown::Span(contents)
      }

    rule span_text() -> InlineMarkdown
      = bold()
      / italic()
      / underline()
      / strikethrough()

    rule single_line() -> String
      = quiet!{ s:$([^'\n']+) {
        s.to_owned()
      }}
      / expected!("single line")

    rule eof() = ![_]

    rule eol() = "\n"
  }
}
