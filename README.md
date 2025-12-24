# be-crud-rust

# impl From<A> for B
* used to change type A to be type B(conversion)
generally this is used to manage error manually, then programmer have full control against that error.</br>
* for example sqlx::Error covert to CustomError so programmer have control to manipulate the error message.
```
struct MyError;

struct DbError;
struct IoError;

impl From<DbError> for MyError {
    fn from(_: DbError) -> Self {
        MyError
    }
}

impl From<IoError> for MyError {
    fn from(_: IoError) -> Self {
        MyError
    }
}
```

# impl A for B
* actually almost same with From<A> for B, but the main point of difference is MOVE the content from A to B 
```
struct Wrapper<T> {
    value: T,
}

struct MyStruct<T> {
    inner: T,
}

impl<T> From<Wrapper<T>> for MyStruct<T> {
    fn from(w: Wrapper<T>) -> Self {
        MyStruct { inner: w.value }
    }
}
```