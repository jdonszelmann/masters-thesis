

/// Source code represents the input to the system.
/// This might be a single source file, or string, or a whole project. that's tbd
struct SourceCode<Lang>;

/// The result of analyzing the source code. Contains all language-specific information
struct Analysis;

/// Annotated code is code that also contains interactions
/// to navigate or analyze the code
struct AnnotatedCode;

/// The analyzer takes in source code
trait Analyzer<Lang> {
    /// This is the language specific bit
    fn analyze(sourcecode: SourceCode<Lang>) -> Analysis;

    /// if we go for incremental, you'd like an interaction like this
    /// that takes an analysis and cheaply computes a new one based on it
    /// and the updates in the sourcecode
    fn update_analysis(sourcecode: SourceCode<Lang>, old_analysis: Analysis) -> Analysis;
}

/// The annotater knows nothing about the language it's working with
/// However, it uses an analysis to annotate
trait Annotater {
    fn annotate(sourcecode: SourceCode, analysis: Analysis) -> AnnotatedCode;
}
