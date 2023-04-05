use strum::{AsRefStr, EnumString};

#[derive(Copy, Clone, Debug, EnumString, Eq, PartialEq, Hash, AsRefStr)]
pub enum XrefKind {
    #[strum(serialize = "accelerators")]
    Accelerators,
    #[strum(serialize = "accessor")]
    Accessor,
    #[strum(serialize = "activeBindingFunc")]
    ActiveBindingFunc,
    #[strum(serialize = "alias")]
    Alias,
    #[strum(serialize = "altstep")]
    Altstep,
    #[strum(serialize = "anchor")]
    Anchor,
    #[strum(serialize = "annotation")]
    Annotation,
    #[strum(serialize = "anonMember")]
    AnonMember,
    #[strum(serialize = "antfile")]
    Antfile,
    #[strum(serialize = "architecture")]
    Architecture,
    #[strum(serialize = "arg")]
    Arg,
    #[strum(serialize = "array")]
    Array,
    #[strum(serialize = "article")]
    Article,
    #[strum(serialize = "artifactId")]
    ArtifactId,
    #[strum(serialize = "assembly")]
    Assembly,
    #[strum(serialize = "assert")]
    Assert,
    #[strum(serialize = "attribute")]
    Attribute,
    #[strum(serialize = "augroup")]
    Augroup,
    #[strum(serialize = "benchmark")]
    Benchmark,
    #[strum(serialize = "bibitem")]
    Bibitem,
    #[strum(serialize = "bitmap")]
    Bitmap,
    #[strum(serialize = "block")]
    Block,
    #[strum(serialize = "blockData")]
    BlockData,
    #[strum(serialize = "book")]
    Book,
    #[strum(serialize = "booklet")]
    Booklet,
    #[strum(serialize = "boolean")]
    Boolean,
    #[strum(serialize = "build")]
    Build,
    #[strum(serialize = "callback")]
    Callback,
    #[strum(serialize = "category")]
    Category,
    #[strum(serialize = "ccflag")]
    Ccflag,
    #[strum(serialize = "cell")]
    Cell,
    #[strum(serialize = "chapter")]
    Chapter,
    #[strum(serialize = "checker")]
    Checker,
    #[strum(serialize = "choice")]
    Choice,
    #[strum(serialize = "chunklabel")]
    Chunklabel,
    #[strum(serialize = "citation")]
    Citation,
    #[strum(serialize = "class")]
    Class,
    #[strum(serialize = "clocking")]
    Clocking,
    #[strum(serialize = "combo")]
    Combo,
    #[strum(serialize = "command")]
    Command,
    #[strum(serialize = "common")]
    Common,
    #[strum(serialize = "component")]
    Component,
    #[strum(serialize = "cond")]
    Cond,
    #[strum(serialize = "condition")]
    Condition,
    #[strum(serialize = "conference")]
    Conference,
    #[strum(serialize = "config")]
    Config,
    #[strum(serialize = "const")]
    Const,
    #[strum(serialize = "constant")]
    Constant,
    #[strum(serialize = "constraint")]
    Constraint,
    #[strum(serialize = "constructor", serialize = "Constructor")]
    Constructor,
    #[strum(serialize = "context")]
    Context,
    #[strum(serialize = "counter")]
    Counter,
    #[strum(serialize = "covergroup")]
    Covergroup,
    #[strum(serialize = "cursor")]
    Cursor,
    #[strum(serialize = "custom")]
    Custom,
    #[strum(serialize = "data")]
    Data,
    #[strum(serialize = "database")]
    Database,
    #[strum(serialize = "dataframe")]
    Dataframe,
    #[strum(serialize = "def")]
    Def,
    #[strum(serialize = "define")]
    Define,
    #[strum(serialize = "definition")]
    Definition,
    #[strum(serialize = "delegate")]
    Delegate,
    #[strum(serialize = "deletedFile")]
    DeletedFile,
    #[strum(serialize = "derivedMode")]
    DerivedMode,
    #[strum(serialize = "describe")]
    Describe,
    #[strum(serialize = "dialog")]
    Dialog,
    #[strum(serialize = "directory")]
    Directory,
    #[strum(serialize = "division")]
    Division,
    #[strum(serialize = "domain")]
    Domain,
    #[strum(serialize = "edesc")]
    Edesc,
    #[strum(serialize = "element")]
    Element,
    #[strum(serialize = "entity")]
    Entity,
    #[strum(serialize = "entry")]
    Entry,
    #[strum(serialize = "enum")]
    Enum,
    #[strum(serialize = "enumConstant")]
    EnumConstant,
    #[strum(serialize = "enumerator")]
    Enumerator,
    #[strum(serialize = "environment")]
    Environment,
    #[strum(serialize = "error")]
    Error,
    #[strum(serialize = "event")]
    Event,
    #[strum(serialize = "exception", serialize = "Exception")]
    Exception,
    #[strum(serialize = "face")]
    Face,
    #[strum(serialize = "fd")]
    Fd,
    #[strum(serialize = "feature")]
    Feature,
    #[strum(serialize = "field")]
    Field,
    #[strum(serialize = "File")]
    File,
    #[strum(serialize = "filename")]
    Filename,
    #[strum(serialize = "filter")]
    Filter,
    #[strum(serialize = "font")]
    Font,
    #[strum(serialize = "footnote")]
    Footnote,
    #[strum(serialize = "formal")]
    Formal,
    #[strum(serialize = "format")]
    Format,
    #[strum(serialize = "fragment")]
    Fragment,
    #[strum(serialize = "framesubtitle")]
    Framesubtitle,
    #[strum(serialize = "frametitle")]
    Frametitle,
    #[strum(serialize = "fun")]
    Fun,
    #[strum(serialize = "func")]
    Func,
    #[strum(serialize = "function")]
    Function,
    #[strum(serialize = "functionVar")]
    FunctionVar,
    #[strum(serialize = "functor")]
    Functor,
    #[strum(serialize = "gem")]
    Gem,
    #[strum(serialize = "generator")]
    Generator,
    #[strum(serialize = "generic")]
    Generic,
    #[strum(serialize = "getter")]
    Getter,
    #[strum(serialize = "global")]
    Global,
    #[strum(serialize = "globalVar")]
    GlobalVar,
    #[strum(serialize = "grammar")]
    Grammar,
    #[strum(serialize = "group")]
    Group,
    #[strum(serialize = "groupId")]
    GroupId,
    #[strum(serialize = "guard")]
    Guard,
    #[strum(serialize = "handler")]
    Handler,
    #[strum(serialize = "header")]
    Header,
    #[strum(serialize = "heading1")]
    Heading1,
    #[strum(serialize = "heading2")]
    Heading2,
    #[strum(serialize = "heading3")]
    Heading3,
    #[strum(serialize = "heredoc")]
    Heredoc,
    #[strum(serialize = "hfunc")]
    Hfunc,
    #[strum(serialize = "hunk")]
    Hunk,
    #[strum(serialize = "icon")]
    Icon,
    #[strum(serialize = "id")]
    Id,
    #[strum(serialize = "identifier")]
    Identifier,
    #[strum(serialize = "ifclass")]
    Ifclass,
    #[strum(serialize = "implementation")]
    Implementation,
    #[strum(serialize = "import")]
    Import,
    #[strum(serialize = "inbook")]
    Inbook,
    #[strum(serialize = "incollection")]
    Incollection,
    #[strum(serialize = "index")]
    Index,
    #[strum(serialize = "infoitem")]
    Infoitem,
    #[strum(serialize = "inline")]
    Inline,
    #[strum(serialize = "inproceedings")]
    Inproceedings,
    #[strum(serialize = "inputSection")]
    InputSection,
    #[strum(serialize = "instance")]
    Instance,
    #[strum(serialize = "integer")]
    Integer,
    #[strum(serialize = "interface")]
    Interface,
    #[strum(serialize = "iparam")]
    Iparam,
    #[strum(serialize = "it")]
    It,
    #[strum(serialize = "kconfig")]
    Kconfig,
    #[strum(serialize = "key")]
    Key,
    #[strum(serialize = "keyword")]
    Keyword,
    #[strum(serialize = "kind")]
    Kind,
    #[strum(serialize = "L1Header")]
    L1Header,
    #[strum(serialize = "L2Header")]
    L2Header,
    #[strum(serialize = "L3Header")]
    L3Header,
    #[strum(serialize = "L4Header")]
    L4Header,
    #[strum(serialize = "l4subsection")]
    L4Subsection,
    #[strum(serialize = "L5Header")]
    L5Header,
    #[strum(serialize = "l5subsection")]
    L5Subsection,
    #[strum(serialize = "L6Header")]
    L6Header,
    #[strum(serialize = "label")]
    Label,
    #[strum(serialize = "langdef")]
    Langdef,
    #[strum(serialize = "langstr")]
    Langstr,
    #[strum(serialize = "lfunc")]
    Lfunc,
    #[strum(serialize = "library")]
    Library,
    #[strum(serialize = "list")]
    List,
    #[strum(serialize = "literal")]
    Literal,
    #[strum(serialize = "local")]
    Local,
    #[strum(serialize = "loggerSection")]
    LoggerSection,
    #[strum(serialize = "ltlibrary")]
    Ltlibrary,
    #[strum(serialize = "macro")]
    Macro,
    #[strum(serialize = "macrofile")]
    Macrofile,
    #[strum(serialize = "mainMenu")]
    MainMenu,
    #[strum(serialize = "makefile")]
    Makefile,
    #[strum(serialize = "man")]
    Man,
    #[strum(serialize = "manual")]
    Manual,
    #[strum(serialize = "map")]
    Map,
    #[strum(serialize = "mastersthesis")]
    Mastersthesis,
    #[strum(serialize = "matchedTemplate")]
    MatchedTemplate,
    #[strum(serialize = "member")]
    Member,
    #[strum(serialize = "menu")]
    Menu,
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "method")]
    Method,
    #[strum(serialize = "methodSpec")]
    MethodSpec,
    #[strum(serialize = "minorMode")]
    MinorMode,
    #[strum(serialize = "misc")]
    Misc,
    #[strum(serialize = "mixin")]
    Mixin,
    #[strum(serialize = "mlconn")]
    Mlconn,
    #[strum(serialize = "mlprop")]
    Mlprop,
    #[strum(serialize = "mltable")]
    Mltable,
    #[strum(serialize = "modifiedFile")]
    ModifiedFile,
    #[strum(serialize = "modport")]
    Modport,
    #[strum(serialize = "module")]
    Module,
    #[strum(serialize = "modulepar")]
    Modulepar,
    #[strum(serialize = "multitask")]
    Multitask,
    #[strum(serialize = "mxtag")]
    Mxtag,
    #[strum(serialize = "nameattr")]
    Nameattr,
    #[strum(serialize = "namedPattern")]
    NamedPattern,
    #[strum(serialize = "namedTemplate")]
    NamedTemplate,
    #[strum(serialize = "namelist")]
    Namelist,
    #[strum(serialize = "namespace")]
    Namespace,
    #[strum(serialize = "net")]
    Net,
    #[strum(serialize = "nettype")]
    Nettype,
    #[strum(serialize = "newFile")]
    NewFile,
    #[strum(serialize = "node")]
    Node,
    #[strum(serialize = "notation")]
    Notation,
    #[strum(serialize = "nsprefix")]
    Nsprefix,
    #[strum(serialize = "null")]
    Null,
    #[strum(serialize = "number")]
    Number,
    #[strum(serialize = "object")]
    Object,
    #[strum(serialize = "oneof")]
    Oneof,
    #[strum(serialize = "oparam")]
    Oparam,
    #[strum(serialize = "operator")]
    Operator,
    #[strum(serialize = "optenable")]
    Optenable,
    #[strum(serialize = "option")]
    Option,
    #[strum(serialize = "optwith")]
    Optwith,
    #[strum(serialize = "package")]
    Package,
    #[strum(serialize = "packageName")]
    PackageName,
    #[strum(serialize = "packspec")]
    Packspec,
    #[strum(serialize = "paragraph")]
    Paragraph,
    #[strum(serialize = "param")]
    Param,
    #[strum(serialize = "parameter")]
    Parameter,
    #[strum(serialize = "parameterEntity")]
    ParameterEntity,
    #[strum(serialize = "part")]
    Part,
    #[strum(serialize = "patch")]
    Patch,
    #[strum(serialize = "path")]
    Path,
    #[strum(serialize = "phandler")]
    Phandler,
    #[strum(serialize = "phdthesis")]
    Phdthesis,
    #[strum(serialize = "pkg")]
    Pkg,
    #[strum(serialize = "placeholder")]
    Placeholder,
    #[strum(serialize = "play")]
    Play,
    #[strum(serialize = "port")]
    Port,
    #[strum(serialize = "probe")]
    Probe,
    #[strum(serialize = "procedure")]
    Procedure,
    #[strum(serialize = "proceedings")]
    Proceedings,
    #[strum(serialize = "process")]
    Process,
    #[strum(serialize = "program")]
    Program,
    #[strum(serialize = "project")]
    Project,
    #[strum(serialize = "property")]
    Property,
    #[strum(serialize = "protected")]
    Protected,
    #[strum(serialize = "protectspec")]
    Protectspec,
    #[strum(serialize = "protocol")]
    Protocol,
    #[strum(serialize = "protodef")]
    Protodef,
    #[strum(serialize = "publication")]
    Publication,
    #[strum(serialize = "qmp")]
    Qmp,
    #[strum(serialize = "qualname")]
    Qualname,
    #[strum(serialize = "record")]
    Record,
    #[strum(serialize = "RecordField")]
    RecordField,
    #[strum(serialize = "regex")]
    Regex,
    #[strum(serialize = "region")]
    Region,
    #[strum(serialize = "register")]
    Register,
    #[strum(serialize = "reopen")]
    Reopen,
    #[strum(serialize = "repoid")]
    Repoid,
    #[strum(serialize = "repositoryId")]
    RepositoryId,
    #[strum(serialize = "repr")]
    Repr,
    #[strum(serialize = "resource")]
    Resource,
    #[strum(serialize = "response")]
    Response,
    #[strum(serialize = "role")]
    Role,
    #[strum(serialize = "root")]
    Root,
    #[strum(serialize = "rpc")]
    Rpc,
    #[strum(serialize = "rule")]
    Rule,
    #[strum(serialize = "run")]
    Run,
    #[strum(serialize = "schema")]
    Schema,
    #[strum(serialize = "script")]
    Script,
    #[strum(serialize = "section")]
    Section,
    #[strum(serialize = "sectionGroup")]
    SectionGroup,
    #[strum(serialize = "selector")]
    Selector,
    #[strum(serialize = "sequence")]
    Sequence,
    #[strum(serialize = "server")]
    Server,
    #[strum(serialize = "service")]
    Service,
    #[strum(serialize = "set")]
    Set,
    #[strum(serialize = "setter")]
    Setter,
    #[strum(serialize = "signal")]
    Signal,
    #[strum(serialize = "signature")]
    Signature,
    #[strum(serialize = "singletonMethod")]
    SingletonMethod,
    #[strum(serialize = "slot")]
    Slot,
    #[strum(serialize = "source")]
    Source,
    #[strum(serialize = "sourcefile")]
    Sourcefile,
    #[strum(serialize = "step")]
    Step,
    #[strum(serialize = "string")]
    String,
    #[strum(serialize = "strpool")]
    Strpool,
    #[strum(serialize = "struct")]
    Struct,
    #[strum(serialize = "structure")]
    Structure,
    #[strum(serialize = "stylesheet")]
    Stylesheet,
    #[strum(serialize = "subdir")]
    Subdir,
    #[strum(serialize = "submethod")]
    Submethod,
    #[strum(serialize = "submodule")]
    Submodule,
    #[strum(serialize = "subparagraph")]
    Subparagraph,
    #[strum(serialize = "subprogram")]
    Subprogram,
    #[strum(serialize = "subprogspec")]
    Subprogspec,
    #[strum(serialize = "subroutine")]
    Subroutine,
    #[strum(serialize = "subsection")]
    Subsection,
    #[strum(serialize = "subst")]
    Subst,
    #[strum(serialize = "substdef")]
    Substdef,
    #[strum(serialize = "subsubsection")]
    Subsubsection,
    #[strum(serialize = "subtitle")]
    Subtitle,
    #[strum(serialize = "subtype")]
    Subtype,
    #[strum(serialize = "symbol")]
    Symbol,
    #[strum(serialize = "synonym")]
    Synonym,
    #[strum(serialize = "table")]
    Table,
    #[strum(serialize = "tag")]
    Tag,
    #[strum(serialize = "talias")]
    Talias,
    #[strum(serialize = "target")]
    Target,
    #[strum(serialize = "task")]
    Task,
    #[strum(serialize = "taskspec")]
    Taskspec,
    #[strum(serialize = "techreport")]
    Techreport,
    #[strum(serialize = "template")]
    Template,
    #[strum(serialize = "test")]
    Test,
    #[strum(serialize = "testcase")]
    Testcase,
    #[strum(serialize = "theme")]
    Theme,
    #[strum(serialize = "theorem")]
    Theorem,
    #[strum(serialize = "thriftFile")]
    ThriftFile,
    #[strum(serialize = "timer")]
    Timer,
    #[strum(serialize = "title")]
    Title,
    #[strum(serialize = "token")]
    Token,
    #[strum(serialize = "toplevelVariable")]
    ToplevelVariable,
    #[strum(serialize = "trait")]
    Trait,
    #[strum(serialize = "trigger")]
    Trigger,
    #[strum(serialize = "type")]
    Type,
    #[strum(serialize = "typealias")]
    Typealias,
    #[strum(serialize = "typedef")]
    Typedef,
    #[strum(serialize = "union")]
    Union,
    #[strum(serialize = "unit")]
    Unit,
    #[strum(serialize = "unknown")]
    Unknown,
    #[strum(serialize = "unpublished")]
    Unpublished,
    #[strum(serialize = "username")]
    Username,
    #[strum(serialize = "val")]
    Val,
    #[strum(serialize = "value")]
    Value,
    #[strum(serialize = "var")]
    Var,
    #[strum(serialize = "varalias")]
    Varalias,
    #[strum(serialize = "variable")]
    Variable,
    #[strum(serialize = "vector")]
    Vector,
    #[strum(serialize = "version")]
    Version,
    #[strum(serialize = "view")]
    View,
    #[strum(serialize = "virtual")]
    Virtual,
    #[strum(serialize = "vresource")]
    Vresource,
    #[strum(serialize = "wrapper")]
    Wrapper,
    #[strum(serialize = "xinput")]
    Xinput,
    #[strum(serialize = "xtask")]
    Xtask,
}