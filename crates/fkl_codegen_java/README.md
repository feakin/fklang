# FKL Codegen

> Feakin killall language, 

Book API

```java
// createBook with @PutMapping("/book/:id) and BookRequest in BookController, and return Book id
@PutMapping("/book/:id")
public Book createBook(@PathVariable("id") Long id, @RequestBody BookRequest bookRequest) {
    Book book = new Book();
    book.setId(id);
    book.setTitle(bookRequest.getTitle());
    book.setAuthor(bookRequest.getAuthor());
    book.setPrice(bookRequest.getPrice());
    return book;
}
```

BookCreated

```java
// create book in bookRepository and return BookResponse
public BookResponse createBook(BookRequest bookRequest) {
    Book book = new Book(bookRequest);
    bookRepository.save(book);
    return new BookResponse(book);
}
```

BookUpdated

```java
// updateBook with BookUpdateRequest and return BookResponse
public BookResponse updateBook(BookUpdateRequest bookUpdateRequest) {
    Book book = bookRepository.findById(bookUpdateRequest.getId());
    book.update(bookUpdateRequest);
    bookRepository.save(book);
    return new BookResponse(book);
}
```


