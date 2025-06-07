#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Tag {
    pub name: &'static str,
    pub void: bool,
}

#[allow(non_upper_case_globals)]
impl Tag {
    pub fn len(&self) -> usize {
        self.name.len() + 2 + if self.void { 0 } else { self.name.len() + 3 }
    }

    pub const a: Tag = Tag {
        name: "a",
        void: false,
    };
    pub const abbr: Tag = Tag {
        name: "abbr",
        void: false,
    };
    pub const address: Tag = Tag {
        name: "address",
        void: false,
    };
    pub const area: Tag = Tag {
        name: "area",
        void: true,
    };
    pub const article: Tag = Tag {
        name: "article",
        void: false,
    };
    pub const aside: Tag = Tag {
        name: "aside",
        void: false,
    };
    pub const audio: Tag = Tag {
        name: "audio",
        void: false,
    };
    pub const b: Tag = Tag {
        name: "b",
        void: false,
    };
    pub const base: Tag = Tag {
        name: "base",
        void: true,
    };
    pub const bdi: Tag = Tag {
        name: "bdi",
        void: false,
    };
    pub const bdo: Tag = Tag {
        name: "bdo",
        void: false,
    };
    pub const blockquote: Tag = Tag {
        name: "blockquote",
        void: false,
    };
    pub const body: Tag = Tag {
        name: "body",
        void: false,
    };
    pub const br: Tag = Tag {
        name: "br",
        void: true,
    };
    pub const button: Tag = Tag {
        name: "button",
        void: false,
    };
    pub const canvas: Tag = Tag {
        name: "canvas",
        void: false,
    };
    pub const caption: Tag = Tag {
        name: "caption",
        void: false,
    };
    pub const cite: Tag = Tag {
        name: "cite",
        void: false,
    };
    pub const code: Tag = Tag {
        name: "code",
        void: false,
    };
    pub const col: Tag = Tag {
        name: "col",
        void: true,
    };
    pub const colgroup: Tag = Tag {
        name: "colgroup",
        void: false,
    };
    pub const data: Tag = Tag {
        name: "data",
        void: false,
    };
    pub const datalist: Tag = Tag {
        name: "datalist",
        void: false,
    };
    pub const dd: Tag = Tag {
        name: "dd",
        void: false,
    };
    pub const del: Tag = Tag {
        name: "del",
        void: false,
    };
    pub const details: Tag = Tag {
        name: "details",
        void: false,
    };
    pub const dfn: Tag = Tag {
        name: "dfn",
        void: false,
    };
    pub const dialog: Tag = Tag {
        name: "dialog",
        void: false,
    };
    pub const div: Tag = Tag {
        name: "div",
        void: false,
    };
    pub const dl: Tag = Tag {
        name: "dl",
        void: false,
    };
    pub const dt: Tag = Tag {
        name: "dt",
        void: false,
    };
    pub const em: Tag = Tag {
        name: "em",
        void: false,
    };
    pub const embed: Tag = Tag {
        name: "embed",
        void: true,
    };
    pub const fieldset: Tag = Tag {
        name: "fieldset",
        void: false,
    };
    pub const figcaption: Tag = Tag {
        name: "figcaption",
        void: false,
    };
    pub const figure: Tag = Tag {
        name: "figure",
        void: false,
    };
    pub const footer: Tag = Tag {
        name: "footer",
        void: false,
    };
    pub const form: Tag = Tag {
        name: "form",
        void: false,
    };
    pub const h1: Tag = Tag {
        name: "h1",
        void: false,
    };
    pub const head: Tag = Tag {
        name: "head",
        void: false,
    };
    pub const header: Tag = Tag {
        name: "header",
        void: false,
    };
    pub const hgroup: Tag = Tag {
        name: "hgroup",
        void: false,
    };
    pub const hr: Tag = Tag {
        name: "hr",
        void: true,
    };
    pub const html: Tag = Tag {
        name: "html",
        void: false,
    };
    pub const i: Tag = Tag {
        name: "i",
        void: false,
    };
    pub const iframe: Tag = Tag {
        name: "iframe",
        void: false,
    };
    pub const img: Tag = Tag {
        name: "img",
        void: true,
    };
    pub const input: Tag = Tag {
        name: "input",
        void: true,
    };
    pub const ins: Tag = Tag {
        name: "ins",
        void: false,
    };
    pub const kbd: Tag = Tag {
        name: "kbd",
        void: false,
    };
    pub const label: Tag = Tag {
        name: "label",
        void: false,
    };
    pub const legend: Tag = Tag {
        name: "legend",
        void: false,
    };
    pub const li: Tag = Tag {
        name: "li",
        void: false,
    };
    pub const link: Tag = Tag {
        name: "link",
        void: true,
    };
    pub const main: Tag = Tag {
        name: "main",
        void: false,
    };
    pub const map: Tag = Tag {
        name: "map",
        void: false,
    };
    pub const mark: Tag = Tag {
        name: "mark",
        void: false,
    };
    pub const menu: Tag = Tag {
        name: "menu",
        void: false,
    };
    pub const meta: Tag = Tag {
        name: "meta",
        void: true,
    };
    pub const meter: Tag = Tag {
        name: "meter",
        void: false,
    };
    pub const nav: Tag = Tag {
        name: "nav",
        void: false,
    };
    pub const noscript: Tag = Tag {
        name: "noscript",
        void: false,
    };
    pub const object: Tag = Tag {
        name: "object",
        void: false,
    };
    pub const ol: Tag = Tag {
        name: "ol",
        void: false,
    };
    pub const optgroup: Tag = Tag {
        name: "optgroup",
        void: false,
    };
    pub const option: Tag = Tag {
        name: "option",
        void: false,
    };
    pub const output: Tag = Tag {
        name: "output",
        void: false,
    };
    pub const p: Tag = Tag {
        name: "p",
        void: false,
    };
    pub const param: Tag = Tag {
        name: "param",
        void: true,
    };
    pub const picture: Tag = Tag {
        name: "picture",
        void: false,
    };
    pub const pre: Tag = Tag {
        name: "pre",
        void: false,
    };
    pub const progress: Tag = Tag {
        name: "progress",
        void: false,
    };
    pub const q: Tag = Tag {
        name: "q",
        void: false,
    };
    pub const rp: Tag = Tag {
        name: "rp",
        void: false,
    };
    pub const rt: Tag = Tag {
        name: "rt",
        void: false,
    };
    pub const ruby: Tag = Tag {
        name: "ruby",
        void: false,
    };
    pub const s: Tag = Tag {
        name: "s",
        void: false,
    };
    pub const samp: Tag = Tag {
        name: "samp",
        void: false,
    };
    pub const script: Tag = Tag {
        name: "script",
        void: false,
    };
    pub const search: Tag = Tag {
        name: "search",
        void: false,
    };
    pub const section: Tag = Tag {
        name: "section",
        void: false,
    };
    pub const select: Tag = Tag {
        name: "select",
        void: false,
    };
    pub const small: Tag = Tag {
        name: "small",
        void: false,
    };
    pub const source: Tag = Tag {
        name: "source",
        void: true,
    };
    pub const span: Tag = Tag {
        name: "span",
        void: false,
    };
    pub const strong: Tag = Tag {
        name: "strong",
        void: false,
    };
    pub const style: Tag = Tag {
        name: "style",
        void: false,
    };
    pub const sub: Tag = Tag {
        name: "sub",
        void: false,
    };
    pub const summary: Tag = Tag {
        name: "summary",
        void: false,
    };
    pub const sup: Tag = Tag {
        name: "sup",
        void: false,
    };
    pub const svg: Tag = Tag {
        name: "svg",
        void: false,
    };
    pub const table: Tag = Tag {
        name: "table",
        void: false,
    };
    pub const tbody: Tag = Tag {
        name: "tbody",
        void: false,
    };
    pub const td: Tag = Tag {
        name: "td",
        void: false,
    };
    pub const template: Tag = Tag {
        name: "template",
        void: false,
    };
    pub const textarea: Tag = Tag {
        name: "textarea",
        void: false,
    };
    pub const tfoot: Tag = Tag {
        name: "tfoot",
        void: false,
    };
    pub const th: Tag = Tag {
        name: "th",
        void: false,
    };
    pub const thead: Tag = Tag {
        name: "thead",
        void: false,
    };
    pub const time: Tag = Tag {
        name: "time",
        void: false,
    };
    pub const title: Tag = Tag {
        name: "title",
        void: false,
    };
    pub const tr: Tag = Tag {
        name: "tr",
        void: false,
    };
    pub const track: Tag = Tag {
        name: "track",
        void: true,
    };
    pub const u: Tag = Tag {
        name: "u",
        void: false,
    };
    pub const ul: Tag = Tag {
        name: "ul",
        void: false,
    };
    pub const var: Tag = Tag {
        name: "var",
        void: false,
    };
    pub const video: Tag = Tag {
        name: "video",
        void: false,
    };
    pub const wbr: Tag = Tag {
        name: "wbr",
        void: true,
    };
}
