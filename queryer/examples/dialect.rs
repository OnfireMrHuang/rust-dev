
use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
	tracing_subscriber::fmt::init();

	let sql = "SELECT a a1, b, 123, myfunc(b), * \
	from data_source where a > b and b < 100 and c between 10 and 20 \
	order by a desc, b \
	limit 50 offset 10";

	let ast = Parser::parse_sql(&GenericDialect::default(), sql);
	print!("{:#?}", ast);
}