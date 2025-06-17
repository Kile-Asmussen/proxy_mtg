#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tag {
    pub name: &'static str,
    pub void: bool,
    pub literal: Option<(&'static str, &'static str)>,
}

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
impl Tag {
    pub const a: Tag = Tag {
        name: "a",
        void: false,
        literal: None,
    };
    pub const abbr: Tag = Tag {
        name: "abbr",
        void: false,
        literal: None,
    };
    pub const address: Tag = Tag {
        name: "address",
        void: false,
        literal: None,
    };
    pub const area: Tag = Tag {
        name: "area",
        void: true,
        literal: None,
    };
    pub const article: Tag = Tag {
        name: "article",
        void: false,
        literal: None,
    };
    pub const aside: Tag = Tag {
        name: "aside",
        void: false,
        literal: None,
    };
    pub const audio: Tag = Tag {
        name: "audio",
        void: false,
        literal: None,
    };
    pub const b: Tag = Tag {
        name: "b",
        void: false,
        literal: None,
    };
    pub const base: Tag = Tag {
        name: "base",
        void: true,
        literal: None,
    };
    pub const bdi: Tag = Tag {
        name: "bdi",
        void: false,
        literal: None,
    };
    pub const bdo: Tag = Tag {
        name: "bdo",
        void: false,
        literal: None,
    };
    pub const blockquote: Tag = Tag {
        name: "blockquote",
        void: false,
        literal: None,
    };
    pub const body: Tag = Tag {
        name: "body",
        void: false,
        literal: None,
    };
    pub const br: Tag = Tag {
        name: "br",
        void: true,
        literal: None,
    };
    pub const button: Tag = Tag {
        name: "button",
        void: false,
        literal: None,
    };
    pub const canvas: Tag = Tag {
        name: "canvas",
        void: false,
        literal: None,
    };
    pub const caption: Tag = Tag {
        name: "caption",
        void: false,
        literal: None,
    };
    pub const cite: Tag = Tag {
        name: "cite",
        void: false,
        literal: None,
    };
    pub const code: Tag = Tag {
        name: "code",
        void: false,
        literal: None,
    };
    pub const col: Tag = Tag {
        name: "col",
        void: true,
        literal: None,
    };
    pub const colgroup: Tag = Tag {
        name: "colgroup",
        void: false,
        literal: None,
    };
    pub const data: Tag = Tag {
        name: "data",
        void: false,
        literal: None,
    };
    pub const datalist: Tag = Tag {
        name: "datalist",
        void: false,
        literal: None,
    };
    pub const dd: Tag = Tag {
        name: "dd",
        void: false,
        literal: None,
    };
    pub const del: Tag = Tag {
        name: "del",
        void: false,
        literal: None,
    };
    pub const details: Tag = Tag {
        name: "details",
        void: false,
        literal: None,
    };
    pub const dfn: Tag = Tag {
        name: "dfn",
        void: false,
        literal: None,
    };
    pub const dialog: Tag = Tag {
        name: "dialog",
        void: false,
        literal: None,
    };
    pub const div: Tag = Tag {
        name: "div",
        void: false,
        literal: None,
    };
    pub const dl: Tag = Tag {
        name: "dl",
        void: false,
        literal: None,
    };
    pub const dt: Tag = Tag {
        name: "dt",
        void: false,
        literal: None,
    };
    pub const em: Tag = Tag {
        name: "em",
        void: false,
        literal: None,
    };
    pub const embed: Tag = Tag {
        name: "embed",
        void: true,
        literal: None,
    };
    pub const fieldset: Tag = Tag {
        name: "fieldset",
        void: false,
        literal: None,
    };
    pub const figcaption: Tag = Tag {
        name: "figcaption",
        void: false,
        literal: None,
    };
    pub const figure: Tag = Tag {
        name: "figure",
        void: false,
        literal: None,
    };
    pub const footer: Tag = Tag {
        name: "footer",
        void: false,
        literal: None,
    };
    pub const form: Tag = Tag {
        name: "form",
        void: false,
        literal: None,
    };
    pub const h1: Tag = Tag {
        name: "h1",
        void: false,
        literal: None,
    };
    pub const head: Tag = Tag {
        name: "head",
        void: false,
        literal: None,
    };
    pub const header: Tag = Tag {
        name: "header",
        void: false,
        literal: None,
    };
    pub const hgroup: Tag = Tag {
        name: "hgroup",
        void: false,
        literal: None,
    };
    pub const hr: Tag = Tag {
        name: "hr",
        void: true,
        literal: None,
    };
    pub const html: Tag = Tag {
        name: "html",
        void: false,
        literal: None,
    };
    pub const i: Tag = Tag {
        name: "i",
        void: false,
        literal: None,
    };
    pub const iframe: Tag = Tag {
        name: "iframe",
        void: false,
        literal: None,
    };
    pub const img: Tag = Tag {
        name: "img",
        void: true,
        literal: None,
    };
    pub const input: Tag = Tag {
        name: "input",
        void: true,
        literal: None,
    };
    pub const ins: Tag = Tag {
        name: "ins",
        void: false,
        literal: None,
    };
    pub const kbd: Tag = Tag {
        name: "kbd",
        void: false,
        literal: None,
    };
    pub const label: Tag = Tag {
        name: "label",
        void: false,
        literal: None,
    };
    pub const legend: Tag = Tag {
        name: "legend",
        void: false,
        literal: None,
    };
    pub const li: Tag = Tag {
        name: "li",
        void: false,
        literal: None,
    };
    pub const link: Tag = Tag {
        name: "link",
        void: true,
        literal: None,
    };
    pub const main: Tag = Tag {
        name: "main",
        void: false,
        literal: None,
    };
    pub const map: Tag = Tag {
        name: "map",
        void: false,
        literal: None,
    };
    pub const mark: Tag = Tag {
        name: "mark",
        void: false,
        literal: None,
    };
    pub const menu: Tag = Tag {
        name: "menu",
        void: false,
        literal: None,
    };
    pub const meta: Tag = Tag {
        name: "meta",
        void: true,
        literal: None,
    };
    pub const meter: Tag = Tag {
        name: "meter",
        void: false,
        literal: None,
    };
    pub const nav: Tag = Tag {
        name: "nav",
        void: false,
        literal: None,
    };
    pub const noscript: Tag = Tag {
        name: "noscript",
        void: false,
        literal: None,
    };
    pub const object: Tag = Tag {
        name: "object",
        void: false,
        literal: None,
    };
    pub const ol: Tag = Tag {
        name: "ol",
        void: false,
        literal: None,
    };
    pub const optgroup: Tag = Tag {
        name: "optgroup",
        void: false,
        literal: None,
    };
    pub const option: Tag = Tag {
        name: "option",
        void: false,
        literal: None,
    };
    pub const output: Tag = Tag {
        name: "output",
        void: false,
        literal: None,
    };
    pub const p: Tag = Tag {
        name: "p",
        void: false,
        literal: None,
    };
    pub const picture: Tag = Tag {
        name: "picture",
        void: false,
        literal: None,
    };
    pub const pre: Tag = Tag {
        name: "pre",
        void: false,
        literal: None,
    };
    pub const progress: Tag = Tag {
        name: "progress",
        void: false,
        literal: None,
    };
    pub const q: Tag = Tag {
        name: "q",
        void: false,
        literal: None,
    };
    pub const rp: Tag = Tag {
        name: "rp",
        void: false,
        literal: None,
    };
    pub const rt: Tag = Tag {
        name: "rt",
        void: false,
        literal: None,
    };
    pub const ruby: Tag = Tag {
        name: "ruby",
        void: false,
        literal: None,
    };
    pub const s: Tag = Tag {
        name: "s",
        void: false,
        literal: None,
    };
    pub const samp: Tag = Tag {
        name: "samp",
        void: false,
        literal: None,
    };
    pub const script: Tag = Tag {
        name: "script",
        void: false,
        literal: Some(("/*<![CDATA[*/", "/*]]>*/")),
    };
    pub const search: Tag = Tag {
        name: "search",
        void: false,
        literal: None,
    };
    pub const section: Tag = Tag {
        name: "section",
        void: false,
        literal: None,
    };
    pub const select: Tag = Tag {
        name: "select",
        void: false,
        literal: None,
    };
    pub const small: Tag = Tag {
        name: "small",
        void: false,
        literal: None,
    };
    pub const source: Tag = Tag {
        name: "source",
        void: true,
        literal: None,
    };
    pub const span: Tag = Tag {
        name: "span",
        void: false,
        literal: None,
    };
    pub const strong: Tag = Tag {
        name: "strong",
        void: false,
        literal: None,
    };
    pub const style: Tag = Tag {
        name: "style",
        void: false,
        literal: Some(("/*<![CDATA[*/", "/*]]>*/")),
    };
    pub const sub: Tag = Tag {
        name: "sub",
        void: false,
        literal: None,
    };
    pub const summary: Tag = Tag {
        name: "summary",
        void: false,
        literal: None,
    };
    pub const sup: Tag = Tag {
        name: "sup",
        void: false,
        literal: None,
    };
    pub const svg: Tag = Tag {
        name: "svg",
        void: false,
        literal: None,
    };
    pub const table: Tag = Tag {
        name: "table",
        void: false,
        literal: None,
    };
    pub const tbody: Tag = Tag {
        name: "tbody",
        void: false,
        literal: None,
    };
    pub const td: Tag = Tag {
        name: "td",
        void: false,
        literal: None,
    };
    pub const template: Tag = Tag {
        name: "template",
        void: false,
        literal: None,
    };
    pub const textarea: Tag = Tag {
        name: "textarea",
        void: false,
        literal: None,
    };
    pub const tfoot: Tag = Tag {
        name: "tfoot",
        void: false,
        literal: None,
    };
    pub const th: Tag = Tag {
        name: "th",
        void: false,
        literal: None,
    };
    pub const thead: Tag = Tag {
        name: "thead",
        void: false,
        literal: None,
    };
    pub const time: Tag = Tag {
        name: "time",
        void: false,
        literal: None,
    };
    pub const title: Tag = Tag {
        name: "title",
        void: false,
        literal: Some(("", "")),
    };
    pub const tr: Tag = Tag {
        name: "tr",
        void: false,
        literal: None,
    };
    pub const track: Tag = Tag {
        name: "track",
        void: true,
        literal: None,
    };
    pub const u: Tag = Tag {
        name: "u",
        void: false,
        literal: None,
    };
    pub const ul: Tag = Tag {
        name: "ul",
        void: false,
        literal: None,
    };
    pub const var: Tag = Tag {
        name: "var",
        void: false,
        literal: None,
    };
    pub const video: Tag = Tag {
        name: "video",
        void: false,
        literal: None,
    };
    pub const wbr: Tag = Tag {
        name: "wbr",
        void: true,
        literal: None,
    };
}
