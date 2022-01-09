use derive_where::derive_where;

// Rust itself already fails to parse this and will provide a separate error message.
#[derive_where = invalid]
struct InvalidAttribute<T>(PhantomData<T>);

#[derive_where = "invalid"]
struct WrongAttributeSyntax<T>(PhantomData<T>);

#[derive_where()]
struct EmptyAttribute<T>(PhantomData<T>);

#[derive_where(Debug = "option")]
struct WrongOptionSyntax<T>(PhantomData<T>);

#[derive_where(Debug())]
struct EmptyOption<T>(PhantomData<T>);

#[derive_where(Debug(option))]
struct UnsupportedOption<T>(PhantomData<T>);

fn main() {}
