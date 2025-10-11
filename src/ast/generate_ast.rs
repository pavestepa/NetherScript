use swc_common::{errors::{EmitterWriter, Handler}, sync::Lrc, SourceMap, };
use swc_ecma_parser::{error::Error, lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};
use swc_ecma_ast::Module;

pub fn generate_ast(path: &str) -> (Result<Module, Error>, Handler) {
  // Инициализируем SourceMap и загружаем файл
  let cm: Lrc<SourceMap> = Default::default();
  let fm = cm
      .load_file(std::path::Path::new(path))
      .expect("failed to load file");
  
  let emitter = Box::new(EmitterWriter::new(
      Box::new(std::io::stderr()), // выводим в stderr
      Some(cm.clone()),            // используем SourceMap для контекста
      false,                       // не используем терминальные цвета
      false,                       // не нужно показать "diagnostic code"
  ));

  // Создаём лексер и парсер для TypeScript
  let lexer = Lexer::new(
      Syntax::Typescript(TsSyntax {
          tsx: false,       // включить true, если TSX (React)
          decorators: true, // если используешь декораторы
          dts: false,
          no_early_errors: false,
          disallow_ambiguous_jsx_like: false
      }),
      Default::default(),
      StringInput::from(&*fm),
      None,
  );

  let mut parser = Parser::new_from(lexer);
  // создаём Handler (диагностический обработчик ошибок)
  let handler = Handler::with_emitter(
      false,
      true, 
      emitter
      
  );
  return (parser.parse_module(), handler);
}