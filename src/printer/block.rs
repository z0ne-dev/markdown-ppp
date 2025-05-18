use crate::ast::*;
use crate::printer::{inline::ToDocInline, ToDoc};
use pretty::{Arena, DocAllocator, DocBuilder};

impl<'a> ToDoc<'a> for Vec<Block> {
    fn to_doc(
        &self,
        config: crate::Xrc<crate::printer::config::Config>,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        let mut acc = arena.nil();
        for (i, block) in self.iter().enumerate() {
            if i > 0 {
                // между блоками — пустая строка
                acc = acc.append(arena.hardline()).append(arena.hardline());
            }
            acc = acc.append(block.to_doc(config.clone(), arena));
        }
        acc
    }
}

impl<'a> ToDoc<'a> for Vec<&Block> {
    fn to_doc(
        &self,
        config: crate::Xrc<crate::printer::config::Config>,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        let mut acc = arena.nil();
        for (i, block) in self.iter().enumerate() {
            if i > 0 {
                // между блоками — пустая строка
                acc = acc.append(arena.hardline()).append(arena.hardline());
            }
            acc = acc.append(block.to_doc(config.clone(), arena));
        }
        acc
    }
}

/// Block-level узлы
impl<'a> ToDoc<'a> for Block {
    fn to_doc(
        &self,
        config: crate::Xrc<crate::printer::config::Config>,
        arena: &'a Arena<'a>,
    ) -> DocBuilder<'a, Arena<'a>, ()> {
        match self {
            Block::Paragraph(inlines) => inlines.to_doc_inline(true, arena),
            Block::Heading(v) => v.to_doc(config, arena),
            Block::ThematicBreak => arena.text("---"),
            Block::BlockQuote(inner) => {
                crate::printer::blockquote::blockquote_to_doc(config, arena, inner)
            }
            Block::List(v) => v.to_doc(config, arena),
            Block::CodeBlock(CodeBlock { kind, literal }) => {
                match kind {
                    CodeBlockKind::Fenced { info } => {
                        let info = info.as_deref().unwrap_or("");
                        arena
                            .text(format!("```{}\n", info))
                            .append(arena.text(literal.clone()))
                            .append(arena.text("\n```"))
                    }
                    CodeBlockKind::Indented => {
                        // каждый строка с отступом 4 пробела
                        let indented = literal
                            .lines()
                            .map(|l| format!("    {}", l))
                            .collect::<Vec<_>>()
                            .join("\n");
                        arena.text(indented)
                    }
                }
            }
            Block::HtmlBlock(html) => arena.text(html.clone()),
            Block::Definition(def) => arena
                .text("[")
                .append(def.label.to_doc_inline(true, arena))
                .append(arena.text("]: "))
                .append(arena.text(format!(
                    "{}{}",
                    def.destination,
                    def.title
                        .as_ref()
                        .map(|t| format!(" \"{}\"", t))
                        .unwrap_or_default()
                ))),

            Block::Empty => arena.nil(),
            Block::Table(v) => v.to_doc(config, arena),
            Block::FootnoteDefinition(def) => arena
                .text(format!("[^{}]: ", def.label))
                .append(def.blocks.to_doc(config, arena)),
        }
    }
}
