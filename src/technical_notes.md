# be-crud-rust

create migration with sqlx:
```sqlx migrate add create_table_name```

apply migration
```sqlx migrate run```

apply cache
```cargo sqlx prepare```

# impl From\<A\> for B
* used to change type A to be type B(conversion)
generally this is used to manage error manually, then programmer have full control against that error.
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
* actually almost same with From\<A\> for B, but the main point of difference is MOVE the content from A to B 
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
# syntactic sugar ```?```
just allowed to used in function which return type support early return
early return are supported in: 
* Result\<T, T\>
* Option\<T\>
* and another type which implemented Try, but it in advanced
that's why in middleware can't use ```?``` because it MUST return Response

# return response in handler vs middleware
## handler
* in handler return Result\<T\<U\>, Error>
* because handler can return Error then able to use syntactic sugar ```?```
* axum will handle the convertion, basically axum will handle returning process
## middleware
* in middleware must return Response then must user -> (...).into_response()
* middleware don't have return Error, it MUST return Response then can't use syntactic sugar ```?```
* middleware is low level layer
* middleware worked before extractor and handler then middleware dont know T and dont have pipline like in hanlder

# order parameter in handler MUST
1. STATE
2. PATH
3. QUERY
4. HEADER / EXTENSION
5. JSON / FORM / MULTIPART

# extract Result\<Option\<T\>, SomeError\>
1. use ```match```
```
match result {
    Ok(Some(v)) => {}
    Ok(None) => {}
    Err(e) =>
}
```

2. use ```?```
```
let opt: Option<T> = result?
```

3. use ```ok_or``` / ```ok_or_else```
```
let value: T = result?
    .ok_or(MyError::NotFound)?;
```

4. use ```and_then```
```
let value = result.and_then(|opt| {
    opt.ok_or(MyError::NotFound)
})?;
```

5. use ```unwrap_or``` (NOT SUGGESTED ON PROD)
```let value = result.unwrap_or(None).unwrap_or(default_value);```

6. suggested
```
let data = repo.find_by_id(id)?
    .ok_or(ApiError::NotFound)?;
```

# take value from Option<T>
1. match
```
match opt {
    Some(v) => {
        // v: T
    }
    None => {
        // tidak ada data
    }
}
```

2. unwrap() (!BECAREFUL)
```
let v = opt.unwrap();
```

3. use ```ok_or```
```
let v = opt.ok_or(MyError::NotFound)?;
```
```
let v = opt.ok_or_else(|| MyError::NotFound)?;
```

4. ```if let```
```
if let Some(v) = opt {
    // pakai v
}
```
```
let v = if let Some(v) = opt {
    v
} else {
    return Err(MyError::NotFound);
};
```

5. ```map```
if None then it be None
```
let len = opt.map(|v| v.len());
```

6. extract reference
* if won't move ownership:
```
let opt: Option<String> = Some("hello".into());

if let Some(s) = opt.as_ref() {
    println!("{}", s);
}
// opt masih bisa dipakai
```
* mutable
```
if let Some(s) = opt.as_mut() {
    s.push_str(" world");
}
```

7. often used
```
let user = opt.ok_or(ApiError::NotFound)?;
```

8. just check exist or not
```
if opt.is_none() {
    return Err(ApiError::NotFound);
}
```