use crate::ast::*;
use crate::printer::ToDoc;
use pretty::{Arena, DocAllocator, DocBuilder};

pub(crate) fn blockquote_to_doc<'a>(
    config: crate::Xrc<crate::printer::config::Config>,
    arena: &'a Arena<'a>,
    inner: &[Block],
) -> DocBuilder<'a, Arena<'a>, ()> {
    let blocks = inner.to_owned();
    arena.column(move |current_column| {
        let prefix = "> ";
        let tmp_arena = Arena::new();
        let doc = blocks.to_doc(config.clone(), &tmp_arena);

        let mut buf = Vec::new();
        doc.render(config.width - current_column - prefix.len(), &mut buf)
            .unwrap();
        let text = String::from_utf8(buf).unwrap();

        let lines = text.lines().map(|d| {
            arena
                .as_string(prefix.to_string())
                .append(arena.as_string(d))
        });

        arena.intersperse(lines, arena.hardline()).into_doc()
    })
}
